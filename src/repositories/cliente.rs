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

    /* [014A-2] Buscar cliente existente por teléfono o email para evitar duplicados.
     * Prioriza teléfono (más fiable), luego email. Solo busca campos no vacíos.
     * [014A-11] Convertido a query_as! para verificación SQL en compilación. */
    pub async fn find_by_telefono_o_email(
        pool: &PgPool,
        user_id: Uuid,
        telefono: &str,
        email: &str,
    ) -> Result<Option<Cliente>, sqlx::Error> {
        sqlx::query_as!(
            Cliente,
            "SELECT * FROM clientes WHERE user_id = $1 \
             AND (($2 != '' AND telefono = $2) OR ($3 != '' AND email = $3)) \
             LIMIT 1",
            user_id,
            telefono,
            email
        )
        .fetch_optional(pool)
        .await
    }

    /// [303A-6] Lista clientes con paginación y búsqueda fulltext.
    /// Busca en nombre, apellidos, teléfono, email, empresa y notas.
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
                  OR email ILIKE $4 \
                  OR empresa ILIKE $4 \
                  OR notas ILIKE $4) \
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
                  OR email ILIKE $2 \
                  OR empresa ILIKE $2 \
                  OR notas ILIKE $2)",
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

    /* [263A-26] Merge atómico de dos clientes en una sola transacción.
     * 1. Migra reservas, etiquetas y campañas del origen → destino.
     * 2. Rellena campos vacíos del destino con los del origen.
     * 3. Elimina el cliente origen.
     * Retorna (cliente_destino_actualizado, reservas_migradas, etiquetas_migradas, campanas_migradas). */
    pub async fn merge(
        pool: &PgPool,
        origen_id: Uuid,
        destino_id: Uuid,
        user_id: Uuid,
    ) -> Result<(Cliente, i64, i64, i64), sqlx::Error> {
        let mut tx = pool.begin().await?;

        /* 1. Migrar reservas: origen → destino */
        let reservas = sqlx::query!(
            "UPDATE reservas SET cliente_id = $1 WHERE cliente_id = $2 AND user_id = $3",
            destino_id,
            origen_id,
            user_id
        )
        .execute(&mut *tx)
        .await?;

        /* 2. Migrar etiquetas: ON CONFLICT ignora duplicados (misma etiqueta ya asignada al destino) */
        let etiquetas = sqlx::query!(
            "UPDATE clientes_etiquetas SET cliente_id = $1 \
             WHERE cliente_id = $2 \
             AND etiqueta_id NOT IN (SELECT etiqueta_id FROM clientes_etiquetas WHERE cliente_id = $1)",
            destino_id,
            origen_id
        )
        .execute(&mut *tx)
        .await?;

        /* Limpiar etiquetas duplicadas que NO se migraron */
        sqlx::query!(
            "DELETE FROM clientes_etiquetas WHERE cliente_id = $1",
            origen_id
        )
        .execute(&mut *tx)
        .await?;

        /* 3. Migrar destinatarios de campañas: evitar duplicados (misma campaña+destino) */
        let campanas = sqlx::query!(
            "UPDATE campana_destinatarios SET cliente_id = $1 \
             WHERE cliente_id = $2 \
             AND campana_id NOT IN (SELECT campana_id FROM campana_destinatarios WHERE cliente_id = $1)",
            destino_id,
            origen_id
        )
        .execute(&mut *tx)
        .await?;

        /* Limpiar destinatarios duplicados que NO se migraron */
        sqlx::query!(
            "DELETE FROM campana_destinatarios WHERE cliente_id = $1",
            origen_id
        )
        .execute(&mut *tx)
        .await?;

        /* 4. Combinar campos: si el destino tiene campo vacío, rellenar con el del origen.
         * Para booleanos, usar OR (si alguno tiene consentimiento, mantenerlo). */
        let cliente = sqlx::query_as!(
            Cliente,
            "WITH origen AS (SELECT * FROM clientes WHERE id = $2 AND user_id = $3) \
             UPDATE clientes SET \
               apellidos = CASE WHEN clientes.apellidos = '' THEN (SELECT apellidos FROM origen) ELSE clientes.apellidos END, \
               telefono = CASE WHEN clientes.telefono = '' THEN (SELECT telefono FROM origen) ELSE clientes.telefono END, \
               prefijo_telefono = CASE WHEN clientes.prefijo_telefono = '+34' AND (SELECT prefijo_telefono FROM origen) <> '+34' \
                 THEN (SELECT prefijo_telefono FROM origen) ELSE clientes.prefijo_telefono END, \
               email = CASE WHEN clientes.email = '' THEN (SELECT email FROM origen) ELSE clientes.email END, \
               empresa = CASE WHEN clientes.empresa = '' THEN (SELECT empresa FROM origen) ELSE clientes.empresa END, \
               notas = CASE WHEN clientes.notas = '' THEN (SELECT notas FROM origen) \
                 WHEN (SELECT notas FROM origen) <> '' THEN clientes.notas || E'\\n---\\n' || (SELECT notas FROM origen) \
                 ELSE clientes.notas END, \
               foto_url = CASE WHEN clientes.foto_url = '' THEN (SELECT foto_url FROM origen) ELSE clientes.foto_url END, \
               consentimiento_comercial_email = clientes.consentimiento_comercial_email OR (SELECT consentimiento_comercial_email FROM origen), \
               consentimiento_comercial_sms = clientes.consentimiento_comercial_sms OR (SELECT consentimiento_comercial_sms FROM origen), \
               enviar_encuestas = clientes.enviar_encuestas OR (SELECT enviar_encuestas FROM origen), \
               alergias = CASE WHEN clientes.alergias = '' THEN (SELECT alergias FROM origen) \
                 WHEN (SELECT alergias FROM origen) <> '' THEN clientes.alergias || ', ' || (SELECT alergias FROM origen) \
                 ELSE clientes.alergias END, \
               preferencias_bebida = CASE WHEN clientes.preferencias_bebida = '' THEN (SELECT preferencias_bebida FROM origen) ELSE clientes.preferencias_bebida END, \
               preferencias_ubicacion = CASE WHEN clientes.preferencias_ubicacion = '' THEN (SELECT preferencias_ubicacion FROM origen) ELSE clientes.preferencias_ubicacion END, \
               updated_at = NOW() \
             WHERE clientes.id = $1 AND clientes.user_id = $3 \
             RETURNING clientes.*",
            destino_id,
            origen_id,
            user_id
        )
        .fetch_one(&mut *tx)
        .await?;

        /* 5. Eliminar cliente origen */
        sqlx::query!(
            "DELETE FROM clientes WHERE id = $1 AND user_id = $2",
            origen_id,
            user_id
        )
        .execute(&mut *tx)
        .await?;

        tx.commit().await?;

        Ok((
            cliente,
            reservas.rows_affected().cast_signed(),
            etiquetas.rows_affected().cast_signed(),
            campanas.rows_affected().cast_signed(),
        ))
    }
}
