# Plan Maestro: Marketplace de Servicios Nakomi Studio
**Fecha:** 2026-04-04  
**ID tarea:** 044A-38  
**Estado:** Planificación  
**Relación:** Integra y extiende `plan-live-chat-2026-04-04.md`

---

## Visión general

Transformar nakomi.studio de un sitio vitrina a un marketplace funcional tipo Fiverr donde:
- **Clientes** solicitan servicios, pagan, ven progreso, se comunican con empleados
- **Empleados** toman/delegan trabajos, entregan por fases, colaboran entre sí
- **Admin** supervisa todo, asigna trabajo, gestiona reembolsos, cambia de rol para probar

---

## Cosas que el usuario probablemente olvida

Estos puntos NO fueron mencionados pero son esenciales para un marketplace funcional:

1. **Verificación de email** — Sin esto, cualquier email falso puede crear cuenta
2. **Perfil de usuario expandido** — Nombre, avatar, teléfono, empresa (actualmente solo email)
3. **Notificaciones** — In-app + email (nuevo pedido, mensaje, pago, asignación, deadline)
4. **Sistema de archivos/entregables** — Empleados entregan archivos, clientes los aprueban/rechazan
5. **Aceptación de términos/contrato** — Antes de iniciar trabajo, ambas partes aceptan condiciones
6. **Deadlines y SLA** — Tiempo estimado por servicio/fase, alertas de retraso
7. **Reviews/calificaciones** — Clientes califican servicio completado (reputación)
8. **Historial de actividad/auditoría** — Quién hizo qué y cuándo (legal + debugging)
9. **Dashboard admin** — Revenue, órdenes activas, rendimiento de empleados, métricas
10. **Disponibilidad de empleados** — Estado online/ocupado/ausente para auto-asignación inteligente
11. **Facturas/recibos automáticos** — PDF generado por cada pago
12. **Resolución de disputas** — Si cliente y empleado no acuerdan, admin media
13. **Múltiples revisiones por fase** — "Incluye N revisiones" antes de cobrar extra
14. **Stripe Connect** — Para que empleados reciban su parte directamente (split payments)
15. **Webhook de Stripe** — Para confirmar pagos, fallos, reembolsos automáticamente
16. **Protección de datos** — GDPR: exportar datos, eliminar cuenta, consentimiento explícito
17. **Rate limiting en API** — Prevenir abuso de endpoints públicos
18. **2FA opcional** — Seguridad extra para cuentas admin/empleado
19. **Sistema de cupones/descuentos** — Opcional pero útil para marketing
20. **Onboarding de empleados** — Proceso de alta: portfolio, especialidades, verificación

---

## Modelo de datos completo

### Diagrama de relaciones

```
users (roles: admin/employee/client)
  │
  ├── user_profiles (nombre, avatar, teléfono, empresa...)
  │
  ├── orders ────────────── order_phases ────── phase_deliverables
  │     │                        │
  │     ├── order_payments       ├── phase_payments
  │     │
  │     ├── order_assignments ──── (employee asignado)
  │     │
  │     ├── order_refunds
  │     │
  │     └── order_reviews
  │
  ├── chat_sessions ──── chat_messages (del plan de chat)
  │
  └── notifications
```

### Tablas SQL (migración incremental)

#### 1. Extensión de users + profiles

```sql
/* Extender tabla users existente */
ALTER TABLE users ADD COLUMN role VARCHAR(20) NOT NULL DEFAULT 'client';
ALTER TABLE users ADD COLUMN active_role VARCHAR(20); -- Solo admin: rol simulado activo
ALTER TABLE users ADD COLUMN email_verified BOOLEAN NOT NULL DEFAULT false;
ALTER TABLE users ADD COLUMN email_verification_token VARCHAR(100);
ALTER TABLE users ADD COLUMN status VARCHAR(20) NOT NULL DEFAULT 'active'; -- active | suspended | deleted

CREATE TABLE user_profiles (
    user_id UUID PRIMARY KEY REFERENCES users(id) ON DELETE CASCADE,
    name VARCHAR(200),
    avatar_url VARCHAR(500),
    phone VARCHAR(30),
    company VARCHAR(200),
    bio TEXT,
    timezone VARCHAR(50) DEFAULT 'America/New_York',
    language VARCHAR(5) DEFAULT 'es',
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

/* Para empleados: especialidades y disponibilidad */
CREATE TABLE employee_profiles (
    user_id UUID PRIMARY KEY REFERENCES users(id) ON DELETE CASCADE,
    specialties TEXT[] NOT NULL DEFAULT '{}',     -- ['web', 'branding', 'ia']
    availability VARCHAR(20) NOT NULL DEFAULT 'available', -- available | busy | away | offline
    max_concurrent_orders INT NOT NULL DEFAULT 3,
    last_activity_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    total_completed_orders INT NOT NULL DEFAULT 0,
    average_rating NUMERIC(3,2) DEFAULT 0.00
);
```

#### 2. Servicios y fases (catálogo)

```sql
/* Catálogo de servicios (reemplaza datos estáticos del frontend eventualmente) */
CREATE TABLE services (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    slug VARCHAR(100) NOT NULL UNIQUE,
    title VARCHAR(200) NOT NULL,
    description TEXT,
    base_price_cents INT NOT NULL,         -- Precio plan básico en centavos
    currency VARCHAR(3) NOT NULL DEFAULT 'USD',
    is_active BOOLEAN NOT NULL DEFAULT true,
    sort_order INT NOT NULL DEFAULT 0,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

/* Planes por servicio (básico/intermedio/completo) */
CREATE TABLE service_plans (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    service_id UUID NOT NULL REFERENCES services(id) ON DELETE CASCADE,
    slug VARCHAR(50) NOT NULL,             -- 'basico' | 'avanzado' | 'personalizado'
    name VARCHAR(100) NOT NULL,
    price_cents INT NOT NULL,              -- 0 = cotización personalizada
    description TEXT,
    features JSONB NOT NULL DEFAULT '[]',  -- [{text, included}]
    is_highlighted BOOLEAN NOT NULL DEFAULT false,
    is_custom BOOLEAN NOT NULL DEFAULT false,
    stripe_price_id VARCHAR(100),
    sort_order INT NOT NULL DEFAULT 0,
    UNIQUE(service_id, slug)
);

/* Fases predefinidas por plan (template) */
CREATE TABLE service_plan_phases (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    plan_id UUID NOT NULL REFERENCES service_plans(id) ON DELETE CASCADE,
    phase_number INT NOT NULL,
    title VARCHAR(200) NOT NULL,
    description TEXT,
    percentage_of_total INT NOT NULL,      -- Ej: 30% del precio total
    estimated_days INT NOT NULL DEFAULT 7,
    max_revisions INT NOT NULL DEFAULT 2,
    UNIQUE(plan_id, phase_number)
);
```

#### 3. Órdenes (pedidos)

```sql
CREATE TYPE order_status AS ENUM (
    'pending_payment',      -- Cliente creó orden, no ha pagado
    'payment_held',         -- Pago retenido (escrow)
    'awaiting_assignment',  -- Pagado, esperando que alguien lo tome
    'in_progress',          -- Empleado trabajando
    'under_review',         -- Entrega pendiente de aprobación del cliente
    'completed',            -- Cliente aprobó y servicio terminado
    'cancelled',            -- Cancelado (con o sin reembolso)
    'disputed'              -- En disputa
);

CREATE TYPE payment_mode AS ENUM (
    'full',                 -- Pago completo (20% descuento)
    'half_half',            -- 50/50 (10% descuento)
    'phased'                -- Por fases (sin descuento)
);

CREATE TABLE orders (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    order_number SERIAL,                    -- Número legible: #1001, #1002...
    client_id UUID NOT NULL REFERENCES users(id),
    service_id UUID NOT NULL REFERENCES services(id),
    plan_id UUID NOT NULL REFERENCES service_plans(id),
    
    /* Pricing */
    payment_mode payment_mode NOT NULL,
    base_price_cents INT NOT NULL,          -- Precio del plan
    discount_percent INT NOT NULL DEFAULT 0, -- 20% full, 10% half
    final_price_cents INT NOT NULL,         -- Después del descuento
    currency VARCHAR(3) NOT NULL DEFAULT 'USD',
    
    /* Estado */
    status order_status NOT NULL DEFAULT 'pending_payment',
    
    /* Asignación */
    assigned_employee_id UUID REFERENCES users(id),
    assigned_at TIMESTAMPTZ,
    auto_assign_deadline TIMESTAMPTZ,       -- 24h después de payment_held
    
    /* Tracking */
    current_phase INT NOT NULL DEFAULT 0,
    started_at TIMESTAMPTZ,
    completed_at TIMESTAMPTZ,
    cancelled_at TIMESTAMPTZ,
    
    /* Notas */
    client_notes TEXT,                      -- Requisitos iniciales del cliente
    internal_notes TEXT,                    -- Notas internas (solo staff)
    
    /* Chat */
    chat_session_id UUID REFERENCES chat_sessions(id), -- Vinculo con chat
    
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_orders_client ON orders(client_id, status);
CREATE INDEX idx_orders_employee ON orders(assigned_employee_id, status);
CREATE INDEX idx_orders_status ON orders(status);
CREATE INDEX idx_orders_auto_assign ON orders(auto_assign_deadline) 
    WHERE status = 'awaiting_assignment';
```

#### 4. Fases de la orden

```sql
CREATE TYPE phase_status AS ENUM (
    'locked',               -- No se puede trabajar aún
    'pending_payment',      -- Fase requiere pago (modo phased)
    'paid',                 -- Pagado, listo para trabajar
    'in_progress',          -- Empleado trabajando
    'delivered',            -- Entregado, esperando aprobación
    'revision_requested',   -- Cliente pidió cambios
    'approved',             -- Cliente aprobó
    'skipped'               -- Fase omitida (personalización)
);

CREATE TABLE order_phases (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    order_id UUID NOT NULL REFERENCES orders(id) ON DELETE CASCADE,
    phase_number INT NOT NULL,
    title VARCHAR(200) NOT NULL,
    description TEXT,
    
    /* Pricing (solo relevante en modo phased) */
    price_cents INT NOT NULL DEFAULT 0,
    
    /* Estado */
    status phase_status NOT NULL DEFAULT 'locked',
    
    /* Revisiones */
    max_revisions INT NOT NULL DEFAULT 2,
    revisions_used INT NOT NULL DEFAULT 0,
    
    /* Tracking */
    estimated_days INT NOT NULL DEFAULT 7,
    started_at TIMESTAMPTZ,
    delivered_at TIMESTAMPTZ,
    approved_at TIMESTAMPTZ,
    deadline TIMESTAMPTZ,
    
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    
    UNIQUE(order_id, phase_number)
);
```

#### 5. Entregables y archivos

```sql
CREATE TABLE phase_deliverables (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    phase_id UUID NOT NULL REFERENCES order_phases(id) ON DELETE CASCADE,
    uploaded_by UUID NOT NULL REFERENCES users(id),
    file_name VARCHAR(500) NOT NULL,
    file_url VARCHAR(1000) NOT NULL,
    file_size_bytes BIGINT,
    mime_type VARCHAR(100),
    revision_number INT NOT NULL DEFAULT 1,
    notes TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_deliverables_phase ON phase_deliverables(phase_id, revision_number);
```

#### 6. Pagos

```sql
CREATE TYPE payment_status AS ENUM (
    'pending',              -- Esperando procesamiento
    'held',                 -- Retenido en escrow
    'released',             -- Liberado al negocio
    'refunded',             -- Reembolsado
    'failed'                -- Falló el cobro
);

CREATE TABLE order_payments (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    order_id UUID NOT NULL REFERENCES orders(id),
    phase_id UUID REFERENCES order_phases(id),  -- NULL si es pago full o half
    
    amount_cents INT NOT NULL,
    currency VARCHAR(3) NOT NULL DEFAULT 'USD',
    status payment_status NOT NULL DEFAULT 'pending',
    payment_mode payment_mode NOT NULL,
    
    /* Stripe */
    stripe_payment_intent_id VARCHAR(100),
    stripe_charge_id VARCHAR(100),
    
    /* Escrow */
    held_at TIMESTAMPTZ,
    released_at TIMESTAMPTZ,
    
    /* Metadata */
    description VARCHAR(500),
    
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_payments_order ON order_payments(order_id);
CREATE INDEX idx_payments_stripe ON order_payments(stripe_payment_intent_id);
```

#### 7. Reembolsos

```sql
CREATE TYPE refund_status AS ENUM (
    'requested',            -- Cliente pidió reembolso
    'under_review',         -- Admin revisando
    'approved',             -- Aprobado, procesando
    'completed',            -- Dinero devuelto
    'rejected'              -- Rechazado
);

CREATE TABLE order_refunds (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    order_id UUID NOT NULL REFERENCES orders(id),
    payment_id UUID NOT NULL REFERENCES order_payments(id),
    requested_by UUID NOT NULL REFERENCES users(id),
    reviewed_by UUID REFERENCES users(id),
    
    amount_cents INT NOT NULL,
    reason TEXT NOT NULL,
    admin_response TEXT,
    status refund_status NOT NULL DEFAULT 'requested',
    
    stripe_refund_id VARCHAR(100),
    
    requested_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    reviewed_at TIMESTAMPTZ,
    completed_at TIMESTAMPTZ
);
```

#### 8. Reviews

```sql
CREATE TABLE order_reviews (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    order_id UUID NOT NULL UNIQUE REFERENCES orders(id),
    client_id UUID NOT NULL REFERENCES users(id),
    employee_id UUID NOT NULL REFERENCES users(id),
    
    rating INT NOT NULL CHECK (rating >= 1 AND rating <= 5),
    comment TEXT,
    
    /* El empleado puede responder */
    employee_response TEXT,
    employee_responded_at TIMESTAMPTZ,
    
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_reviews_employee ON order_reviews(employee_id, rating);
```

#### 9. Delegaciones

```sql
CREATE TYPE delegation_status AS ENUM (
    'requested',            -- Empleado pidió delegar
    'accepted',             -- Otro empleado aceptó
    'rejected',             -- Nadie aceptó / admin rechazó
    'completed'             -- Transferencia completada
);

CREATE TABLE order_delegations (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    order_id UUID NOT NULL REFERENCES orders(id),
    from_employee_id UUID NOT NULL REFERENCES users(id),
    to_employee_id UUID REFERENCES users(id),    -- NULL hasta que alguien acepte
    
    reason TEXT NOT NULL,
    type VARCHAR(20) NOT NULL,  -- 'delegate' | 'help_request'
    status delegation_status NOT NULL DEFAULT 'requested',
    
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    resolved_at TIMESTAMPTZ
);
```

#### 10. Notificaciones

```sql
CREATE TABLE notifications (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    
    type VARCHAR(50) NOT NULL,   -- 'new_order' | 'assignment' | 'message' | 'payment' | 'delivery' | 'review' | 'refund' | 'deadline'
    title VARCHAR(200) NOT NULL,
    body TEXT,
    link VARCHAR(500),           -- URL para navegar al contexto
    
    read BOOLEAN NOT NULL DEFAULT false,
    
    /* Referencia al objeto que generó la notificación */
    reference_type VARCHAR(50),  -- 'order' | 'chat' | 'payment' | 'refund'
    reference_id UUID,
    
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_notifications_user ON notifications(user_id, read, created_at DESC);
```

#### 11. Actividad/auditoría

```sql
CREATE TABLE activity_log (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id UUID REFERENCES users(id),
    
    action VARCHAR(100) NOT NULL,   -- 'order.created' | 'order.assigned' | 'payment.held' | 'phase.delivered'
    entity_type VARCHAR(50) NOT NULL,
    entity_id UUID NOT NULL,
    details JSONB,
    ip_address INET,
    
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_activity_entity ON activity_log(entity_type, entity_id, created_at DESC);
CREATE INDEX idx_activity_user ON activity_log(user_id, created_at DESC);
```

---

## Actualizaciones al plan de chat

El plan de chat (`plan-live-chat-2026-04-04.md`) necesita estas extensiones para ser coherente:

### 1. Chat vinculado a órdenes
- `chat_sessions` gana columna `order_id UUID REFERENCES orders(id)` — cuando un cliente tiene una orden, su chat es específico de esa orden
- Chat pre-venta (sin orden): `order_id = NULL`, comportamiento actual del plan
- Chat de una orden: `order_id = {uuid}`, el empleado asignado ve este chat en su panel

### 2. Roles en el chat
- **Antes (plan original):** `sender_type = 'visitor' | 'ai' | 'staff'`
- **Ahora:** `sender_type = 'client' | 'ai' | 'employee' | 'admin'`
- Clientes autenticados usan `sender_id = user.id` (no visitor_id anónimo)
- Si el cliente NO está logueado: sigue funcionando como visitante anónimo (pre-venta)

### 3. Contexto de IA
- La IA recibe contexto de la orden (si existe): servicio, plan, fase actual, historial
- System prompt dinámico según si es pre-venta o soporte de orden activa

### 4. Widget de chat
- Si el usuario está logueado y tiene órdenes activas: lista de chats por orden
- Si no está logueado: chat genérico (comportamiento original)

---

## Arquitectura backend (Rust/Axum)

### Nuevos módulos

```
src/
  models/
    user.rs          (EXTENDER: role, active_role, email_verified)
    order.rs         (NUEVO: Order, OrderPhase, CreateOrderRequest...)
    payment.rs       (NUEVO: Payment, RefundRequest...)
    service.rs       (NUEVO: Service, ServicePlan, ServicePhase)
    notification.rs  (NUEVO: Notification)
    review.rs        (NUEVO: Review)
    delegation.rs    (NUEVO: Delegation)
  
  handlers/
    auth.rs          (EXTENDER: verificar email, role switching)
    orders.rs        (NUEVO: CRUD órdenes, flujo de estados)
    payments.rs      (NUEVO: crear pago, webhooks Stripe)
    admin.rs         (NUEVO: asignación, reembolsos, dashboard)
    employees.rs     (NUEVO: tomar trabajo, delegar, estado)
    notifications.rs (NUEVO: listar, marcar leídas)
    reviews.rs       (NUEVO: crear, responder)
    files.rs         (NUEVO: upload/download entregables)
  
  services/
    auth.rs          (EXTENDER: claims con role)
    order.rs         (NUEVO: lógica de negocio de órdenes)
    payment.rs       (NUEVO: integración Stripe)
    assignment.rs    (NUEVO: auto-asignación 24h)
    notification.rs  (NUEVO: crear + despachar notificaciones)
    escrow.rs        (NUEVO: retención y liberación de pagos)
  
  repositories/
    user.rs          (EXTENDER)
    order.rs         (NUEVO)
    payment.rs       (NUEVO)
    notification.rs  (NUEVO)
  
  middleware/
    auth.rs          (EXTENDER: role-based guards)
    role_guard.rs    (NUEVO: RequireRole<Admin>, RequireRole<Employee>)
```

### JWT Claims extendidos

```rust
pub struct Claims {
    pub sub: Uuid,              // user_id
    pub role: String,           // "admin" | "employee" | "client"
    pub active_role: Option<String>, // Solo admin: rol que está simulando
    pub exp: usize,
}

/* El middleware AuthUser ahora incluye: */
pub struct AuthUser {
    pub user_id: Uuid,
    pub role: UserRole,
    pub effective_role: UserRole,  // active_role si admin, sino role real
}
```

### Endpoints principales

#### Auth (extender)
| Método | Ruta | Descripción | Role |
|---|---|---|---|
| POST | `/api/auth/switch-role` | Admin cambia su active_role | admin |
| POST | `/api/auth/verify-email` | Verificar email con token | público |
| POST | `/api/auth/resend-verification` | Reenviar email de verificación | auth |

#### Servicios (catálogo)
| Método | Ruta | Descripción | Role |
|---|---|---|---|
| GET | `/api/services` | Listar servicios activos | público |
| GET | `/api/services/:slug` | Detalle + planes + fases | público |
| POST | `/api/services` | Crear servicio | admin |
| PUT | `/api/services/:id` | Actualizar servicio | admin |

#### Órdenes
| Método | Ruta | Descripción | Role |
|---|---|---|---|
| POST | `/api/orders` | Crear orden (solicitar servicio) | client |
| GET | `/api/orders` | Mis órdenes (filtro por rol) | auth |
| GET | `/api/orders/:id` | Detalle de orden | auth (propietario/asignado/admin) |
| PATCH | `/api/orders/:id/assign` | Asignar a empleado | admin/employee |
| PATCH | `/api/orders/:id/status` | Cambiar estado | admin/employee |
| POST | `/api/orders/:id/cancel` | Cancelar orden | client/admin |

#### Fases
| Método | Ruta | Descripción | Role |
|---|---|---|---|
| GET | `/api/orders/:id/phases` | Listar fases de la orden | auth |
| POST | `/api/orders/:id/phases/:n/deliver` | Entregar fase | employee |
| POST | `/api/orders/:id/phases/:n/approve` | Aprobar entrega | client |
| POST | `/api/orders/:id/phases/:n/request-revision` | Pedir revisión | client |

#### Pagos
| Método | Ruta | Descripción | Role |
|---|---|---|---|
| POST | `/api/orders/:id/pay` | Iniciar pago (Stripe checkout) | client |
| POST | `/api/webhooks/stripe` | Webhook de Stripe | público (verificado) |
| GET | `/api/orders/:id/payments` | Historial de pagos de orden | auth |

#### Reembolsos
| Método | Ruta | Descripción | Role |
|---|---|---|---|
| POST | `/api/orders/:id/refund` | Solicitar reembolso | client |
| PATCH | `/api/refunds/:id` | Aprobar/rechazar reembolso | admin |
| GET | `/api/refunds` | Listar reembolsos pendientes | admin |

#### Delegaciones
| Método | Ruta | Descripción | Role |
|---|---|---|---|
| POST | `/api/orders/:id/delegate` | Solicitar delegación | employee |
| POST | `/api/orders/:id/help` | Pedir ayuda | employee |
| PATCH | `/api/delegations/:id` | Aceptar/rechazar delegación | employee |

#### Notificaciones
| Método | Ruta | Descripción | Role |
|---|---|---|---|
| GET | `/api/notifications` | Listar notificaciones | auth |
| PATCH | `/api/notifications/:id/read` | Marcar como leída | auth |
| PATCH | `/api/notifications/read-all` | Marcar todas como leídas | auth |

#### Reviews
| Método | Ruta | Descripción | Role |
|---|---|---|---|
| POST | `/api/orders/:id/review` | Dejar review | client |
| POST | `/api/reviews/:id/respond` | Responder review | employee |

#### Admin dashboard
| Método | Ruta | Descripción | Role |
|---|---|---|---|
| GET | `/api/admin/dashboard` | Métricas generales | admin |
| GET | `/api/admin/employees` | Lista de empleados + stats | admin |
| GET | `/api/admin/orders/unassigned` | Órdenes sin asignar | admin |

---

## Arquitectura frontend

### Panel con tabs por rol

```
/panel (ruta única, contenido dinámico por rol)
  │
  ├── Cliente:
  │   ├── Tab "Mis Proyectos"     → órdenes activas con progreso
  │   ├── Tab "Historial"         → órdenes completadas + reviews
  │   ├── Tab "Pagos"             → historial de pagos
  │   ├── Tab "Mensajes"          → chats por orden + pre-venta
  │   ├── Tab "Hosting"           → suscripciones mensuales (futuro)
  │   └── Tab "Perfil"            → configuración de cuenta
  │
  ├── Empleado:
  │   ├── Tab "Mis Trabajos"      → órdenes asignadas a mí
  │   ├── Tab "Disponibles"       → órdenes sin asignar
  │   ├── Tab "Delegaciones"      → solicitudes de ayuda
  │   ├── Tab "Mensajes"          → chats con clientes
  │   ├── Tab "Stats"             → rating, completados, ingresos
  │   └── Tab "Perfil"            → disponibilidad, especialidades
  │
  └── Admin:
      ├── Tab "Dashboard"         → métricas, revenue, alertas
      ├── Tab "Órdenes"           → todas las órdenes (filtros)
      ├── Tab "Sin Asignar"       → órdenes que necesitan empleado
      ├── Tab "Empleados"         → lista + performance
      ├── Tab "Reembolsos"        → solicitudes pendientes
      ├── Tab "Mensajes"          → todos los chats
      ├── Tab "Hosting"           → panel de hosting (plan existente)
      └── Tab "Config"            → servicios, planes, precios
```

### Switch de rol (admin)
- Botón fijo en esquina inferior izquierda: icono con rol actual
- Click → dropdown: "Admin", "Empleado", "Cliente"
- Al cambiar: `POST /api/auth/switch-role` → nuevo JWT con `active_role`
- El panel se recarga mostrando la vista del rol seleccionado
- Es un cambio GENUINO en el JWT, no solo visual

### Flujo de contratación (cliente)

```
1. /servicios/:slug         → Ve planes (básico/avanzado/personalizado)
2. Click "Contratar"        → Modal de configuración:
                               - Seleccionar modo de pago: completo/50-50/fases
                               - Ver descuento aplicable
                               - Ver fases con estimaciones
                               - Agregar notas/requisitos
                               - Aceptar términos
3. "Confirmar"              → POST /api/orders → orden creada (pending_payment)
4. Redirige a Stripe        → stripe.redirectToCheckout()
5. Stripe webhook           → status: payment_held
6. /panel                   → Ve orden en "Mis Proyectos" con estado "Esperando asignación"
7. Admin/empleado toma      → status: in_progress
8. Cliente ve progreso      → fases con indicador visual
9. Empleado entrega fase    → status: delivered → cliente aprueba o pide revisión
10. Todas fases aprobadas   → status: completed → pago liberado → prompt de review
```

### Redirección al login

- Si el usuario está logueado y entra a `/`: redirige a `/panel`
- En `/panel` hay botón "Ver sitio web" que lleva a `/` sin redirigir (flag en URL o state)

---

## Flujo de pagos (Stripe)

### Escrow (retención de pagos)

Stripe soporta retención nativa con PaymentIntents:
```
1. Crear PaymentIntent con capture_method: 'manual'
2. Cliente autoriza → dinero retenido (no cobrado)
3. Cuando servicio completo → capture PaymentIntent (cobro real)
4. Si reembolso → cancel PaymentIntent (libera fondos)
```

### Por modo de pago

| Modo | Flujo |
|---|---|
| **Completo** (20% desc) | 1 PaymentIntent por el total con descuento. Captura al completar orden. |
| **50/50** (10% desc) | 2 PaymentIntents: 50% retenido al inicio, 50% retenido cuando fase intermedia completada. |
| **Por fases** (sin desc) | N PaymentIntents: 1 por fase, cada uno retenido cuando la fase anterior se completa. |

### Webhook events a manejar

| Evento | Acción |
|---|---|
| `payment_intent.succeeded` | Marcar pago como held, avanzar orden |
| `payment_intent.payment_failed` | Marcar pago como failed, notificar cliente |
| `charge.refunded` | Marcar reembolso como completed |

---

## Auto-asignación (24h)

### Lógica

```
Cada minuto (background task o cron):
  SELECT * FROM orders 
  WHERE status = 'awaiting_assignment' 
  AND auto_assign_deadline < NOW();

  Para cada orden:
    1. Buscar empleado con:
       - availability = 'available'
       - specialty que match el servicio
       - concurrent_orders < max_concurrent_orders
       - ORDER BY last_activity_at DESC (más activo primero)
    2. Si encuentra → asignar automáticamente + notificar
    3. Si no encuentra → notificar a todos los admins
```

### Background task en Axum

```rust
/* Tarea periódica usando tokio::spawn + tokio::time::interval */
tokio::spawn(async move {
    let mut interval = tokio::time::interval(Duration::from_secs(60));
    loop {
        interval.tick().await;
        if let Err(e) = auto_assign_service.check_and_assign(&pool).await {
            tracing::error!("Auto-assign failed: {e}");
        }
    }
});
```

---

## Fases de implementación

### Fase 0 — Modelo de datos y migración (lo más difícil)
**Complejidad: Alta**

1. Nueva migración SQL con todas las tablas
2. Extender tabla users con role, active_role, email_verified
3. Modelos Rust (structs + SQLx)
4. Repositorios básicos (CRUD)
5. Seed: hacer admin@admin.com role='admin'
6. Migrar datos estáticos de planes a BD (opcional: mantener frontend data como fallback)

### Fase 1 — Sistema de roles y auth extendido
**Complejidad: Media-Alta**

1. Extender JWT claims con role
2. Middleware de role-based access (RequireRole guards)
3. Endpoint switch-role para admin
4. Frontend: authStore con role, switch button en esquina
5. Panel dinámico por rol (tabs diferentes)
6. Redirección `/` → `/panel` si logueado

### Fase 2 — CRUD de órdenes (core del marketplace)
**Complejidad: Alta**

1. Endpoints de órdenes: crear, listar, detalle, cancelar
2. Endpoints de fases: listar, entregar, aprobar, revisión
3. Máquina de estados de orden (transiciones válidas)
4. Frontend: flujo de contratación desde servicio
5. Frontend: panel cliente → "Mis Proyectos" con progreso

### Fase 3 — Pagos con Stripe
**Complejidad: Alta**

1. Integración Stripe: PaymentIntent con capture manual
2. Webhook handler con verificación de firma
3. Lógica de escrow: retener → liberar / reembolsar
4. 3 modos de pago: completo, 50/50, fases
5. Frontend: checkout con Stripe Elements
6. Historial de pagos en panel

### Fase 4 — Asignación y delegación
**Complejidad: Media**

1. Admin: panel de órdenes sin asignar
2. Empleado: tomar orden disponible
3. Auto-asignación background task (24h deadline)
4. Delegación: solicitar + aceptar/rechazar
5. Solicitud de ayuda (multi-empleado)
6. Notificaciones de asignación

### Fase 5 — Integración con chat (extiende plan existente)
**Complejidad: Media**

1. Vincular chat_sessions con orders
2. Chat específico por orden (empleado ↔ cliente)
3. Chat pre-venta (sin orden, IA responde)
4. Contexto de orden en system prompt de IA
5. Widget de chat muestra conversaciones por orden

### Fase 6 — Entregables y revisiones
**Complejidad: Media**

1. Upload de archivos (S3/local)
2. Entrega por fase con archivos adjuntos
3. Aprobación / solicitud de revisión
4. Límite de revisiones por fase
5. Notificaciones de entrega/revisión

### Fase 7 — Reembolsos
**Complejidad: Media-Baja**

1. Cliente solicita reembolso con razón
2. Admin revisa y aprueba/rechaza
3. Si aprobado → Stripe refund automático
4. Cambio de estado de orden a cancelled
5. Notificaciones a todas las partes

### Fase 8 — Reviews y ratings
**Complejidad: Baja**

1. Prompt de review post-completar orden
2. Rating 1-5 + comentario
3. Empleado puede responder
4. Average rating en perfil de empleado
5. Mostrar reviews en panel admin

### Fase 9 — Notificaciones
**Complejidad: Media**

1. Sistema de notificaciones en BD
2. Endpoint: listar + marcar leídas
3. Frontend: campana con badge de no-leídas
4. WebSocket para notificaciones en tiempo real
5. Email para notificaciones críticas (pago, asignación, reembolso)

### Fase 10 — Dashboard admin
**Complejidad: Media**

1. Revenue total/mensual
2. Órdenes activas/completadas/canceladas
3. Rendimiento de empleados
4. Reembolsos pendientes
5. Alertas (órdenes sin asignar, pagos fallidos, deadlines)

---

## Estimación de orden de ejecución (prioridad real)

| # | Fase | Justificación |
|---|---|---|
| 1 | Fase 0 — Modelo de datos | Todo depende de esto |
| 2 | Fase 1 — Roles y auth | Sin roles no hay paneles diferenciados |
| 3 | Fase 2 — Órdenes | Core del marketplace |
| 4 | Fase 3 — Pagos | Sin pagos no hay negocio |
| 5 | Fase 4 — Asignación | Completa el ciclo de vida de una orden |
| 6 | Fase 5 — Chat integrado | Comunicación durante la orden |
| 7 | Fase 6 — Entregables | Cierre de fases con archivos |
| 8 | Fase 9 — Notificaciones | Casi todo genera notificaciones |
| 9 | Fase 7 — Reembolsos | Confianza del cliente |
| 10 | Fase 8 — Reviews | Reputación |
| 11 | Fase 10 — Dashboard | Admin necesita visibilidad |

---

## Consideraciones de seguridad

- **Stripe webhook verification**: Verificar firma `Stripe-Signature` en cada webhook
- **Role escalation**: Middleware que valida rol en servidor, nunca confiar en el frontend
- **File upload**: Validar tipo MIME, limitar tamaño (10MB), escanear malware básico
- **Rate limiting**: Endpoints públicos max 20/min, auth 60/min, admin sin límite
- **Escrow**: Nunca liberar pago sin aprobación explícita del cliente o admin
- **Audit log**: Toda acción de pago/reembolso/asignación queda registrada
- **Data isolation**: Clientes solo ven sus propias órdenes, empleados solo las asignadas
- **Input validation**: Todos los request bodies validados con `validator` crate
- **SQL injection**: Ya cubierto por SQLx query macros (prepared statements)
