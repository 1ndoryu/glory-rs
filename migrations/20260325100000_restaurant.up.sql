/* 253A-5: Migración para el sistema de gestión de restaurante.
   Agrega tablas de ventas, gastos, categorías de gasto y reservas.
   Mantiene la tabla users existente, agrega campo nombre. */

/* Agregar nombre al usuario (dueño del restaurante) */
ALTER TABLE users ADD COLUMN IF NOT EXISTS nombre VARCHAR(255) NOT NULL DEFAULT '';

/* Categorías de gasto — precargadas con las del sistema Haddock */
CREATE TABLE categorias_gasto (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    nombre VARCHAR(100) NOT NULL UNIQUE,
    color VARCHAR(7) NOT NULL DEFAULT '#888888',
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

/* Insertar categorías por defecto del sector restauración */
INSERT INTO categorias_gasto (nombre, color) VALUES
    ('Bebidas', '#F5A623'),
    ('Materias Primas', '#F5C451'),
    ('Personal', '#4A90D9'),
    ('Mantenimiento', '#888888'),
    ('Alquiler', '#50E3C2'),
    ('Suministros', '#7EC8E3'),
    ('Comunicación y Marketing', '#7B61FF'),
    ('Limpieza', '#F5A06E'),
    ('Tecnología', '#4A90D9'),
    ('Otros', '#C0C0C0');

/* Ventas del restaurante */
CREATE TABLE ventas (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    fecha DATE NOT NULL,
    comensales INTEGER,
    descripcion TEXT NOT NULL DEFAULT '',
    iva_porcentaje DECIMAL(5,2) NOT NULL DEFAULT 10.00,
    turno VARCHAR(20) NOT NULL CHECK (turno IN ('manana', 'mediodia', 'noche')),
    canal VARCHAR(30) NOT NULL CHECK (canal IN ('comedor', 'barra', 'terraza', 'delivery', 'just_eat', 'eventos')),
    metodo_pago VARCHAR(20) NOT NULL CHECK (metodo_pago IN ('efectivo', 'tarjeta', 'transferencia')),
    importe_base DECIMAL(12,2) NOT NULL,
    importe_iva DECIMAL(12,2) NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_ventas_user_id ON ventas(user_id);
CREATE INDEX idx_ventas_fecha ON ventas(fecha);

/* Gastos del restaurante */
CREATE TABLE gastos (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    fecha DATE NOT NULL,
    proveedor VARCHAR(255) NOT NULL DEFAULT '',
    categoria_id UUID REFERENCES categorias_gasto(id),
    tipo_documento VARCHAR(20) NOT NULL CHECK (tipo_documento IN ('factura', 'albaran', 'ticket')),
    metodo_pago VARCHAR(20) NOT NULL CHECK (metodo_pago IN ('efectivo', 'tarjeta', 'transferencia')),
    numero_documento VARCHAR(100) NOT NULL DEFAULT '',
    recurrente BOOLEAN NOT NULL DEFAULT FALSE,
    importe_base DECIMAL(12,2) NOT NULL,
    importe_iva DECIMAL(12,2) NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_gastos_user_id ON gastos(user_id);
CREATE INDEX idx_gastos_fecha ON gastos(fecha);
CREATE INDEX idx_gastos_categoria ON gastos(categoria_id);

/* Reservas */
CREATE TABLE reservas (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    fecha DATE NOT NULL,
    hora TIME NOT NULL,
    nombre_cliente VARCHAR(255) NOT NULL,
    num_personas INTEGER NOT NULL DEFAULT 1,
    estado VARCHAR(20) NOT NULL DEFAULT 'confirmada'
        CHECK (estado IN ('pendiente', 'confirmada', 'cancelada', 'completada')),
    notas TEXT NOT NULL DEFAULT '',
    telefono VARCHAR(20) NOT NULL DEFAULT '',
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_reservas_user_id ON reservas(user_id);
CREATE INDEX idx_reservas_fecha ON reservas(fecha);
CREATE INDEX idx_reservas_estado ON reservas(estado);

/* Eliminar tabla de notas del template — ya no se usa */
DROP TABLE IF EXISTS notes;
