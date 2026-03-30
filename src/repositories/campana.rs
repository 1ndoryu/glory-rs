/* [263A-23] Repositorio de campañas de marketing.
 * La segmentación se basa en la última reserva completada/confirmada de cada cliente.
 * Consultas con INTERVAL para calcular actividad requieren whitelist (regla 7). */

use sqlx::PgPool;
use uuid::Uuid;

use crate::models::{Campana, CampanaDestinatario, SegmentoPreview};

pub struct NuevaCampana<'a> {
    pub user_id: Uuid,
    pub nombre: &'a str,
    pub descripcion_interna: &'a str,
    pub cuerpo_mensaje: &'a str,
    pub canales: &'a [String],
    pub segmento: &'a str,
    pub incluir_baja: bool,
    pub telefono_baja: &'a str,
}

pub struct ActualizarCampanaData<'a> {
    pub id: Uuid,
    pub user_id: Uuid,
    pub nombre: Option<&'a str>,
    pub descripcion_interna: Option<&'a str>,
    pub cuerpo_mensaje: Option<&'a str>,
    pub canales: Option<&'a [String]>,
    pub segmento: Option<&'a str>,
    pub incluir_baja: Option<bool>,
    pub telefono_baja: Option<&'a str>,
}

pub struct CampanaRepository;

impl CampanaRepository {
    pub async fn create(pool: &PgPool, data: &NuevaCampana<'_>) -> Result<Campana, sqlx::Error> {
        let id = Uuid::new_v4();
        sqlx::query_as!(
            Campana,
            "INSERT INTO campanas (id, user_id, nombre, descripcion_interna, cuerpo_mensaje, \
             canales, segmento, incluir_baja, telefono_baja) \
             VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9) \
             RETURNING *",
            id,
            data.user_id,
            data.nombre,
            data.descripcion_interna,
            data.cuerpo_mensaje,
            data.canales,
            data.segmento,
            data.incluir_baja,
            data.telefono_baja
        )
        .fetch_one(pool)
        .await
    }

    pub async fn find_by_id(
        pool: &PgPool,
        id: Uuid,
        user_id: Uuid,
    ) -> Result<Option<Campana>, sqlx::Error> {
        sqlx::query_as!(
            Campana,
            "SELECT * FROM campanas WHERE id = $1 AND user_id = $2",
            id,
            user_id
        )
        .fetch_optional(pool)
        .await
    }

    pub async fn list(
        pool: &PgPool,
        user_id: Uuid,
        page: i64,
        per_page: i64,
        estado: Option<&str>,
    ) -> Result<(Vec<Campana>, i64), sqlx::Error> {
        let offset = (page - 1) * per_page;
        let estado_filtro = estado.filter(|s| !s.is_empty());

        let items = sqlx::query_as!(
            Campana,
            "SELECT * FROM campanas WHERE user_id = $1 \
             AND ($4::TEXT IS NULL OR estado = $4) \
             ORDER BY created_at DESC \
             LIMIT $2 OFFSET $3",
            user_id,
            per_page,
            offset,
            estado_filtro
        )
        .fetch_all(pool)
        .await?;

        let rec = sqlx::query!(
            "SELECT COUNT(*) as total FROM campanas WHERE user_id = $1 \
             AND ($2::TEXT IS NULL OR estado = $2)",
            user_id,
            estado_filtro
        )
        .fetch_one(pool)
        .await?;

        Ok((items, rec.total.unwrap_or(0)))
    }

    pub async fn update(
        pool: &PgPool,
        data: &ActualizarCampanaData<'_>,
    ) -> Result<Option<Campana>, sqlx::Error> {
        sqlx::query_as!(
            Campana,
            "UPDATE campanas SET \
             nombre = COALESCE($3, nombre), \
             descripcion_interna = COALESCE($4, descripcion_interna), \
             cuerpo_mensaje = COALESCE($5, cuerpo_mensaje), \
             canales = COALESCE($6, canales), \
             segmento = COALESCE($7, segmento), \
             incluir_baja = COALESCE($8, incluir_baja), \
             telefono_baja = COALESCE($9, telefono_baja), \
             updated_at = NOW() \
             WHERE id = $1 AND user_id = $2 AND estado = 'borrador' \
             RETURNING *",
            data.id,
            data.user_id,
            data.nombre,
            data.descripcion_interna,
            data.cuerpo_mensaje,
            data.canales as Option<&[String]>,
            data.segmento,
            data.incluir_baja,
            data.telefono_baja
        )
        .fetch_optional(pool)
        .await
    }

    pub async fn delete(pool: &PgPool, id: Uuid, user_id: Uuid) -> Result<bool, sqlx::Error> {
        let result = sqlx::query!(
            "DELETE FROM campanas WHERE id = $1 AND user_id = $2",
            id,
            user_id
        )
        .execute(pool)
        .await?;
        Ok(result.rows_affected() > 0)
    }

    /* [303A-1] Actualizar estado + contadores reales de envío */
    pub async fn set_estado(
        pool: &PgPool,
        id: Uuid,
        user_id: Uuid,
        estado: &str,
        total_destinatarios: i32,
        total_enviados: i32,
        total_fallidos: i32,
    ) -> Result<Option<Campana>, sqlx::Error> {
        sqlx::query_as!(
            Campana,
            "UPDATE campanas SET estado = $3, total_destinatarios = $4, \
             total_enviados = $5, total_fallidos = $6, updated_at = NOW() \
             WHERE id = $1 AND user_id = $2 \
             RETURNING *",
            id,
            user_id,
            estado,
            total_destinatarios,
            total_enviados,
            total_fallidos
        )
        .fetch_optional(pool)
        .await
    }

    /* [303A-1] Actualizar estado de un destinatario individual después de envío */
    pub async fn actualizar_estado_destinatario(
        pool: &PgPool,
        campana_id: Uuid,
        cliente_id: Uuid,
        canal: &str,
        estado: &str,
    ) -> Result<(), sqlx::Error> {
        sqlx::query!(
            "UPDATE campana_destinatarios SET estado = $4, enviado_at = NOW() \
             WHERE campana_id = $1 AND cliente_id = $2 AND canal = $3",
            campana_id,
            cliente_id,
            canal,
            estado
        )
        .execute(pool)
        .await?;
        Ok(())
    }

    /* [263A-23] Segmentación de clientes por actividad.
     * Usa CTE con última reserva por cliente (estado completada/confirmada).
     * Whitelist de intervalos para cumplir regla 7 (no interpolar INTERVAL). */
    pub async fn segmento_preview(
        pool: &PgPool,
        user_id: Uuid,
        segmento: &str,
    ) -> Result<SegmentoPreview, sqlx::Error> {
        /* Mapear segmento a rango de días.
         * Se pasan como parámetros enteros para construir la condición SQL.
         * - habitual: 0-30 días
         * - sin_1m: 30-90 días
         * - sin_3m: 90-180 días
         * - sin_6m: 180-270 días
         * - sin_9m: 270-365 días
         * - sin_1a: 365-730 días
         * - sin_mas_1a: >730 días
         * - todos: sin filtro de fecha */
        let (dias_min, dias_max): (Option<i32>, Option<i32>) = match segmento {
            "habitual" => (Some(0), Some(30)),
            "sin_1m" => (Some(30), Some(90)),
            "sin_3m" => (Some(90), Some(180)),
            "sin_6m" => (Some(180), Some(270)),
            "sin_9m" => (Some(270), Some(365)),
            "sin_1a" => (Some(365), Some(730)),
            "sin_mas_1a" => (Some(730), None),
            _ => (None, None), /* todos */
        };

        let rec = sqlx::query!(
            r#"WITH ultima_reserva AS (
                SELECT r.cliente_id, MAX(r.fecha) as ultima_fecha
                FROM reservas r
                WHERE r.user_id = $1
                  AND r.estado IN ('completada', 'confirmada')
                  AND r.cliente_id IS NOT NULL
                GROUP BY r.cliente_id
            )
            SELECT
                COUNT(*) as "total_clientes!",
                COUNT(*) FILTER (WHERE c.email <> '') as "con_email!",
                COUNT(*) FILTER (WHERE c.telefono <> '') as "con_telefono!",
                COUNT(*) FILTER (WHERE c.consentimiento_comercial_email = true) as "con_consentimiento_email!",
                COUNT(*) FILTER (WHERE c.consentimiento_comercial_sms = true) as "con_consentimiento_sms!"
            FROM clientes c
            LEFT JOIN ultima_reserva ur ON ur.cliente_id = c.id
            WHERE c.user_id = $1
              AND (
                  /* todos: sin filtro de actividad */
                  ($2::INT IS NULL AND $3::INT IS NULL)
                  /* segmentos con rango: días desde última reserva entre min y max */
                  OR (
                      $2 IS NOT NULL AND $3 IS NOT NULL
                      AND ur.ultima_fecha IS NOT NULL
                      AND (CURRENT_DATE - ur.ultima_fecha::date) >= $2
                      AND (CURRENT_DATE - ur.ultima_fecha::date) < $3
                  )
                  /* sin_mas_1a: solo mínimo, sin máximo */
                  OR (
                      $2 IS NOT NULL AND $3 IS NULL
                      AND (
                          ur.ultima_fecha IS NULL
                          OR (CURRENT_DATE - ur.ultima_fecha::date) >= $2
                      )
                  )
              )"#,
            user_id,
            dias_min,
            dias_max
        )
        .fetch_one(pool)
        .await?;

        Ok(SegmentoPreview {
            segmento: segmento.to_string(),
            total_clientes: rec.total_clientes,
            con_email: rec.con_email,
            con_telefono: rec.con_telefono,
            con_consentimiento_email: rec.con_consentimiento_email,
            con_consentimiento_sms: rec.con_consentimiento_sms,
        })
    }

    /* Obtener IDs de clientes que caen en un segmento y tienen consentimiento
     * para al menos uno de los canales solicitados */
    pub async fn clientes_segmento(
        pool: &PgPool,
        user_id: Uuid,
        segmento: &str,
        canales: &[String],
    ) -> Result<Vec<ClienteSegmentado>, sqlx::Error> {
        let (dias_min, dias_max): (Option<i32>, Option<i32>) = match segmento {
            "habitual" => (Some(0), Some(30)),
            "sin_1m" => (Some(30), Some(90)),
            "sin_3m" => (Some(90), Some(180)),
            "sin_6m" => (Some(180), Some(270)),
            "sin_9m" => (Some(270), Some(365)),
            "sin_1a" => (Some(365), Some(730)),
            "sin_mas_1a" => (Some(730), None),
            _ => (None, None),
        };

        let tiene_sms = canales.iter().any(|c| c == "sms");
        let tiene_email = canales.iter().any(|c| c == "email");
        let tiene_whatsapp = canales.iter().any(|c| c == "whatsapp");

        sqlx::query_as!(
            ClienteSegmentado,
            r#"WITH ultima_reserva AS (
                SELECT r.cliente_id, MAX(r.fecha) as ultima_fecha
                FROM reservas r
                WHERE r.user_id = $1
                  AND r.estado IN ('completada', 'confirmada')
                  AND r.cliente_id IS NOT NULL
                GROUP BY r.cliente_id
            )
            SELECT
                c.id,
                c.email,
                c.telefono,
                c.prefijo_telefono,
                c.consentimiento_comercial_email,
                c.consentimiento_comercial_sms
            FROM clientes c
            LEFT JOIN ultima_reserva ur ON ur.cliente_id = c.id
            WHERE c.user_id = $1
              AND (
                  ($2::INT IS NULL AND $3::INT IS NULL)
                  OR (
                      $2 IS NOT NULL AND $3 IS NOT NULL
                      AND ur.ultima_fecha IS NOT NULL
                      AND (CURRENT_DATE - ur.ultima_fecha::date) >= $2
                      AND (CURRENT_DATE - ur.ultima_fecha::date) < $3
                  )
                  OR (
                      $2 IS NOT NULL AND $3 IS NULL
                      AND (
                          ur.ultima_fecha IS NULL
                          OR (CURRENT_DATE - ur.ultima_fecha::date) >= $2
                      )
                  )
              )
              AND (
                  ($4 = true AND c.consentimiento_comercial_sms = true AND c.telefono <> '')
                  OR ($5 = true AND c.consentimiento_comercial_email = true AND c.email <> '')
                  OR ($6 = true AND c.consentimiento_comercial_sms = true AND c.telefono <> '')
              )"#,
            user_id,
            dias_min,
            dias_max,
            tiene_sms,
            tiene_email,
            tiene_whatsapp
        )
        .fetch_all(pool)
        .await
    }

    /* Insertar destinatarios en bloque después de segmentar */
    pub async fn insertar_destinatarios(
        pool: &PgPool,
        campana_id: Uuid,
        destinatarios: &[NuevoDestinatario],
    ) -> Result<u64, sqlx::Error> {
        let mut total: u64 = 0;
        for dest in destinatarios {
            sqlx::query!(
                "INSERT INTO campana_destinatarios (id, campana_id, cliente_id, canal) \
                 VALUES ($1, $2, $3, $4)",
                Uuid::new_v4(),
                campana_id,
                dest.cliente_id,
                dest.canal
            )
            .execute(pool)
            .await?;
            total += 1;
        }
        Ok(total)
    }

    pub async fn listar_destinatarios(
        pool: &PgPool,
        campana_id: Uuid,
    ) -> Result<Vec<CampanaDestinatario>, sqlx::Error> {
        sqlx::query_as!(
            CampanaDestinatario,
            "SELECT * FROM campana_destinatarios WHERE campana_id = $1 ORDER BY created_at",
            campana_id
        )
        .fetch_all(pool)
        .await
    }
}

/* Estructura auxiliar para clientes segmentados */
#[derive(Debug, sqlx::FromRow)]
pub struct ClienteSegmentado {
    pub id: Uuid,
    pub email: String,
    pub telefono: String,
    pub prefijo_telefono: String,
    pub consentimiento_comercial_email: bool,
    pub consentimiento_comercial_sms: bool,
}

/* Estructura auxiliar para insertar destinatarios */
pub struct NuevoDestinatario {
    pub cliente_id: Uuid,
    pub canal: String,
}
