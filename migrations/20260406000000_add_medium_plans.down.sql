/* [054A-21] Rollback: eliminar planes Medio */
DELETE FROM service_plan_phases WHERE plan_id IN (
    SELECT id FROM service_plans WHERE slug = 'medio'
);
DELETE FROM service_plans WHERE slug = 'medio';

/* Restaurar highlight y sort_order de Avanzado */
UPDATE service_plans SET sort_order = 2, is_highlighted = true WHERE slug = 'avanzado';
