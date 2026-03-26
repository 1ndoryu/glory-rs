/* 263A-1: Servicio de clientes — CRM con búsqueda y paginación. */

use sqlx::PgPool;
use uuid::Uuid;

use crate::errors::AppError;
use crate::models::{
    ActualizarClienteRequest, ClientesPaginados, ClientesQuery, CrearClienteRequest,
    Cliente,
};
use crate::repositories::cliente::{ActualizarClienteData, NuevoCliente};
use crate::repositories::ClienteRepository;

pub struct ClienteService;

impl ClienteService {
    pub async fn create(
        pool: &PgPool,
        user_id: Uuid,
        req: CrearClienteRequest,
    ) -> Result<Cliente, AppError> {
        let data = NuevoCliente {
            user_id,
            nombre: &req.nombre,
            apellidos: req.apellidos.as_deref().unwrap_or(""),
            telefono: req.telefono.as_deref().unwrap_or(""),
            prefijo_telefono: req.prefijo_telefono.as_deref().unwrap_or("+34"),
            email: req.email.as_deref().unwrap_or(""),
            empresa: req.empresa.as_deref().unwrap_or(""),
            notas: req.notas.as_deref().unwrap_or(""),
            foto_url: req.foto_url.as_deref().unwrap_or(""),
            consentimiento_comercial_email: req.consentimiento_comercial_email.unwrap_or(false),
            consentimiento_comercial_sms: req.consentimiento_comercial_sms.unwrap_or(false),
            enviar_encuestas: req.enviar_encuestas.unwrap_or(false),
            alergias: req.alergias.as_deref().unwrap_or(""),
            preferencias_bebida: req.preferencias_bebida.as_deref().unwrap_or(""),
            preferencias_ubicacion: req.preferencias_ubicacion.as_deref().unwrap_or(""),
        };

        let cliente = ClienteRepository::create(pool, &data).await?;
        Ok(cliente)
    }

    pub async fn get(pool: &PgPool, id: Uuid, user_id: Uuid) -> Result<Cliente, AppError> {
        ClienteRepository::find_by_id(pool, id, user_id)
            .await?
            .ok_or_else(|| AppError::NotFound("Cliente no encontrado".into()))
    }

    pub async fn list(
        pool: &PgPool,
        user_id: Uuid,
        query: ClientesQuery,
    ) -> Result<ClientesPaginados, AppError> {
        let page = query.page;
        let per_page = query.per_page;
        let (items, total) =
            ClienteRepository::list(pool, user_id, page, per_page, query.busqueda.as_deref())
                .await?;
        Ok(ClientesPaginados {
            items,
            total,
            page,
            per_page,
        })
    }

    pub async fn update(
        pool: &PgPool,
        id: Uuid,
        user_id: Uuid,
        req: ActualizarClienteRequest,
    ) -> Result<Cliente, AppError> {
        let data = ActualizarClienteData {
            id,
            user_id,
            nombre: req.nombre.as_deref(),
            apellidos: req.apellidos.as_deref(),
            telefono: req.telefono.as_deref(),
            prefijo_telefono: req.prefijo_telefono.as_deref(),
            email: req.email.as_deref(),
            empresa: req.empresa.as_deref(),
            notas: req.notas.as_deref(),
            foto_url: req.foto_url.as_deref(),
            consentimiento_comercial_email: req.consentimiento_comercial_email,
            consentimiento_comercial_sms: req.consentimiento_comercial_sms,
            enviar_encuestas: req.enviar_encuestas,
            alergias: req.alergias.as_deref(),
            preferencias_bebida: req.preferencias_bebida.as_deref(),
            preferencias_ubicacion: req.preferencias_ubicacion.as_deref(),
        };

        ClienteRepository::update(pool, &data)
            .await?
            .ok_or_else(|| AppError::NotFound("Cliente no encontrado".into()))
    }

    pub async fn delete(pool: &PgPool, id: Uuid, user_id: Uuid) -> Result<(), AppError> {
        if !ClienteRepository::delete(pool, id, user_id).await? {
            return Err(AppError::NotFound("Cliente no encontrado".into()));
        }
        Ok(())
    }
}
