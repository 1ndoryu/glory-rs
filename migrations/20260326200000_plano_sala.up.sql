/* [263A-14] Plano de sala: zonas, mesas con posición, combinaciones.
 * Agrega mesa_id a reservas para vincular con mesa real del plano. */

/* Zonas del restaurante (plantas/áreas) */
CREATE TABLE zonas_sala (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    nombre VARCHAR(100) NOT NULL,
    orden INTEGER NOT NULL DEFAULT 0,
    ancho INTEGER NOT NULL DEFAULT 800,
    alto INTEGER NOT NULL DEFAULT 600,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

/* Mesas individuales con posición en el plano */
CREATE TABLE mesas (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    zona_id UUID NOT NULL REFERENCES zonas_sala(id) ON DELETE CASCADE,
    numero INTEGER NOT NULL,
    pos_x INTEGER NOT NULL DEFAULT 0,
    pos_y INTEGER NOT NULL DEFAULT 0,
    ancho INTEGER NOT NULL DEFAULT 80,
    alto INTEGER NOT NULL DEFAULT 80,
    forma VARCHAR(20) NOT NULL DEFAULT 'cuadrada',
    min_personas INTEGER NOT NULL DEFAULT 1,
    max_personas INTEGER NOT NULL DEFAULT 4,
    activa BOOLEAN NOT NULL DEFAULT true,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE(zona_id, numero)
);

/* Combinaciones de mesas (grupos que se pueden juntar) */
CREATE TABLE combinaciones_mesas (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    nombre VARCHAR(100) NOT NULL,
    min_personas INTEGER NOT NULL DEFAULT 1,
    max_personas INTEGER NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

/* Items de cada combinación (N:N con mesas) */
CREATE TABLE combinacion_mesa_items (
    combinacion_id UUID NOT NULL REFERENCES combinaciones_mesas(id) ON DELETE CASCADE,
    mesa_id UUID NOT NULL REFERENCES mesas(id) ON DELETE CASCADE,
    PRIMARY KEY (combinacion_id, mesa_id)
);

/* Vincular reservas con mesa real del plano */
ALTER TABLE reservas ADD COLUMN mesa_id UUID REFERENCES mesas(id) ON DELETE SET NULL;
