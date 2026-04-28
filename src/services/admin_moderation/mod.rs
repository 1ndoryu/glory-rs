/* [274A-29+30+32+33+34] Servicio admin de moderacion.
 * Mantiene validaciones legacy, notificaciones a autores y bans manuales fuera
 * del handler para que cada endpoint solo traduzca HTTP a dominio. */

use chrono::{Duration, Utc};
use serde_json::json;
use sqlx::PgPool;

use crate::errors::AppError;
use crate::repositories::AdminModerationRepository;
use crate::services::{CreateNotificationInput, NotificationService};

const REJECTION_REASON: &str = "revision_manual";

pub struct AdminModerationService;

pub struct ModerateContentInput {
    pub tipo: String,
    pub id: i32,
    pub accion: String,
}

pub struct ResolveReportInput {
    pub admin_id: i32,
    pub id: i32,
    pub accion: String,
}

pub struct ManualBanInput {
    pub admin_id: i32,
    pub usuario_id: i32,
    pub duracion: String,
    pub razon: String,
}

impl AdminModerationService {
    pub async fn moderate_content(
        pool: &PgPool,
        input: ModerateContentInput,
    ) -> Result<(), AppError> {
        if input.id <= 0 {
            return Err(AppError::BadRequest("id requerido".into()));
        }
        let estado = moderation_state_from_action(&input.accion)?;
        let reason = (estado == "rechazado").then_some(REJECTION_REASON);

        let author_id = match input.tipo.as_str() {
            "publicacion" => {
                AdminModerationRepository::set_post_moderation(pool, input.id, estado, reason)
                    .await?
            }
            "comentario" => {
                AdminModerationRepository::set_comment_moderation(pool, input.id, estado).await?
            }
            "articulo" => {
                AdminModerationRepository::set_article_moderation(pool, input.id, estado, reason)
                    .await?
            }
            _ => return Err(AppError::BadRequest("tipo de contenido invalido".into())),
        }
        .ok_or_else(|| AppError::NotFound("Contenido no encontrado".into()))?;

        if estado == "rechazado" && input.tipo != "comentario" {
            notify_content_rejected(pool, author_id, &input.tipo).await?;
        }
        Ok(())
    }

    pub async fn resolve_report(pool: &PgPool, input: ResolveReportInput) -> Result<(), AppError> {
        if input.id <= 0 {
            return Err(AppError::BadRequest("id de reporte requerido".into()));
        }
        let estado = report_state_from_action(&input.accion)?;
        let updated =
            AdminModerationRepository::resolve_report(pool, input.id, estado, input.admin_id)
                .await?;
        if !updated {
            return Err(AppError::NotFound(
                "Reporte no encontrado o ya resuelto".into(),
            ));
        }
        Ok(())
    }

    pub async fn reject_pending_posts(pool: &PgPool) -> Result<i64, AppError> {
        AdminModerationRepository::reject_pending_posts(pool).await
    }

    pub async fn reject_user_posts(pool: &PgPool, author_id: i32) -> Result<i64, AppError> {
        if author_id <= 0 {
            return Err(AppError::BadRequest("autor_id requerido".into()));
        }
        let affected = AdminModerationRepository::reject_user_posts(pool, author_id).await?;
        if affected > 0 {
            let message = format!(
                "Se han rechazado {affected} de tus publicaciones tras una revision del equipo de moderacion."
            );
            NotificationService::create(
                pool,
                CreateNotificationInput {
                    destinatario_id: author_id,
                    tipo: "moderacion".to_string(),
                    titulo: "Publicaciones rechazadas".to_string(),
                    mensaje: message,
                    datos: json!({ "razon": "rechazo_masivo", "afectados": affected }),
                    actor_id: None,
                    enlace: None,
                },
            )
            .await?;
        }
        Ok(affected)
    }

    pub async fn apply_manual_ban(pool: &PgPool, input: ManualBanInput) -> Result<(), AppError> {
        if input.usuario_id <= 0 {
            return Err(AppError::BadRequest("usuario_id requerido".into()));
        }
        if input.usuario_id == input.admin_id {
            return Err(AppError::BadRequest(
                "No puedes banear tu propia cuenta".into(),
            ));
        }
        let clean_reason = trim_required(&input.razon, "razon requerida")?;
        let hours = ban_duration_hours(&input.duracion);
        let ban_until = Utc::now() + Duration::hours(i64::from(hours));
        let ban_reason = format!("Ban manual por equipo de moderacion. Razon: {clean_reason}");
        let updated = AdminModerationRepository::apply_manual_ban(
            pool,
            input.usuario_id,
            ban_until,
            &ban_reason,
        )
        .await?;
        if !updated {
            return Err(AppError::NotFound("Usuario no encontrado".into()));
        }

        let label = if hours >= 24 {
            format!("{} dia(s)", hours / 24)
        } else {
            format!("{hours} hora(s)")
        };
        NotificationService::create(
            pool,
            CreateNotificationInput {
                destinatario_id: input.usuario_id,
                tipo: "moderacion".to_string(),
                titulo: "Cuenta suspendida".to_string(),
                mensaje: format!(
                    "Tu cuenta fue suspendida por {label} por el equipo de moderacion. Razon: {clean_reason}."
                ),
                datos: json!({ "razon": clean_reason, "horas": hours }),
                actor_id: Some(input.admin_id),
                enlace: None,
            },
        )
        .await?;
        Ok(())
    }
}

fn moderation_state_from_action(action: &str) -> Result<&'static str, AppError> {
    match action {
        "aprobar" => Ok("aprobado"),
        "rechazar" => Ok("rechazado"),
        _ => Err(AppError::BadRequest("accion de moderacion invalida".into())),
    }
}

fn report_state_from_action(action: &str) -> Result<&'static str, AppError> {
    match action {
        "resolver" => Ok("resuelto"),
        "descartar" => Ok("descartado"),
        _ => Err(AppError::BadRequest("accion de reporte invalida".into())),
    }
}

fn ban_duration_hours(duration: &str) -> i32 {
    match duration {
        "1h" => 1,
        "7d" => 168,
        "30d" => 720,
        _ => 24,
    }
}

fn trim_required(value: &str, message: &str) -> Result<String, AppError> {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        Err(AppError::BadRequest(message.into()))
    } else {
        Ok(trimmed.chars().take(500).collect())
    }
}

async fn notify_content_rejected(
    pool: &PgPool,
    author_id: i32,
    content_type: &str,
) -> Result<(), AppError> {
    let (title, message) = match content_type {
        "articulo" => (
            "Articulo rechazado",
            "Tu articulo fue revisado y rechazado por el equipo de moderacion.",
        ),
        _ => (
            "Publicacion rechazada",
            "Tu publicacion fue revisada y rechazada por el equipo de moderacion.",
        ),
    };
    NotificationService::create(
        pool,
        CreateNotificationInput {
            destinatario_id: author_id,
            tipo: "moderacion".to_string(),
            titulo: title.to_string(),
            mensaje: message.to_string(),
            datos: json!({ "razon": REJECTION_REASON }),
            actor_id: None,
            enlace: None,
        },
    )
    .await?;
    Ok(())
}
