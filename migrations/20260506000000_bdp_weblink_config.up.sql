/* [065A-2] Configuracion para BDP WebLink REST API.
 * Las credenciales viven en BD por restaurante y no se devuelven serializadas
 * desde el modelo Rust. Los IDs operativos permiten probar CreateOrder sin
 * hardcodear terminal, empleado ni perfil de articulos. */

ALTER TABLE configuracion_restaurante
    ADD COLUMN IF NOT EXISTS bdp_base_url TEXT NOT NULL DEFAULT '',
    ADD COLUMN IF NOT EXISTS bdp_login TEXT NOT NULL DEFAULT '',
    ADD COLUMN IF NOT EXISTS bdp_password TEXT NOT NULL DEFAULT '',
    ADD COLUMN IF NOT EXISTS bdp_integrator_code TEXT NOT NULL DEFAULT '',
    ADD COLUMN IF NOT EXISTS bdp_sync_enabled BOOLEAN NOT NULL DEFAULT false,
    ADD COLUMN IF NOT EXISTS bdp_pos_id INTEGER NOT NULL DEFAULT 1,
    ADD COLUMN IF NOT EXISTS bdp_employee_id INTEGER NOT NULL DEFAULT 1,
    ADD COLUMN IF NOT EXISTS bdp_items_profile_id INTEGER NOT NULL DEFAULT 1;