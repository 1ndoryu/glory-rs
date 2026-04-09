/* [094A-3] Tabla de trabajadores: staff vinculado al propietario.
 * Cada trabajador tiene su propio login y permisos restringidos.
 * El propietario (user original) tiene acceso total. */

CREATE TABLE IF NOT EXISTS trabajadores (
    id UUID PRIMARY KEY,
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    nombre VARCHAR(100) NOT NULL,
    email VARCHAR(255) NOT NULL,
    password_hash VARCHAR(255) NOT NULL,
    cargo VARCHAR(100) NOT NULL DEFAULT '',
    activo BOOLEAN NOT NULL DEFAULT true,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE(user_id, email)
);

/* Permisos por sección — cada fila define si el trabajador puede ver una sección.
 * Si no hay fila para una sección, se asume denegado. */
CREATE TABLE IF NOT EXISTS permisos_trabajador (
    id UUID PRIMARY KEY,
    trabajador_id UUID NOT NULL REFERENCES trabajadores(id) ON DELETE CASCADE,
    seccion VARCHAR(50) NOT NULL,
    permitido BOOLEAN NOT NULL DEFAULT true,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE(trabajador_id, seccion)
);

/* Secciones válidas (referencia, no constraint):
 * reservas, ventas, gastos, clientes, marketing, plano_sala,
 * configuracion, campanas, recordatorios, dashboard, notificaciones */
