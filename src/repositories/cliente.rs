/* 263A-1: Repositorio de clientes — CRM con búsqueda y paginación.
   Rendimiento crítico: debe manejar ~43k clientes con índices adecuados. */

use sqlx::PgPool;
use uuid::Uuid;

use crate::models::Cliente;

/// Datos para crear un cliente
pub struct NuevoCliente<'a> {
    pub user_id: Uuid,
    pub nombre: &'a str,
    pub apellidos: &'a str,
    pub telefono: &'a str,
    pub prefijo_telefono: &'a str,
    pub email: &'a str,
    pub empresa: &'a str,
    pub notas: &'a str,
    pub foto_url: &'a str,
    pub consentimiento_comercial_email: bool,
    pub consentimiento_comercial_sms: bool,
    pub enviar_encuestas: bool,
    pub alergias: &'a str,
    pub preferencias_bebida: &'a str,
    pub preferencias_ubicacion: &'a str,
}

/// Datos para actualizar un cliente
pub struct ActualizarClienteData<'a> {
    pub id: Uuid,
    pub user_id: Uuid,
    pub nombre: Option<&'a str>,
    pub apellidos: Option<&'a str>,
    pub telefono: Option<&'a str>,
    pub prefijo_telefono: Option<&'a str>,
    pub email: Option<&'a str>,
    pub empresa: Option<&'a str>,
    pub notas: Option<&'a str>,
    pub foto_url: Option<&'a str>,
    pub consentimiento_comercial_email: Option<bool>,
    pub consentimiento_comercial_sms: Option<bool>,
    pub enviar_encuestas: Option<bool>,
    pub alergias: Option<&'a str>,
    pub preferencias_bebida: Option<&'a str>,
    pub preferencias_ubicacion: Option<&'a str>,
}

pub struct ClienteRepository;

impl ClienteRepository {
    pub async fn create(pool: &PgPool, data: &NuevoCliente<'_>) -> Result<Cliente, sqlx::Error> {
        let id = Uuid::new_v4();
        sqlx::query_as!(
            Cliente,
            "INSERT INTO clientes (id, user_id, nombre, apellidos, telefono, prefijo_telefono, \
             email, empresa, notas, foto_url, consentimiento_comercial_email, \
             consentimiento_comercial_sms, enviar_encuestas, alergias, preferencias_bebida, \
             preferencias_ubicacion) \
             VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16) \
             RETURNING *",
            id,
            data.user_id,
            data.nombre,
            data.apellidos,
            data.telefono,
            data.prefijo_telefono,
            data.email,
            data.empresa,
            data.notas,
            data.foto_url,
            data.consentimiento_comercial_email,
            data.consentimiento_comercial_sms,
            data.enviar_encuestas,
            data.alergias,
            data.preferencias_bebida,
            data.preferencias_ubicacion
        )
        .fetch_one(pool)
        .await
    }

    pub async fn find_by_id(
        pool: &PgPool,
        id: Uuid,
        user_id: Uuid,
    ) -> Result<Option<Cliente>, sqlx::Error> {
        sqlx::query_as!(
            Cliente,
            "SELECT * FROM clientes WHERE id = $1 AND user_id = $2",
            id,
            user_id
        )
        .fetch_optional(pool)
        .await
    }

    /// Lista clientes con paginación y búsqueda fulltext por nombre/apellidos/teléfono/email.
    /// Usa ILIKE para búsqueda case-insensitive, compatible con los índices creados.
    pub async fn list(
        pool: &PgPool,
        user_id: Uuid,
        page: i64,
        per_page: i64,
        busqueda: Option<&str>,
    ) -> Result<(Vec<Cliente>, i64), sqlx::Error> {
        let offset = (page - 1) * per_page;
        let patron = busqueda.map(|b| format!("%{b}%"));

        let items = sqlx::query_as!(
            Cliente,
            "SELECT * FROM clientes WHERE user_id = $1 \
             AND ($4::TEXT IS NULL \
                  OR nombre ILIKE $4 \
                  OR apellidos ILIKE $4 \
                  OR telefono ILIKE $4 \
                  OR email ILIKE $4) \
             ORDER BY apellidos ASC, nombre ASC \
             LIMIT $2 OFFSET $3",
            user_id,
            per_page,
            offset,
            patron.as_deref()
        )
        .fetch_all(pool)
        .await?;

        let rec = sqlx::query!(
            "SELECT COUNT(*) as total FROM clientes WHERE user_id = $1 \
             AND ($2::TEXT IS NULL \
                  OR nombre ILIKE $2 \
                  OR apellidos ILIKE $2 \
                  OR telefono ILIKE $2 \
                  OR email ILIKE $2)",
            user_id,
            patron.as_deref()
        )
        .fetch_one(pool)
        .await?;

        Ok((items, rec.total.unwrap_or(0)))
    }

    pub async fn update(
        pool: &PgPool,
        data: &ActualizarClienteData<'_>,
    ) -> Result<Option<Cliente>, sqlx::Error> {
        sqlx::query_as!(
            Cliente,
            "UPDATE clientes SET \
             nombre = COALESCE($3, nombre), \
             apellidos = COALESCE($4, apellidos), \
             telefono = COALESCE($5, telefono), \
             prefijo_telefono = COALESCE($6, prefijo_telefono), \
             email = COALESCE($7, email), \
             empresa = COALESCE($8, empresa), \
             notas = COALESCE($9, notas), \
             foto_url = COALESCE($10, foto_url), \
             consentimiento_comercial_email = COALESCE($11, consentimiento_comercial_email), \
             consentimiento_comercial_sms = COALESCE($12, consentimiento_comercial_sms), \
             enviar_encuestas = COALESCE($13, enviar_encuestas), \
             alergias = COALESCE($14, alergias), \
             preferencias_bebida = COALESCE($15, preferencias_bebida), \
             preferencias_ubicacion = COALESCE($16, preferencias_ubicacion), \
             updated_at = NOW() \
             WHERE id = $1 AND user_id = $2 \
             RETURNING *",
            data.id,
            data.user_id,
            data.nombre,
            data.apellidos,
            data.telefono,
            data.prefijo_telefono,
            data.email,
            data.empresa,
            data.notas,
            data.foto_url,
            data.consentimiento_comercial_email,
            data.consentimiento_comercial_sms,
            data.enviar_encuestas,
            data.alergias,
            data.preferencias_bebida,
            data.preferencias_ubicacion
        )
        .fetch_optional(pool)
        .await
    }

    pub async fn delete(pool: &PgPool, id: Uuid, user_id: Uuid) -> Result<bool, sqlx::Error> {
        let result = sqlx::query!(
            "DELETE FROM clientes WHERE id = $1 AND user_id = $2",
            id,
            user_id
        )
        .execute(pool)
        .await?;
        Ok(result.rows_affected() > 0)
    }
}
