/* [054A-21] Agrega plan "Medio" a todos los servicios — 3 tiers: Basico, Medio, Avanzado.
 * Precio del plan Medio = punto medio entre Basico y Avanzado con descuento.
 * El plan Personalizado (is_custom=true) se mantiene para cotizaciones especiales. */

/* Diseño Web — Medio */
INSERT INTO service_plans (service_id, slug, name, price_cents, description, features, is_highlighted, sort_order)
SELECT s.id, 'medio', 'Medio', 17500, 'Sitio multi-pagina con blog y SEO basico',
    '[{"texto":"Hasta 3 paginas","incluido":true},{"texto":"Diseno responsive","incluido":true},{"texto":"Formulario de contacto","incluido":true},{"texto":"Hosting 6 meses","incluido":true},{"texto":"Blog integrado","incluido":true},{"texto":"SEO on-page basico","incluido":true},{"texto":"Panel administrable","incluido":false}]'::jsonb,
    true, 2
FROM services s WHERE s.slug = 'diseno-web'
ON CONFLICT (service_id, slug) DO UPDATE SET
    price_cents = EXCLUDED.price_cents,
    description = EXCLUDED.description,
    features = EXCLUDED.features,
    is_highlighted = EXCLUDED.is_highlighted,
    sort_order = EXCLUDED.sort_order;

/* Mover Avanzado a sort_order 3 y quitar highlight */
UPDATE service_plans SET sort_order = 3, is_highlighted = false
FROM services s WHERE service_plans.service_id = s.id AND s.slug = 'diseno-web' AND service_plans.slug = 'avanzado';

/* Desarrollo de Aplicaciones — Medio */
INSERT INTO service_plans (service_id, slug, name, price_cents, description, features, is_highlighted, sort_order)
SELECT s.id, 'medio', 'Medio', 35000, 'App completa con panel admin y auth avanzada',
    '[{"texto":"1 plataforma","incluido":true},{"texto":"Hasta 5 funcionalidades","incluido":true},{"texto":"Auth + roles","incluido":true},{"texto":"API REST completa","incluido":true},{"texto":"Panel de administracion","incluido":true},{"texto":"1 integracion externa","incluido":true},{"texto":"Soporte post-lanzamiento","incluido":false}]'::jsonb,
    true, 2
FROM services s WHERE s.slug = 'desarrollo-apps'
ON CONFLICT (service_id, slug) DO UPDATE SET
    price_cents = EXCLUDED.price_cents,
    description = EXCLUDED.description,
    features = EXCLUDED.features,
    is_highlighted = EXCLUDED.is_highlighted,
    sort_order = EXCLUDED.sort_order;

UPDATE service_plans SET sort_order = 3, is_highlighted = false
FROM services s WHERE service_plans.service_id = s.id AND s.slug = 'desarrollo-apps' AND service_plans.slug = 'avanzado';

/* Agentes IA — Medio */
INSERT INTO service_plans (service_id, slug, name, price_cents, description, features, is_highlighted, sort_order)
SELECT s.id, 'medio', 'Medio', 15000, 'Agente con IA generativa y analisis',
    '[{"texto":"Hasta 2 agentes","incluido":true},{"texto":"5,000 interacciones/mes","incluido":true},{"texto":"Multi-plataforma","incluido":true},{"texto":"IA generativa","incluido":true},{"texto":"Aprendizaje continuo","incluido":true},{"texto":"Dashboard analisis","incluido":false},{"texto":"API personalizada","incluido":false}]'::jsonb,
    true, 2
FROM services s WHERE s.slug = 'agentes-ia'
ON CONFLICT (service_id, slug) DO UPDATE SET
    price_cents = EXCLUDED.price_cents,
    description = EXCLUDED.description,
    features = EXCLUDED.features,
    is_highlighted = EXCLUDED.is_highlighted,
    sort_order = EXCLUDED.sort_order;

UPDATE service_plans SET sort_order = 3, is_highlighted = false
FROM services s WHERE service_plans.service_id = s.id AND s.slug = 'agentes-ia' AND service_plans.slug = 'avanzado';

/* Branding — Medio */
INSERT INTO service_plans (service_id, slug, name, price_cents, description, features, is_highlighted, sort_order)
SELECT s.id, 'medio', 'Medio', 27500, 'Identidad visual con manual y papeleria',
    '[{"texto":"Logo (4 propuestas)","incluido":true},{"texto":"Paleta extendida","incluido":true},{"texto":"Sistema tipografico","incluido":true},{"texto":"Manual de marca","incluido":true},{"texto":"Papeleria corporativa","incluido":true},{"texto":"Kit redes sociales","incluido":false},{"texto":"Iconografia personalizada","incluido":false}]'::jsonb,
    true, 2
FROM services s WHERE s.slug = 'branding'
ON CONFLICT (service_id, slug) DO UPDATE SET
    price_cents = EXCLUDED.price_cents,
    description = EXCLUDED.description,
    features = EXCLUDED.features,
    is_highlighted = EXCLUDED.is_highlighted,
    sort_order = EXCLUDED.sort_order;

UPDATE service_plans SET sort_order = 3, is_highlighted = false
FROM services s WHERE service_plans.service_id = s.id AND s.slug = 'branding' AND service_plans.slug = 'avanzado';

/* E-commerce — Medio */
INSERT INTO service_plans (service_id, slug, name, price_cents, description, features, is_highlighted, sort_order)
SELECT s.id, 'medio', 'Medio', 35000, 'Tienda con inventario, analytics y cupones',
    '[{"texto":"Hasta 200 productos","incluido":true},{"texto":"Multi-pasarela","incluido":true},{"texto":"Gestion inventario","incluido":true},{"texto":"Panel analytics","incluido":true},{"texto":"Cupones y descuentos","incluido":true},{"texto":"Email marketing","incluido":false},{"texto":"Marketplace multi-vendor","incluido":false}]'::jsonb,
    true, 2
FROM services s WHERE s.slug = 'ecommerce'
ON CONFLICT (service_id, slug) DO UPDATE SET
    price_cents = EXCLUDED.price_cents,
    description = EXCLUDED.description,
    features = EXCLUDED.features,
    is_highlighted = EXCLUDED.is_highlighted,
    sort_order = EXCLUDED.sort_order;

UPDATE service_plans SET sort_order = 3, is_highlighted = false
FROM services s WHERE service_plans.service_id = s.id AND s.slug = 'ecommerce' AND service_plans.slug = 'avanzado';

/* SEO — Medio */
INSERT INTO service_plans (service_id, slug, name, price_cents, description, features, is_highlighted, sort_order)
SELECT s.id, 'medio', 'Medio', 27500, 'SEO con contenido y link building basico',
    '[{"texto":"Estrategia integral","incluido":true},{"texto":"15 paginas","incluido":true},{"texto":"2 articulos SEO/mes","incluido":true},{"texto":"Link building basico","incluido":true},{"texto":"SEO local + GMB","incluido":true},{"texto":"Reportes semanales","incluido":false},{"texto":"Analisis competencia","incluido":false}]'::jsonb,
    true, 2
FROM services s WHERE s.slug = 'seo'
ON CONFLICT (service_id, slug) DO UPDATE SET
    price_cents = EXCLUDED.price_cents,
    description = EXCLUDED.description,
    features = EXCLUDED.features,
    is_highlighted = EXCLUDED.is_highlighted,
    sort_order = EXCLUDED.sort_order;

UPDATE service_plans SET sort_order = 3, is_highlighted = false
FROM services s WHERE service_plans.service_id = s.id AND s.slug = 'seo' AND service_plans.slug = 'avanzado';

/* Marketing Digital — Medio */
INSERT INTO service_plans (service_id, slug, name, price_cents, description, features, is_highlighted, sort_order)
SELECT s.id, 'medio', 'Medio', 27500, 'Marketing con email, contenido y ads avanzados',
    '[{"texto":"3 redes sociales","incluido":true},{"texto":"15 publicaciones/mes","incluido":true},{"texto":"Ads ilimitados","incluido":true},{"texto":"Email marketing","incluido":true},{"texto":"Estrategia contenidos","incluido":true},{"texto":"Reportes semanales","incluido":false},{"texto":"A/B testing","incluido":false}]'::jsonb,
    true, 2
FROM services s WHERE s.slug = 'marketing-digital'
ON CONFLICT (service_id, slug) DO UPDATE SET
    price_cents = EXCLUDED.price_cents,
    description = EXCLUDED.description,
    features = EXCLUDED.features,
    is_highlighted = EXCLUDED.is_highlighted,
    sort_order = EXCLUDED.sort_order;

UPDATE service_plans SET sort_order = 3, is_highlighted = false
FROM services s WHERE service_plans.service_id = s.id AND s.slug = 'marketing-digital' AND service_plans.slug = 'avanzado';

/* Fases genéricas para planes medios (3 fases estándar) */
DO $$
DECLARE
    plan RECORD;
BEGIN
    FOR plan IN
        SELECT sp.id as plan_id FROM service_plans sp
        JOIN services s ON sp.service_id = s.id
        WHERE sp.slug = 'medio'
    LOOP
        INSERT INTO service_plan_phases (plan_id, phase_number, title, description, percentage_of_total, estimated_days) VALUES
            (plan.plan_id, 1, 'Descubrimiento y Planificacion', 'Analisis de requisitos, benchmarking, plan de accion', 20, 5),
            (plan.plan_id, 2, 'Ejecucion Principal', 'Desarrollo del servicio con iteraciones intermedias', 50, 10),
            (plan.plan_id, 3, 'Revision y Entrega', 'Ajustes finales, documentacion y entrega', 30, 5)
        ON CONFLICT (plan_id, phase_number) DO NOTHING;
    END LOOP;
END $$;
