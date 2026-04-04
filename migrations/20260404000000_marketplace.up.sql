/* [044A-38] Migración marketplace: extensión de users + tablas de órdenes, pagos,
 * reembolsos, reviews, delegaciones, notificaciones y auditoría.
 * Ver plan completo: Agente/planes/plan-marketplace-2026-04-04.md */

/* ============================================================
   1. TIPOS ENUM
   ============================================================ */

CREATE TYPE user_role AS ENUM ('admin', 'employee', 'client');
CREATE TYPE order_status AS ENUM (
    'pending_payment',
    'payment_held',
    'awaiting_assignment',
    'in_progress',
    'under_review',
    'completed',
    'cancelled',
    'disputed'
);
CREATE TYPE payment_mode AS ENUM ('full', 'half_half', 'phased');
CREATE TYPE phase_status AS ENUM (
    'locked',
    'pending_payment',
    'paid',
    'in_progress',
    'delivered',
    'revision_requested',
    'approved',
    'skipped'
);
CREATE TYPE payment_status AS ENUM ('pending', 'held', 'released', 'refunded', 'failed');
CREATE TYPE refund_status AS ENUM ('requested', 'under_review', 'approved', 'completed', 'rejected');
CREATE TYPE delegation_status AS ENUM ('requested', 'accepted', 'rejected', 'completed');

/* ============================================================
   2. EXTENDER USERS
   ============================================================ */

ALTER TABLE users ADD COLUMN role user_role NOT NULL DEFAULT 'client';
ALTER TABLE users ADD COLUMN active_role user_role;
ALTER TABLE users ADD COLUMN email_verified BOOLEAN NOT NULL DEFAULT false;
ALTER TABLE users ADD COLUMN email_verification_token VARCHAR(100);
ALTER TABLE users ADD COLUMN status VARCHAR(20) NOT NULL DEFAULT 'active';

/* Hacer admin al seed user */
UPDATE users SET role = 'admin' WHERE email = 'admin@admin.com';

/* ============================================================
   3. USER PROFILES
   ============================================================ */

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

CREATE TABLE employee_profiles (
    user_id UUID PRIMARY KEY REFERENCES users(id) ON DELETE CASCADE,
    specialties TEXT[] NOT NULL DEFAULT '{}',
    availability VARCHAR(20) NOT NULL DEFAULT 'available',
    max_concurrent_orders INT NOT NULL DEFAULT 3,
    last_activity_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    total_completed_orders INT NOT NULL DEFAULT 0,
    average_rating NUMERIC(3,2) DEFAULT 0.00
);

/* ============================================================
   4. CATÁLOGO DE SERVICIOS
   ============================================================ */

CREATE TABLE services (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    slug VARCHAR(100) NOT NULL UNIQUE,
    title VARCHAR(200) NOT NULL,
    description TEXT,
    base_price_cents INT NOT NULL,
    currency VARCHAR(3) NOT NULL DEFAULT 'USD',
    is_active BOOLEAN NOT NULL DEFAULT true,
    sort_order INT NOT NULL DEFAULT 0,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE service_plans (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    service_id UUID NOT NULL REFERENCES services(id) ON DELETE CASCADE,
    slug VARCHAR(50) NOT NULL,
    name VARCHAR(100) NOT NULL,
    price_cents INT NOT NULL,
    description TEXT,
    features JSONB NOT NULL DEFAULT '[]',
    is_highlighted BOOLEAN NOT NULL DEFAULT false,
    is_custom BOOLEAN NOT NULL DEFAULT false,
    stripe_price_id VARCHAR(100),
    sort_order INT NOT NULL DEFAULT 0,
    UNIQUE(service_id, slug)
);

CREATE TABLE service_plan_phases (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    plan_id UUID NOT NULL REFERENCES service_plans(id) ON DELETE CASCADE,
    phase_number INT NOT NULL,
    title VARCHAR(200) NOT NULL,
    description TEXT,
    percentage_of_total INT NOT NULL,
    estimated_days INT NOT NULL DEFAULT 7,
    max_revisions INT NOT NULL DEFAULT 2,
    UNIQUE(plan_id, phase_number)
);

/* ============================================================
   5. ÓRDENES
   ============================================================ */

CREATE TABLE orders (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    order_number SERIAL,
    client_id UUID NOT NULL REFERENCES users(id),
    service_id UUID NOT NULL REFERENCES services(id),
    plan_id UUID NOT NULL REFERENCES service_plans(id),
    
    payment_mode payment_mode NOT NULL,
    base_price_cents INT NOT NULL,
    discount_percent INT NOT NULL DEFAULT 0,
    final_price_cents INT NOT NULL,
    currency VARCHAR(3) NOT NULL DEFAULT 'USD',
    
    status order_status NOT NULL DEFAULT 'pending_payment',
    
    assigned_employee_id UUID REFERENCES users(id),
    assigned_at TIMESTAMPTZ,
    auto_assign_deadline TIMESTAMPTZ,
    
    current_phase INT NOT NULL DEFAULT 0,
    started_at TIMESTAMPTZ,
    completed_at TIMESTAMPTZ,
    cancelled_at TIMESTAMPTZ,
    
    client_notes TEXT,
    internal_notes TEXT,
    
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_orders_client ON orders(client_id, status);
CREATE INDEX idx_orders_employee ON orders(assigned_employee_id, status);
CREATE INDEX idx_orders_status ON orders(status);
CREATE INDEX idx_orders_auto_assign ON orders(auto_assign_deadline) 
    WHERE status = 'awaiting_assignment';

/* ============================================================
   6. FASES DE ORDEN
   ============================================================ */

CREATE TABLE order_phases (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    order_id UUID NOT NULL REFERENCES orders(id) ON DELETE CASCADE,
    phase_number INT NOT NULL,
    title VARCHAR(200) NOT NULL,
    description TEXT,
    
    price_cents INT NOT NULL DEFAULT 0,
    
    status phase_status NOT NULL DEFAULT 'locked',
    
    max_revisions INT NOT NULL DEFAULT 2,
    revisions_used INT NOT NULL DEFAULT 0,
    
    estimated_days INT NOT NULL DEFAULT 7,
    started_at TIMESTAMPTZ,
    delivered_at TIMESTAMPTZ,
    approved_at TIMESTAMPTZ,
    deadline TIMESTAMPTZ,
    
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    
    UNIQUE(order_id, phase_number)
);

/* ============================================================
   7. ENTREGABLES
   ============================================================ */

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

/* ============================================================
   8. PAGOS
   ============================================================ */

CREATE TABLE order_payments (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    order_id UUID NOT NULL REFERENCES orders(id),
    phase_id UUID REFERENCES order_phases(id),
    
    amount_cents INT NOT NULL,
    currency VARCHAR(3) NOT NULL DEFAULT 'USD',
    status payment_status NOT NULL DEFAULT 'pending',
    payment_mode payment_mode NOT NULL,
    
    stripe_payment_intent_id VARCHAR(100),
    stripe_charge_id VARCHAR(100),
    
    held_at TIMESTAMPTZ,
    released_at TIMESTAMPTZ,
    
    description VARCHAR(500),
    
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_payments_order ON order_payments(order_id);
CREATE INDEX idx_payments_stripe ON order_payments(stripe_payment_intent_id);

/* ============================================================
   9. REEMBOLSOS
   ============================================================ */

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

/* ============================================================
   10. REVIEWS
   ============================================================ */

CREATE TABLE order_reviews (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    order_id UUID NOT NULL UNIQUE REFERENCES orders(id),
    client_id UUID NOT NULL REFERENCES users(id),
    employee_id UUID NOT NULL REFERENCES users(id),
    
    rating INT NOT NULL CHECK (rating >= 1 AND rating <= 5),
    comment TEXT,
    
    employee_response TEXT,
    employee_responded_at TIMESTAMPTZ,
    
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_reviews_employee ON order_reviews(employee_id, rating);

/* ============================================================
   11. DELEGACIONES
   ============================================================ */

CREATE TABLE order_delegations (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    order_id UUID NOT NULL REFERENCES orders(id),
    from_employee_id UUID NOT NULL REFERENCES users(id),
    to_employee_id UUID REFERENCES users(id),
    
    reason TEXT NOT NULL,
    delegation_type VARCHAR(20) NOT NULL,
    status delegation_status NOT NULL DEFAULT 'requested',
    
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    resolved_at TIMESTAMPTZ
);

/* ============================================================
   12. NOTIFICACIONES
   ============================================================ */

CREATE TABLE notifications (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    
    notification_type VARCHAR(50) NOT NULL,
    title VARCHAR(200) NOT NULL,
    body TEXT,
    link VARCHAR(500),
    
    read BOOLEAN NOT NULL DEFAULT false,
    
    reference_type VARCHAR(50),
    reference_id UUID,
    
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_notifications_user ON notifications(user_id, read, created_at DESC);

/* ============================================================
   13. AUDITORÍA
   ============================================================ */

CREATE TABLE activity_log (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id UUID REFERENCES users(id),
    
    action VARCHAR(100) NOT NULL,
    entity_type VARCHAR(50) NOT NULL,
    entity_id UUID NOT NULL,
    details JSONB,
    ip_address INET,
    
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_activity_entity ON activity_log(entity_type, entity_id, created_at DESC);
CREATE INDEX idx_activity_user ON activity_log(user_id, created_at DESC);

/* ============================================================
   14. SEED: SERVICIOS Y PLANES
   ============================================================ */

/* Insertar servicios del catálogo actual */
INSERT INTO services (slug, title, description, base_price_cents, sort_order) VALUES
    ('diseno-web', 'Diseño de Sitios Web', 'Diseño web profesional adaptado a las necesidades del negocio', 10000, 1),
    ('desarrollo-apps', 'Desarrollo de Aplicaciones', 'Aplicaciones móviles y web a medida', 20000, 2),
    ('agentes-ia', 'Agentes de IA', 'Soluciones de inteligencia artificial integradas', 6000, 3),
    ('branding', 'Identidad de Marca', 'Diseño de identidad visual completa', 15000, 4),
    ('ecommerce', 'E-commerce', 'Tiendas online optimizadas para ventas', 20000, 5),
    ('seo', 'SEO', 'Optimización para motores de búsqueda', 15000, 6),
    ('marketing-digital', 'Marketing Digital', 'Estrategias de marketing online', 15000, 7);

/* Planes para Diseño Web */
INSERT INTO service_plans (service_id, slug, name, price_cents, description, features, is_highlighted, sort_order)
SELECT s.id, 'basico', 'Básico', 10000, 'Sitio web de una página, ideal para presencia online básica',
    '[{"texto":"Diseño responsive","incluido":true},{"texto":"1 página","incluido":true},{"texto":"Formulario de contacto","incluido":true},{"texto":"SEO básico","incluido":true},{"texto":"Soporte 30 días","incluido":true},{"texto":"Páginas adicionales","incluido":false},{"texto":"Blog integrado","incluido":false},{"texto":"E-commerce","incluido":false}]'::jsonb,
    false, 1
FROM services s WHERE s.slug = 'diseno-web';

INSERT INTO service_plans (service_id, slug, name, price_cents, description, features, is_highlighted, sort_order)
SELECT s.id, 'avanzado', 'Avanzado', 25000, 'Sitio web multisección con diseño premium',
    '[{"texto":"Diseño responsive","incluido":true},{"texto":"Hasta 5 páginas","incluido":true},{"texto":"Formulario de contacto","incluido":true},{"texto":"SEO avanzado","incluido":true},{"texto":"Soporte 60 días","incluido":true},{"texto":"Blog integrado","incluido":true},{"texto":"Animaciones personalizadas","incluido":true},{"texto":"E-commerce","incluido":false}]'::jsonb,
    true, 2
FROM services s WHERE s.slug = 'diseno-web';

INSERT INTO service_plans (service_id, slug, name, price_cents, description, features, is_highlighted, is_custom, sort_order)
SELECT s.id, 'personalizado', 'Personalizado', 0, 'Sitio web a medida según tus necesidades',
    '[{"texto":"Diseño responsive","incluido":true},{"texto":"Páginas ilimitadas","incluido":true},{"texto":"SEO avanzado","incluido":true},{"texto":"Blog integrado","incluido":true},{"texto":"E-commerce","incluido":true},{"texto":"Funcionalidades a medida","incluido":true},{"texto":"Soporte extendido","incluido":true},{"texto":"Consultoría incluida","incluido":true}]'::jsonb,
    false, true, 3
FROM services s WHERE s.slug = 'diseno-web';

/* Planes para Desarrollo de Aplicaciones */
INSERT INTO service_plans (service_id, slug, name, price_cents, description, features, is_highlighted, sort_order)
SELECT s.id, 'basico', 'Básico', 20000, 'App móvil sencilla con funcionalidades esenciales',
    '[{"texto":"1 plataforma (iOS o Android)","incluido":true},{"texto":"Hasta 5 pantallas","incluido":true},{"texto":"Diseño UI básico","incluido":true},{"texto":"Backend básico","incluido":true},{"texto":"Soporte 30 días","incluido":true},{"texto":"Push notifications","incluido":false},{"texto":"Integraciones API","incluido":false}]'::jsonb,
    false, 1
FROM services s WHERE s.slug = 'desarrollo-apps';

INSERT INTO service_plans (service_id, slug, name, price_cents, description, features, is_highlighted, sort_order)
SELECT s.id, 'avanzado', 'Avanzado', 50000, 'App completa con diseño premium y funcionalidades avanzadas',
    '[{"texto":"Multiplataforma","incluido":true},{"texto":"Pantallas ilimitadas","incluido":true},{"texto":"Diseño UI/UX premium","incluido":true},{"texto":"Backend escalable","incluido":true},{"texto":"Soporte 90 días","incluido":true},{"texto":"Push notifications","incluido":true},{"texto":"Integraciones API","incluido":true}]'::jsonb,
    true, 2
FROM services s WHERE s.slug = 'desarrollo-apps';

INSERT INTO service_plans (service_id, slug, name, price_cents, description, features, is_highlighted, is_custom, sort_order)
SELECT s.id, 'personalizado', 'Personalizado', 0, 'Aplicación a medida para necesidades complejas',
    '[{"texto":"Multiplataforma","incluido":true},{"texto":"Arquitectura a medida","incluido":true},{"texto":"Diseño UI/UX premium","incluido":true},{"texto":"Backend escalable","incluido":true},{"texto":"Soporte extendido","incluido":true},{"texto":"Todas las integraciones","incluido":true},{"texto":"Consultoría incluida","incluido":true}]'::jsonb,
    false, true, 3
FROM services s WHERE s.slug = 'desarrollo-apps';

/* Planes para Agentes IA */
INSERT INTO service_plans (service_id, slug, name, price_cents, description, features, is_highlighted, sort_order)
SELECT s.id, 'basico', 'Básico', 6000, 'Agente IA básico con respuestas automatizadas',
    '[{"texto":"1 agente conversacional","incluido":true},{"texto":"Base de conocimiento básica","incluido":true},{"texto":"Integración web","incluido":true},{"texto":"Soporte 30 días","incluido":true},{"texto":"Múltiples idiomas","incluido":false},{"texto":"Integraciones CRM","incluido":false}]'::jsonb,
    false, 1
FROM services s WHERE s.slug = 'agentes-ia';

INSERT INTO service_plans (service_id, slug, name, price_cents, description, features, is_highlighted, sort_order)
SELECT s.id, 'avanzado', 'Avanzado', 30000, 'Agente IA avanzado con múltiples integraciones',
    '[{"texto":"Agentes múltiples","incluido":true},{"texto":"Base de conocimiento avanzada","incluido":true},{"texto":"Integración multicanal","incluido":true},{"texto":"Soporte 90 días","incluido":true},{"texto":"Múltiples idiomas","incluido":true},{"texto":"Integraciones CRM","incluido":true}]'::jsonb,
    true, 2
FROM services s WHERE s.slug = 'agentes-ia';

INSERT INTO service_plans (service_id, slug, name, price_cents, description, features, is_highlighted, is_custom, sort_order)
SELECT s.id, 'personalizado', 'Personalizado', 0, 'Solución IA a medida para tu negocio',
    '[{"texto":"Agentes ilimitados","incluido":true},{"texto":"Entrenamiento personalizado","incluido":true},{"texto":"Todas las integraciones","incluido":true},{"texto":"Soporte dedicado","incluido":true},{"texto":"Consultoría IA","incluido":true}]'::jsonb,
    false, true, 3
FROM services s WHERE s.slug = 'agentes-ia';

/* Planes para Branding */
INSERT INTO service_plans (service_id, slug, name, price_cents, description, features, is_highlighted, sort_order)
SELECT s.id, 'basico', 'Básico', 15000, 'Identidad visual básica para tu marca',
    '[{"texto":"Logo principal","incluido":true},{"texto":"Paleta de colores","incluido":true},{"texto":"Tipografía","incluido":true},{"texto":"2 revisiones","incluido":true},{"texto":"Manual de marca","incluido":false},{"texto":"Papelería corporativa","incluido":false}]'::jsonb,
    false, 1
FROM services s WHERE s.slug = 'branding';

INSERT INTO service_plans (service_id, slug, name, price_cents, description, features, is_highlighted, sort_order)
SELECT s.id, 'avanzado', 'Avanzado', 40000, 'Identidad de marca completa con aplicaciones',
    '[{"texto":"Logo + variaciones","incluido":true},{"texto":"Paleta de colores","incluido":true},{"texto":"Tipografía completa","incluido":true},{"texto":"5 revisiones","incluido":true},{"texto":"Manual de marca","incluido":true},{"texto":"Papelería corporativa","incluido":true}]'::jsonb,
    true, 2
FROM services s WHERE s.slug = 'branding';

INSERT INTO service_plans (service_id, slug, name, price_cents, description, features, is_highlighted, is_custom, sort_order)
SELECT s.id, 'personalizado', 'Personalizado', 0, 'Estrategia de marca completa a medida',
    '[{"texto":"Todo lo anterior","incluido":true},{"texto":"Estrategia de posicionamiento","incluido":true},{"texto":"Redes sociales","incluido":true},{"texto":"Consultoría de marca","incluido":true}]'::jsonb,
    false, true, 3
FROM services s WHERE s.slug = 'branding';

/* Planes para E-commerce */
INSERT INTO service_plans (service_id, slug, name, price_cents, description, features, is_highlighted, sort_order)
SELECT s.id, 'basico', 'Básico', 20000, 'Tienda online básica lista para vender',
    '[{"texto":"Hasta 50 productos","incluido":true},{"texto":"Pasarela de pago","incluido":true},{"texto":"Diseño responsive","incluido":true},{"texto":"Soporte 30 días","incluido":true},{"texto":"Gestión de inventario","incluido":false},{"texto":"Multi-idioma","incluido":false}]'::jsonb,
    false, 1
FROM services s WHERE s.slug = 'ecommerce';

INSERT INTO service_plans (service_id, slug, name, price_cents, description, features, is_highlighted, sort_order)
SELECT s.id, 'avanzado', 'Avanzado', 50000, 'Tienda online completa con funcionalidades avanzadas',
    '[{"texto":"Productos ilimitados","incluido":true},{"texto":"Múltiples pasarelas","incluido":true},{"texto":"Diseño premium","incluido":true},{"texto":"Soporte 90 días","incluido":true},{"texto":"Gestión de inventario","incluido":true},{"texto":"Multi-idioma","incluido":true}]'::jsonb,
    true, 2
FROM services s WHERE s.slug = 'ecommerce';

INSERT INTO service_plans (service_id, slug, name, price_cents, description, features, is_highlighted, is_custom, sort_order)
SELECT s.id, 'personalizado', 'Personalizado', 0, 'E-commerce a medida con integraciones',
    '[{"texto":"Todo lo anterior","incluido":true},{"texto":"Integraciones ERP","incluido":true},{"texto":"Marketplace","incluido":true},{"texto":"Consultoría e-commerce","incluido":true}]'::jsonb,
    false, true, 3
FROM services s WHERE s.slug = 'ecommerce';

/* Planes para SEO */
INSERT INTO service_plans (service_id, slug, name, price_cents, description, features, is_highlighted, sort_order)
SELECT s.id, 'basico', 'Básico', 15000, 'Optimización SEO fundamental',
    '[{"texto":"Auditoría SEO","incluido":true},{"texto":"Optimización on-page","incluido":true},{"texto":"Configuración analytics","incluido":true},{"texto":"Reporte mensual","incluido":true},{"texto":"Link building","incluido":false},{"texto":"Contenido SEO","incluido":false}]'::jsonb,
    false, 1
FROM services s WHERE s.slug = 'seo';

INSERT INTO service_plans (service_id, slug, name, price_cents, description, features, is_highlighted, sort_order)
SELECT s.id, 'avanzado', 'Avanzado', 40000, 'Estrategia SEO completa con contenido',
    '[{"texto":"Auditoría SEO completa","incluido":true},{"texto":"Optimización on-page y off-page","incluido":true},{"texto":"Analytics avanzado","incluido":true},{"texto":"Reportes semanales","incluido":true},{"texto":"Link building","incluido":true},{"texto":"Contenido SEO","incluido":true}]'::jsonb,
    true, 2
FROM services s WHERE s.slug = 'seo';

INSERT INTO service_plans (service_id, slug, name, price_cents, description, features, is_highlighted, is_custom, sort_order)
SELECT s.id, 'personalizado', 'Personalizado', 0, 'Estrategia SEO personalizada',
    '[{"texto":"Todo lo anterior","incluido":true},{"texto":"Consultoría SEO dedicada","incluido":true},{"texto":"Estrategia de contenidos","incluido":true}]'::jsonb,
    false, true, 3
FROM services s WHERE s.slug = 'seo';

/* Planes para Marketing Digital */
INSERT INTO service_plans (service_id, slug, name, price_cents, description, features, is_highlighted, sort_order)
SELECT s.id, 'basico', 'Básico', 15000, 'Plan de marketing digital fundamental',
    '[{"texto":"Estrategia básica","incluido":true},{"texto":"Gestión 2 redes sociales","incluido":true},{"texto":"Reporte mensual","incluido":true},{"texto":"Publicidad básica","incluido":true},{"texto":"Email marketing","incluido":false},{"texto":"Campañas avanzadas","incluido":false}]'::jsonb,
    false, 1
FROM services s WHERE s.slug = 'marketing-digital';

INSERT INTO service_plans (service_id, slug, name, price_cents, description, features, is_highlighted, sort_order)
SELECT s.id, 'avanzado', 'Avanzado', 40000, 'Estrategia de marketing digital completa',
    '[{"texto":"Estrategia integral","incluido":true},{"texto":"Gestión todas las redes","incluido":true},{"texto":"Reportes semanales","incluido":true},{"texto":"Publicidad avanzada","incluido":true},{"texto":"Email marketing","incluido":true},{"texto":"Campañas avanzadas","incluido":true}]'::jsonb,
    true, 2
FROM services s WHERE s.slug = 'marketing-digital';

INSERT INTO service_plans (service_id, slug, name, price_cents, description, features, is_highlighted, is_custom, sort_order)
SELECT s.id, 'personalizado', 'Personalizado', 0, 'Estrategia de marketing a medida',
    '[{"texto":"Todo lo anterior","incluido":true},{"texto":"Consultoría dedicada","incluido":true},{"texto":"Automatización","incluido":true},{"texto":"Growth hacking","incluido":true}]'::jsonb,
    false, true, 3
FROM services s WHERE s.slug = 'marketing-digital';

/* ============================================================
   15. FASES POR DEFECTO PARA CADA PLAN (ejemplo: Diseño Web)
   ============================================================ */

/* Fases para Diseño Web - Básico */
INSERT INTO service_plan_phases (plan_id, phase_number, title, description, percentage_of_total, estimated_days)
SELECT sp.id, 1, 'Descubrimiento y Briefing', 'Reunión inicial, análisis de requisitos, referencias visuales', 20, 3
FROM service_plans sp JOIN services s ON sp.service_id = s.id WHERE s.slug = 'diseno-web' AND sp.slug = 'basico';

INSERT INTO service_plan_phases (plan_id, phase_number, title, description, percentage_of_total, estimated_days)
SELECT sp.id, 2, 'Diseño y Maquetación', 'Diseño visual, maqueta responsive, iteraciones', 50, 7
FROM service_plans sp JOIN services s ON sp.service_id = s.id WHERE s.slug = 'diseno-web' AND sp.slug = 'basico';

INSERT INTO service_plan_phases (plan_id, phase_number, title, description, percentage_of_total, estimated_days)
SELECT sp.id, 3, 'Desarrollo y Entrega', 'Implementación, pruebas, puesta en producción', 30, 5
FROM service_plans sp JOIN services s ON sp.service_id = s.id WHERE s.slug = 'diseno-web' AND sp.slug = 'basico';

/* Fases para Diseño Web - Avanzado */
INSERT INTO service_plan_phases (plan_id, phase_number, title, description, percentage_of_total, estimated_days)
SELECT sp.id, 1, 'Descubrimiento y Estrategia', 'Briefing, benchmarking, arquitectura de información', 15, 5
FROM service_plans sp JOIN services s ON sp.service_id = s.id WHERE s.slug = 'diseno-web' AND sp.slug = 'avanzado';

INSERT INTO service_plan_phases (plan_id, phase_number, title, description, percentage_of_total, estimated_days)
SELECT sp.id, 2, 'Diseño UI/UX', 'Wireframes, diseño visual, prototipos interactivos', 30, 10
FROM service_plans sp JOIN services s ON sp.service_id = s.id WHERE s.slug = 'diseno-web' AND sp.slug = 'avanzado';

INSERT INTO service_plan_phases (plan_id, phase_number, title, description, percentage_of_total, estimated_days)
SELECT sp.id, 3, 'Desarrollo Frontend', 'Maquetación responsive, animaciones, interacciones', 30, 10
FROM service_plans sp JOIN services s ON sp.service_id = s.id WHERE s.slug = 'diseno-web' AND sp.slug = 'avanzado';

INSERT INTO service_plan_phases (plan_id, phase_number, title, description, percentage_of_total, estimated_days)
SELECT sp.id, 4, 'Integración y QA', 'Backend, testing, optimización, deployment', 25, 7
FROM service_plans sp JOIN services s ON sp.service_id = s.id WHERE s.slug = 'diseno-web' AND sp.slug = 'avanzado';

/* Fases genéricas para planes básicos de los otros servicios (3 fases estándar) */
DO $$
DECLARE
    plan RECORD;
BEGIN
    FOR plan IN 
        SELECT sp.id as plan_id FROM service_plans sp 
        JOIN services s ON sp.service_id = s.id 
        WHERE sp.slug = 'basico' AND s.slug != 'diseno-web'
    LOOP
        INSERT INTO service_plan_phases (plan_id, phase_number, title, description, percentage_of_total, estimated_days) VALUES
            (plan.plan_id, 1, 'Descubrimiento', 'Análisis de requisitos, briefing inicial, planificación', 20, 5),
            (plan.plan_id, 2, 'Ejecución', 'Desarrollo principal del servicio contratado', 50, 10),
            (plan.plan_id, 3, 'Entrega', 'Revisión final, ajustes, documentación y entrega', 30, 5);
    END LOOP;
END $$;

/* Fases genéricas para planes avanzados de los otros servicios (4 fases estándar) */
DO $$
DECLARE
    plan RECORD;
BEGIN
    FOR plan IN 
        SELECT sp.id as plan_id FROM service_plans sp 
        JOIN services s ON sp.service_id = s.id 
        WHERE sp.slug = 'avanzado' AND s.slug != 'diseno-web'
    LOOP
        INSERT INTO service_plan_phases (plan_id, phase_number, title, description, percentage_of_total, estimated_days) VALUES
            (plan.plan_id, 1, 'Descubrimiento y Estrategia', 'Análisis profundo, benchmarking, planificación estratégica', 15, 5),
            (plan.plan_id, 2, 'Diseño y Planificación', 'Diseño detallado, propuestas, aprobación de concepto', 25, 7),
            (plan.plan_id, 3, 'Ejecución', 'Desarrollo principal con iteraciones', 35, 14),
            (plan.plan_id, 4, 'QA y Entrega', 'Testing, optimización, documentación, entrega final', 25, 7);
    END LOOP;
END $$;
