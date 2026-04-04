/* [044A-38] Servicio de órdenes: lógica de negocio para crear órdenes, calcular
 * descuentos según payment_mode, y generar fases automáticamente desde plantillas.
 * Descuentos: Full = 20%, HalfHalf = 10%, Phased = 0%. */

use sqlx::PgPool;
use uuid::Uuid;

use crate::errors::AppError;
use crate::models::{
    CreateOrderRequest, Order, OrderPhaseResponse, OrderResponse,
    OrderStatus, PaymentMode, PhaseStatus, ServiceDetailResponse, ServicePlanResponse,
    ServicePlanPhaseResponse,
};
use crate::repositories::{OrderRepository, CreateOrderParams, CreatePhaseParams};

pub struct OrderService;

impl OrderService {
    /// Lista todos los servicios activos con planes y fases
    pub async fn list_services(pool: &PgPool) -> Result<Vec<ServiceDetailResponse>, AppError> {
        let services = OrderRepository::list_services(pool).await?;
        let mut result = Vec::with_capacity(services.len());

        for svc in services {
            let plans = OrderRepository::list_plans_for_service(pool, svc.id).await?;
            let mut plan_responses = Vec::with_capacity(plans.len());

            for plan in plans {
                let phases = OrderRepository::list_plan_phases(pool, plan.id).await?;
                plan_responses.push(ServicePlanResponse {
                    id: plan.id,
                    slug: plan.slug,
                    name: plan.name,
                    price_cents: plan.price_cents,
                    description: plan.description,
                    features: plan.features,
                    is_highlighted: plan.is_highlighted,
                    is_custom: plan.is_custom,
                    phases: phases.into_iter().map(ServicePlanPhaseResponse::from).collect(),
                });
            }

            result.push(ServiceDetailResponse {
                id: svc.id,
                slug: svc.slug,
                title: svc.title,
                description: svc.description,
                plans: plan_responses,
            });
        }

        Ok(result)
    }

    /// Detalle completo de un servicio por slug
    pub async fn get_service(
        pool: &PgPool,
        slug: &str,
    ) -> Result<ServiceDetailResponse, AppError> {
        let svc = OrderRepository::find_service_by_slug(pool, slug)
            .await?
            .ok_or_else(|| AppError::NotFound(format!("Servicio '{slug}' no encontrado")))?;

        let plans = OrderRepository::list_plans_for_service(pool, svc.id).await?;
        let mut plan_responses = Vec::with_capacity(plans.len());

        for plan in plans {
            let phases = OrderRepository::list_plan_phases(pool, plan.id).await?;
            plan_responses.push(ServicePlanResponse {
                id: plan.id,
                slug: plan.slug,
                name: plan.name,
                price_cents: plan.price_cents,
                description: plan.description,
                features: plan.features,
                is_highlighted: plan.is_highlighted,
                is_custom: plan.is_custom,
                phases: phases.into_iter().map(ServicePlanPhaseResponse::from).collect(),
            });
        }

        Ok(ServiceDetailResponse {
            id: svc.id,
            slug: svc.slug,
            title: svc.title,
            description: svc.description,
            plans: plan_responses,
        })
    }

    /// Crea una orden: resuelve servicio/plan, calcula descuento, genera fases
    pub async fn create_order(
        pool: &PgPool,
        client_id: Uuid,
        req: CreateOrderRequest,
    ) -> Result<OrderResponse, AppError> {
        let svc = OrderRepository::find_service_by_slug(pool, &req.service_slug)
            .await?
            .ok_or_else(|| {
                AppError::NotFound(format!("Servicio '{}' no encontrado", req.service_slug))
            })?;

        let plan = OrderRepository::find_plan_by_slug(pool, svc.id, &req.plan_slug)
            .await?
            .ok_or_else(|| {
                AppError::NotFound(format!("Plan '{}' no encontrado", req.plan_slug))
            })?;

        let base_price = plan.price_cents;
        let discount = Self::discount_for_mode(req.payment_mode);
        let final_price = base_price - (base_price * discount / 100);

        let plan_phases = OrderRepository::list_plan_phases(pool, plan.id).await?;

        let order = OrderRepository::create_order(
            pool,
            CreateOrderParams {
                client_id,
                service_id: svc.id,
                plan_id: plan.id,
                payment_mode: req.payment_mode,
                base_price_cents: base_price,
                discount_percent: discount,
                final_price_cents: final_price,
                client_notes: req.client_notes.as_deref(),
            },
        )
        .await?;

        /* Generar fases de la orden a partir de las plantillas del plan */
        #[allow(clippy::cast_possible_truncation, clippy::cast_possible_wrap)]
        let total_phases = plan_phases.len() as i32;
        for tmpl in &plan_phases {
            let phase_price = final_price * tmpl.percentage_of_total / 100;
            let status = if tmpl.phase_number == 1 {
                Self::initial_phase_status(req.payment_mode)
            } else {
                PhaseStatus::Locked
            };

            OrderRepository::create_order_phase(
                pool,
                CreatePhaseParams {
                    order_id: order.id,
                    phase_number: tmpl.phase_number,
                    title: &tmpl.title,
                    description: tmpl.description.as_deref(),
                    price_cents: phase_price,
                    status,
                    max_revisions: tmpl.max_revisions,
                    estimated_days: tmpl.estimated_days,
                },
            )
            .await?;
        }

        Ok(OrderResponse {
            id: order.id,
            order_number: order.order_number,
            service_title: svc.title,
            plan_name: plan.name,
            payment_mode: order.payment_mode,
            base_price_cents: order.base_price_cents,
            discount_percent: order.discount_percent,
            final_price_cents: order.final_price_cents,
            currency: order.currency,
            status: order.status,
            assigned_employee_id: order.assigned_employee_id,
            current_phase: order.current_phase,
            total_phases,
            client_notes: order.client_notes,
            created_at: order.created_at,
        })
    }

    /// Obtiene el detalle de una orden con sus fases.
    /// Retorna (`client_id`, response, phases) para que el handler verifique acceso.
    pub async fn get_order(
        pool: &PgPool,
        order_id: Uuid,
    ) -> Result<(Uuid, OrderResponse, Vec<OrderPhaseResponse>), AppError> {
        let order = OrderRepository::find_order_by_id(pool, order_id)
            .await?
            .ok_or_else(|| AppError::NotFound("Orden no encontrada".into()))?;

        let phases = OrderRepository::list_order_phases(pool, order_id).await?;
        #[allow(clippy::cast_possible_truncation, clippy::cast_possible_wrap)]
        let total_phases = phases.len() as i32;

        let (svc_title, plan_name) =
            OrderRepository::get_order_display_info(pool, order.service_id, order.plan_id).await?;

        let response = OrderResponse {
            id: order.id,
            order_number: order.order_number,
            service_title: svc_title,
            plan_name,
            payment_mode: order.payment_mode,
            base_price_cents: order.base_price_cents,
            discount_percent: order.discount_percent,
            final_price_cents: order.final_price_cents,
            currency: order.currency,
            status: order.status,
            assigned_employee_id: order.assigned_employee_id,
            current_phase: order.current_phase,
            total_phases,
            client_notes: order.client_notes,
            created_at: order.created_at,
        };

        let phase_responses: Vec<OrderPhaseResponse> =
            phases.into_iter().map(OrderPhaseResponse::from).collect();

        Ok((order.client_id, response, phase_responses))
    }

    /// Lista órdenes según rol: cliente ve las suyas, employee las asignadas, admin todas
    pub async fn list_orders_for_user(
        pool: &PgPool,
        user_id: Uuid,
        effective_role: crate::models::UserRole,
    ) -> Result<Vec<OrderResponse>, AppError> {
        use crate::models::UserRole;

        let orders = match effective_role {
            UserRole::Client => OrderRepository::list_orders_for_client(pool, user_id).await?,
            UserRole::Employee => OrderRepository::list_orders_for_employee(pool, user_id).await?,
            UserRole::Admin => OrderRepository::list_all_orders(pool).await?,
        };

        let mut result = Vec::with_capacity(orders.len());
        for order in orders {
            let (svc_title, plan_name) =
                OrderRepository::get_order_display_info(pool, order.service_id, order.plan_id)
                    .await?;
            let phases = OrderRepository::list_order_phases(pool, order.id).await?;

            result.push(OrderResponse {
                id: order.id,
                order_number: order.order_number,
                service_title: svc_title,
                plan_name,
                payment_mode: order.payment_mode,
                base_price_cents: order.base_price_cents,
                discount_percent: order.discount_percent,
                final_price_cents: order.final_price_cents,
                currency: order.currency,
                status: order.status,
                assigned_employee_id: order.assigned_employee_id,
                current_phase: order.current_phase,
                total_phases: {
                    #[allow(clippy::cast_possible_truncation, clippy::cast_possible_wrap)]
                    { phases.len() as i32 }
                },
                client_notes: order.client_notes,
                created_at: order.created_at,
            });
        }

        Ok(result)
    }

    /// Asigna un empleado a una orden (admin o auto-asignación)
    pub async fn assign_order(
        pool: &PgPool,
        order_id: Uuid,
        employee_id: Uuid,
    ) -> Result<Order, AppError> {
        let order = OrderRepository::find_order_by_id(pool, order_id)
            .await?
            .ok_or_else(|| AppError::NotFound("Orden no encontrada".into()))?;

        if order.status != OrderStatus::AwaitingAssignment
            && order.status != OrderStatus::PaymentHeld
        {
            return Err(AppError::BadRequest(
                "La orden no está en estado de asignación".into(),
            ));
        }

        let assigned = OrderRepository::assign_order(pool, order_id, employee_id).await?;
        Ok(assigned)
    }

    /* ============================================================
       HELPERS PRIVADOS
       ============================================================ */

    /// Descuento por modo de pago: full=20%, `half_half`=10%, phased=0%
    fn discount_for_mode(mode: PaymentMode) -> i32 {
        match mode {
            PaymentMode::Full => 20,
            PaymentMode::HalfHalf => 10,
            PaymentMode::Phased => 0,
        }
    }

    /// Estado inicial de la primera fase según modo de pago
    fn initial_phase_status(mode: PaymentMode) -> PhaseStatus {
        match mode {
            PaymentMode::Full => PhaseStatus::Paid,
            PaymentMode::HalfHalf | PaymentMode::Phased => PhaseStatus::PendingPayment,
        }
    }
}
