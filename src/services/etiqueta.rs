/* 263A-1: Servicio de etiquetas y categorías — gestión de tags. */

use sqlx::PgPool;
use uuid::Uuid;

use crate::errors::AppError;
use crate::models::{
    CategoriaEtiqueta, CrearEtiquetaRequest, EtiquetaConCategoria, EtiquetasQuery,
};
use crate::repositories::EtiquetaRepository;

pub struct EtiquetaService;

impl EtiquetaService {
    /* --- Categorías --- */

    pub async fn list_categorias(
        pool: &PgPool,
        user_id: Uuid,
    ) -> Result<Vec<CategoriaEtiqueta>, AppError> {
        let cats = EtiquetaRepository::list_categorias(pool, user_id).await?;
        Ok(cats)
    }

    pub async fn create_categoria(
        pool: &PgPool,
        user_id: Uuid,
        nombre: &str,
        aplica_a: &str,
    ) -> Result<CategoriaEtiqueta, AppError> {
        let cat = EtiquetaRepository::create_categoria(pool, user_id, nombre, aplica_a).await?;
        Ok(cat)
    }

    /* --- Etiquetas --- */

    pub async fn list_etiquetas(
        pool: &PgPool,
        user_id: Uuid,
        query: EtiquetasQuery,
    ) -> Result<Vec<EtiquetaConCategoria>, AppError> {
        let etiquetas =
            EtiquetaRepository::list_etiquetas(pool, user_id, query.categoria_id).await?;
        Ok(etiquetas)
    }

    pub async fn create_etiqueta(
        pool: &PgPool,
        user_id: Uuid,
        req: CrearEtiquetaRequest,
    ) -> Result<EtiquetaConCategoria, AppError> {
        let etiqueta = EtiquetaRepository::create_etiqueta(
            pool,
            user_id,
            &req.nombre,
            req.color.as_deref().unwrap_or("#6b7280"),
            req.categoria_id,
        )
        .await?;

        /* Retornar con datos de categoría para consistencia con la lista */
        let tags =
            EtiquetaRepository::list_etiquetas(pool, user_id, Some(etiqueta.categoria_id)).await?;
        tags.into_iter()
            .find(|t| t.id == etiqueta.id)
            .ok_or_else(|| AppError::Internal("Etiqueta creada pero no encontrada".into()))
    }

    pub async fn delete_etiqueta(pool: &PgPool, id: Uuid, user_id: Uuid) -> Result<(), AppError> {
        if !EtiquetaRepository::delete_etiqueta(pool, id, user_id).await? {
            return Err(AppError::NotFound(
                "Etiqueta no encontrada o es de sistema".into(),
            ));
        }
        Ok(())
    }

    /* --- Asignaciones a clientes --- */

    pub async fn assign_to_client(
        pool: &PgPool,
        cliente_id: Uuid,
        etiqueta_id: Uuid,
    ) -> Result<(), AppError> {
        EtiquetaRepository::assign_to_client(pool, cliente_id, etiqueta_id).await?;
        Ok(())
    }

    pub async fn unassign_from_client(
        pool: &PgPool,
        cliente_id: Uuid,
        etiqueta_id: Uuid,
    ) -> Result<(), AppError> {
        if !EtiquetaRepository::unassign_from_client(pool, cliente_id, etiqueta_id).await? {
            return Err(AppError::NotFound("Asignación no encontrada".into()));
        }
        Ok(())
    }

    pub async fn get_client_tags(
        pool: &PgPool,
        cliente_id: Uuid,
    ) -> Result<Vec<EtiquetaConCategoria>, AppError> {
        let tags = EtiquetaRepository::get_client_tags(pool, cliente_id).await?;
        Ok(tags)
    }

    /* --- Asignaciones a reservas --- */

    pub async fn assign_to_reservation(
        pool: &PgPool,
        reserva_id: Uuid,
        etiqueta_id: Uuid,
    ) -> Result<(), AppError> {
        EtiquetaRepository::assign_to_reservation(pool, reserva_id, etiqueta_id).await?;
        Ok(())
    }

    pub async fn unassign_from_reservation(
        pool: &PgPool,
        reserva_id: Uuid,
        etiqueta_id: Uuid,
    ) -> Result<(), AppError> {
        if !EtiquetaRepository::unassign_from_reservation(pool, reserva_id, etiqueta_id).await? {
            return Err(AppError::NotFound("Asignación no encontrada".into()));
        }
        Ok(())
    }

    pub async fn get_reservation_tags(
        pool: &PgPool,
        reserva_id: Uuid,
    ) -> Result<Vec<EtiquetaConCategoria>, AppError> {
        let tags = EtiquetaRepository::get_reservation_tags(pool, reserva_id).await?;
        Ok(tags)
    }
}
