/* [274A-49+50+51+52] Servicio admin para experimentos, embeddings y benchmark.
 * Porta endpoints legacy con trabajo real: recalcula pgvector 128d, crea usuario/
 * notificacion/mensaje de prueba y mide consultas reales del algoritmo Rust. */

use std::collections::BTreeMap;
use std::time::Instant;

use rand::seq::SliceRandom;
use serde_json::{json, Value};
use sqlx::PgPool;

use crate::audio::embeddings::{AudioEmbedding, EmbeddingInput};
use crate::errors::AppError;
use crate::repositories::{
    AdminExperimentsRepository, ConversationRepository, CreateMessageParams, DirectMessageKind,
    MessageRepository,
};
use crate::services::{CreateNotificationInput, NotificationService};

const EMBEDDING_BATCH_SIZE: i64 = 200;
const TEST_USERNAME: &str = "alice_test";
const TEST_EMAIL: &str = "alice_test@glory.local";
const TEST_DISPLAY: &str = "Alice Test";
const TEST_BIO: &str = "Productora de test para validar interacciones sociales.";
const TEST_AVATAR: &str =
    "https://ui-avatars.com/api/?name=Alice%20Test&background=6366f1&color=fff&size=128&bold=true";

pub struct AdminExperimentsService;

pub struct EmbeddingBatchResult {
    pub actualizados: i64,
    pub tiempo_ms: i64,
    pub mensaje: String,
}

pub struct BenchmarkResult {
    pub output: String,
    pub stderr: String,
    pub exit_code: i32,
}

impl AdminExperimentsService {
    pub async fn generate_embeddings(
        pool: &PgPool,
        regenerate: bool,
    ) -> Result<EmbeddingBatchResult, AppError> {
        let start = Instant::now();
        if regenerate {
            AdminExperimentsRepository::clear_embeddings(pool).await?;
        }

        let mut updated = 0_i64;
        loop {
            let rows = AdminExperimentsRepository::list_embedding_candidates(
                pool,
                EMBEDDING_BATCH_SIZE,
                true,
            )
            .await?;
            if rows.is_empty() {
                break;
            }

            for row in rows {
                let embedding = AudioEmbedding::generate(&EmbeddingInput {
                    bpm: row
                        .bpm
                        .and_then(|value| u16::try_from(value.clamp(0, i32::from(u16::MAX))).ok()),
                    music_key: row.music_key,
                    scale: row.scale,
                    sample_type: Some(normalize_sample_type(&row.sample_type)),
                    duration_seconds: Some(row.duration_seconds),
                    is_premium: row.is_premium,
                    tags: row.tags,
                });
                AdminExperimentsRepository::update_sample_embedding(
                    pool,
                    row.id,
                    embedding.to_pgvector(),
                )
                .await?;
                updated += 1;
            }
        }

        let tiempo_ms = elapsed_ms(start);
        let action = if regenerate {
            "regeneraron"
        } else {
            "generaron"
        };
        Ok(EmbeddingBatchResult {
            actualizados: updated,
            tiempo_ms,
            mensaje: format!("Se {action} {updated} embeddings en {tiempo_ms}ms"),
        })
    }

    pub async fn generate_experiment(
        pool: &PgPool,
        admin_id: i32,
        acciones: Option<Vec<String>>,
    ) -> Result<BTreeMap<String, Value>, AppError> {
        let actions = normalize_actions(acciones);
        let mut result = BTreeMap::new();

        let needs_user = actions
            .iter()
            .any(|action| matches!(action.as_str(), "usuario" | "notificacion" | "mensaje"));
        if !needs_user {
            return Ok(result);
        }

        let test_user = AdminExperimentsRepository::upsert_test_user(
            pool,
            TEST_USERNAME,
            TEST_EMAIL,
            TEST_DISPLAY,
            TEST_BIO,
            TEST_AVATAR,
        )
        .await?;

        if actions.iter().any(|action| action == "usuario") {
            result.insert(
                "usuario".to_string(),
                json!({
                    "pgId": test_user.id,
                    "username": test_user.username,
                    "nombre": test_user.nombre_visible,
                    "mensaje": "Usuario de test listo",
                }),
            );
        }

        if actions.iter().any(|action| action == "notificacion") {
            let notification = Self::generate_notification(pool, admin_id, test_user.id).await?;
            result.insert("notificacion".to_string(), notification);
        }

        if actions.iter().any(|action| action == "mensaje") {
            let message = Self::generate_message(pool, admin_id, test_user.id).await?;
            result.insert("mensaje".to_string(), message);
        }

        Ok(result)
    }

    pub async fn run_benchmark(
        pool: &PgPool,
        user_id: i32,
        per_page: i64,
    ) -> Result<BenchmarkResult, AppError> {
        let per_page = per_page.clamp(1, 100);
        let total_start = Instant::now();

        let active_start = Instant::now();
        let active_samples = AdminExperimentsRepository::active_sample_count(pool).await?;
        let active_ms = elapsed_ms(active_start);

        let embedding_start = Instant::now();
        let embedding_samples = AdminExperimentsRepository::embedding_count(pool).await?;
        let embedding_ms = elapsed_ms(embedding_start);

        let feed_start = Instant::now();
        let feed_rows = AdminExperimentsRepository::benchmark_feed(pool, per_page).await?;
        let feed_ms = elapsed_ms(feed_start);

        let similar_start = Instant::now();
        let similar_rows = AdminExperimentsRepository::benchmark_similar(pool, 12).await?;
        let similar_ms = elapsed_ms(similar_start);

        let total_ms = elapsed_ms(total_start);
        let output = format!(
            "BENCHMARK ALGORITMO - KAMPLES (Rust)\n\
             usuario={user_id} perPage={per_page}\n\
             [1/4] samples activos: {active_samples} ({active_ms}ms)\n\
             [2/4] samples con embedding: {embedding_samples} ({embedding_ms}ms)\n\
             [3/4] feed publico: {feed_rows} filas ({feed_ms}ms)\n\
             [4/4] similares pgvector: {similar_rows} filas ({similar_ms}ms)\n\
             TOTAL: {total_ms}ms\n"
        );

        Ok(BenchmarkResult {
            output,
            stderr: String::new(),
            exit_code: 0,
        })
    }

    async fn generate_notification(
        pool: &PgPool,
        admin_id: i32,
        test_user_id: i32,
    ) -> Result<Value, AppError> {
        let tipos = ["follow", "like", "mensaje"];
        let tipo = tipos
            .choose(&mut rand::thread_rng())
            .copied()
            .unwrap_or("follow");
        let datos = match tipo {
            "like" => {
                let sample_id =
                    AdminExperimentsRepository::latest_sample_id_for_creator(pool, admin_id)
                        .await?
                        .unwrap_or(1);
                json!({ "liker_id": test_user_id, "sample_id": sample_id })
            }
            "mensaje" => {
                json!({ "remitente_id": test_user_id, "preview": "Hey! Me encantaron tus beats." })
            }
            _ => json!({ "seguidor_id": test_user_id }),
        };

        NotificationService::create(
            pool,
            CreateNotificationInput {
                destinatario_id: admin_id,
                tipo: tipo.to_string(),
                titulo: format!("Experimento {tipo}"),
                mensaje: format!("Notificacion tipo '{tipo}' creada"),
                datos,
                actor_id: Some(test_user_id),
                enlace: None,
            },
        )
        .await?;

        Ok(json!({
            "tipo": tipo,
            "destino": admin_id,
            "actor": test_user_id,
            "mensaje": format!("Notificacion tipo '{tipo}' creada"),
        }))
    }

    async fn generate_message(
        pool: &PgPool,
        admin_id: i32,
        test_user_id: i32,
    ) -> Result<Value, AppError> {
        let mensajes = [
            "Hey! Escuche tus ultimos samples y estan increibles. Me gustaria colaborar contigo en un proyecto.",
            "Ese loop de guitarra que subiste es de otro nivel. Tienes mas en ese estilo?",
            "Hola! Yo trabajo principalmente con trap y R&B, estemos en contacto!",
            "Acabo de descargar tu pack y lo estoy usando en un beat. Te mando el resultado cuando termine.",
        ];
        let contenido = mensajes
            .choose(&mut rand::thread_rng())
            .copied()
            .unwrap_or(mensajes[0]);

        let conversation_id =
            match ConversationRepository::find_between_users(pool, admin_id, test_user_id).await? {
                Some(id) => id,
                None => ConversationRepository::create(pool, admin_id, test_user_id).await?,
            };
        MessageRepository::create(
            pool,
            CreateMessageParams {
                conversacion_id: conversation_id,
                autor_id: test_user_id,
                contenido,
                tipo: DirectMessageKind::Texto,
                media_url: None,
                media_metadata: None,
            },
        )
        .await?;

        Ok(json!({
            "conversacionId": conversation_id,
            "contenido": contenido,
            "remitente": TEST_USERNAME,
            "mensaje": format!("Mensaje enviado desde {TEST_DISPLAY}"),
        }))
    }
}

fn normalize_actions(acciones: Option<Vec<String>>) -> Vec<String> {
    let mut actions =
        acciones.unwrap_or_else(|| vec!["usuario".into(), "notificacion".into(), "mensaje".into()]);
    actions.retain(|action| matches!(action.as_str(), "usuario" | "notificacion" | "mensaje"));
    actions.sort();
    actions.dedup();
    actions
}

fn normalize_sample_type(value: &str) -> String {
    match value.trim().to_ascii_lowercase().as_str() {
        "oneshot" | "one-shot" | "one_shot" => "one_shot".to_string(),
        "fx" => "fx".to_string(),
        "vocal" => "vocal".to_string(),
        "stem" => "stem".to_string(),
        "otro" | "other" => "other".to_string(),
        _ => "loop".to_string(),
    }
}

fn elapsed_ms(start: Instant) -> i64 {
    i64::try_from(start.elapsed().as_millis()).unwrap_or(i64::MAX)
}
