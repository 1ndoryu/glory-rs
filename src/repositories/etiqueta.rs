/* 263A-1: Repositorio de etiquetas y categorías — sistema de tags. */

use sqlx::PgPool;
use uuid::Uuid;

use crate::models::{CategoriaEtiqueta, Etiqueta, EtiquetaConCategoria};

pub struct EtiquetaRepository;

impl EtiquetaRepository {
    /* --- Categorías --- */

    pub async fn list_categorias(
        pool: &PgPool,
        user_id: Uuid,
    ) -> Result<Vec<CategoriaEtiqueta>, sqlx::Error> {
        sqlx::query_as!(
            CategoriaEtiqueta,
            "SELECT * FROM categorias_etiqueta WHERE user_id = $1 OR es_sistema = TRUE \
             ORDER BY nombre ASC",
            user_id,
        )
        .fetch_all(pool)
        .await
    }

    pub async fn create_categoria(
        pool: &PgPool,
        user_id: Uuid,
        nombre: &str,
        aplica_a: &str,
    ) -> Result<CategoriaEtiqueta, sqlx::Error> {
        let id = Uuid::new_v4();
        sqlx::query_as!(
            CategoriaEtiqueta,
            "INSERT INTO categorias_etiqueta (id, user_id, nombre, aplica_a, es_sistema) \
             VALUES ($1, $2, $3, $4, FALSE) RETURNING *",
            id,
            user_id,
            nombre,
            aplica_a,
        )
        .fetch_one(pool)
        .await
    }

    /* --- Etiquetas --- */

    pub async fn list_etiquetas(
        pool: &PgPool,
        user_id: Uuid,
        categoria_id: Option<Uuid>,
    ) -> Result<Vec<EtiquetaConCategoria>, sqlx::Error> {
        sqlx::query_as!(
            EtiquetaConCategoria,
            r#"SELECT e.id, e.user_id, e.nombre, e.color, e.categoria_id,
                      e.es_sistema, e.created_at,
                      c.nombre AS "categoria_nombre!"
               FROM etiquetas e
               JOIN categorias_etiqueta c ON c.id = e.categoria_id
               WHERE (e.user_id = $1 OR e.es_sistema = TRUE)
                 AND ($2::UUID IS NULL OR e.categoria_id = $2)
               ORDER BY c.nombre ASC, e.nombre ASC"#,
            user_id,
            categoria_id,
        )
        .fetch_all(pool)
        .await
    }

    pub async fn create_etiqueta(
        pool: &PgPool,
        user_id: Uuid,
        nombre: &str,
        color: &str,
        categoria_id: Uuid,
    ) -> Result<Etiqueta, sqlx::Error> {
        let id = Uuid::new_v4();
        sqlx::query_as!(
            Etiqueta,
            "INSERT INTO etiquetas (id, user_id, nombre, color, categoria_id, es_sistema) \
             VALUES ($1, $2, $3, $4, $5, FALSE) RETURNING *",
            id,
            user_id,
            nombre,
            color,
            categoria_id,
        )
        .fetch_one(pool)
        .await
    }

    pub async fn delete_etiqueta(
        pool: &PgPool,
        id: Uuid,
        user_id: Uuid,
    ) -> Result<bool, sqlx::Error> {
        let result = sqlx::query!(
            "DELETE FROM etiquetas WHERE id = $1 AND user_id = $2 AND es_sistema = FALSE",
            id,
            user_id,
        )
        .execute(pool)
        .await?;
        Ok(result.rows_affected() > 0)
    }

    /* --- Asignaciones a clientes --- */

    pub async fn assign_to_client(
        pool: &PgPool,
        cliente_id: Uuid,
        etiqueta_id: Uuid,
    ) -> Result<(), sqlx::Error> {
        sqlx::query!(
            "INSERT INTO clientes_etiquetas (cliente_id, etiqueta_id) \
             VALUES ($1, $2) ON CONFLICT DO NOTHING",
            cliente_id,
            etiqueta_id,
        )
        .execute(pool)
        .await?;
        Ok(())
    }

    pub async fn unassign_from_client(
        pool: &PgPool,
        cliente_id: Uuid,
        etiqueta_id: Uuid,
    ) -> Result<bool, sqlx::Error> {
        let result = sqlx::query!(
            "DELETE FROM clientes_etiquetas WHERE cliente_id = $1 AND etiqueta_id = $2",
            cliente_id,
            etiqueta_id,
        )
        .execute(pool)
        .await?;
        Ok(result.rows_affected() > 0)
    }

    pub async fn get_client_tags(
        pool: &PgPool,
        cliente_id: Uuid,
    ) -> Result<Vec<EtiquetaConCategoria>, sqlx::Error> {
        sqlx::query_as!(
            EtiquetaConCategoria,
            r#"SELECT e.id, e.user_id, e.nombre, e.color, e.categoria_id,
                      e.es_sistema, e.created_at,
                      c.nombre AS "categoria_nombre!"
               FROM etiquetas e
               JOIN categorias_etiqueta c ON c.id = e.categoria_id
               JOIN clientes_etiquetas ce ON ce.etiqueta_id = e.id
               WHERE ce.cliente_id = $1
               ORDER BY c.nombre ASC, e.nombre ASC"#,
            cliente_id,
        )
        .fetch_all(pool)
        .await
    }

    /* --- Asignaciones a reservas --- */

    pub async fn assign_to_reservation(
        pool: &PgPool,
        reserva_id: Uuid,
        etiqueta_id: Uuid,
    ) -> Result<(), sqlx::Error> {
        sqlx::query!(
            "INSERT INTO reservas_etiquetas (reserva_id, etiqueta_id) \
             VALUES ($1, $2) ON CONFLICT DO NOTHING",
            reserva_id,
            etiqueta_id,
        )
        .execute(pool)
        .await?;
        Ok(())
    }

    pub async fn unassign_from_reservation(
        pool: &PgPool,
        reserva_id: Uuid,
        etiqueta_id: Uuid,
    ) -> Result<bool, sqlx::Error> {
        let result = sqlx::query!(
            "DELETE FROM reservas_etiquetas WHERE reserva_id = $1 AND etiqueta_id = $2",
            reserva_id,
            etiqueta_id,
        )
        .execute(pool)
        .await?;
        Ok(result.rows_affected() > 0)
    }

    pub async fn get_reservation_tags(
        pool: &PgPool,
        reserva_id: Uuid,
    ) -> Result<Vec<EtiquetaConCategoria>, sqlx::Error> {
        sqlx::query_as!(
            EtiquetaConCategoria,
            r#"SELECT e.id, e.user_id, e.nombre, e.color, e.categoria_id,
                      e.es_sistema, e.created_at,
                      c.nombre AS "categoria_nombre!"
               FROM etiquetas e
               JOIN categorias_etiqueta c ON c.id = e.categoria_id
               JOIN reservas_etiquetas re ON re.etiqueta_id = e.id
               WHERE re.reserva_id = $1
               ORDER BY c.nombre ASC, e.nombre ASC"#,
            reserva_id,
        )
        .fetch_all(pool)
        .await
    }
}
