ALTER TABLE configuracion_restaurante
    DROP COLUMN IF EXISTS bdp_items_profile_id,
    DROP COLUMN IF EXISTS bdp_employee_id,
    DROP COLUMN IF EXISTS bdp_pos_id,
    DROP COLUMN IF EXISTS bdp_sync_enabled,
    DROP COLUMN IF EXISTS bdp_integrator_code,
    DROP COLUMN IF EXISTS bdp_password,
    DROP COLUMN IF EXISTS bdp_login,
    DROP COLUMN IF EXISTS bdp_base_url;