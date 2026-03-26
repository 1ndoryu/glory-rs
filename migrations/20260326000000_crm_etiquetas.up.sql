/* 263A-1: Migración para CRM de clientes, etiquetas y canales de reserva.
   Agrega: clientes, etiquetas, categorias_etiqueta, clientes_etiquetas,
   reservas_etiquetas, canales_reserva.
   También agrega 'otros' y 'no_show' como opciones válidas. */

/* ========== CRM DE CLIENTES ========== */

CREATE TABLE clientes (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    nombre VARCHAR(255) NOT NULL,
    apellidos VARCHAR(255) NOT NULL DEFAULT '',
    telefono VARCHAR(30) NOT NULL DEFAULT '',
    prefijo_telefono VARCHAR(10) NOT NULL DEFAULT '+34',
    email VARCHAR(255) NOT NULL DEFAULT '',
    empresa VARCHAR(255) NOT NULL DEFAULT '',
    notas TEXT NOT NULL DEFAULT '',
    foto_url TEXT NOT NULL DEFAULT '',
    consentimiento_comercial_email BOOLEAN NOT NULL DEFAULT FALSE,
    consentimiento_comercial_sms BOOLEAN NOT NULL DEFAULT FALSE,
    enviar_encuestas BOOLEAN NOT NULL DEFAULT TRUE,
    alergias TEXT NOT NULL DEFAULT '',
    preferencias_bebida TEXT NOT NULL DEFAULT '',
    preferencias_ubicacion TEXT NOT NULL DEFAULT '',
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_clientes_user_id ON clientes(user_id);
CREATE INDEX idx_clientes_nombre ON clientes(user_id, nombre, apellidos);
CREATE INDEX idx_clientes_telefono ON clientes(user_id, telefono);
CREATE INDEX idx_clientes_email ON clientes(user_id, email);

/* ========== CATEGORÍAS DE ETIQUETAS ========== */

CREATE TABLE categorias_etiqueta (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id UUID REFERENCES users(id) ON DELETE CASCADE,
    nombre VARCHAR(100) NOT NULL,
    aplica_a VARCHAR(20) NOT NULL CHECK (aplica_a IN ('cliente', 'reserva')),
    es_sistema BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE(user_id, nombre, aplica_a)
);

/* Categorías de etiquetas preestablecidas del sistema (user_id NULL = global) */
INSERT INTO categorias_etiqueta (user_id, nombre, aplica_a, es_sistema) VALUES
    (NULL, 'Fidelización', 'cliente', TRUE),
    (NULL, 'Alergias e Intolerancias', 'cliente', TRUE),
    (NULL, 'Preferencias Alimentarias', 'cliente', TRUE),
    (NULL, 'Preferencias de Bebida', 'cliente', TRUE),
    (NULL, 'Preferencias de Ubicación', 'cliente', TRUE),
    (NULL, 'Eventos', 'reserva', TRUE),
    (NULL, 'Peticiones Especiales', 'reserva', TRUE),
    (NULL, 'Servicios Gastronómicos', 'reserva', TRUE);

/* ========== ETIQUETAS ========== */

CREATE TABLE etiquetas (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id UUID REFERENCES users(id) ON DELETE CASCADE,
    categoria_id UUID NOT NULL REFERENCES categorias_etiqueta(id) ON DELETE CASCADE,
    nombre VARCHAR(100) NOT NULL,
    color VARCHAR(7) NOT NULL DEFAULT '#6B7280',
    es_sistema BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE(user_id, categoria_id, nombre)
);

/* Etiquetas preestablecidas del sistema */
INSERT INTO etiquetas (user_id, categoria_id, nombre, color, es_sistema)
SELECT NULL,
       ce.id,
       e.nombre,
       e.color,
       TRUE
FROM (VALUES
    ('Fidelización', 'cliente', 'VIP', '#FFD700'),
    ('Fidelización', 'cliente', 'Frecuente', '#4CAF50'),
    ('Fidelización', 'cliente', 'Ocasional', '#FF9800'),
    ('Fidelización', 'cliente', 'Poco Frecuente', '#9E9E9E'),
    ('Fidelización', 'cliente', 'No Paga', '#F44336'),
    ('Eventos', 'reserva', 'Cumpleaños', '#E91E63'),
    ('Eventos', 'reserva', 'Evento Corporativo', '#3F51B5'),
    ('Peticiones Especiales', 'reserva', 'Mesa Específica', '#009688'),
    ('Peticiones Especiales', 'reserva', 'Decoración Especial', '#FF5722')
) AS e(categoria, aplica_a, nombre, color)
JOIN categorias_etiqueta ce ON ce.nombre = e.categoria
    AND ce.aplica_a = e.aplica_a
    AND ce.es_sistema = TRUE;

/* ========== RELACIONES M:N ETIQUETAS ========== */

CREATE TABLE clientes_etiquetas (
    cliente_id UUID NOT NULL REFERENCES clientes(id) ON DELETE CASCADE,
    etiqueta_id UUID NOT NULL REFERENCES etiquetas(id) ON DELETE CASCADE,
    PRIMARY KEY (cliente_id, etiqueta_id)
);

CREATE TABLE reservas_etiquetas (
    reserva_id UUID NOT NULL REFERENCES reservas(id) ON DELETE CASCADE,
    etiqueta_id UUID NOT NULL REFERENCES etiquetas(id) ON DELETE CASCADE,
    PRIMARY KEY (reserva_id, etiqueta_id)
);

/* ========== CANALES DE RESERVA ========== */

CREATE TABLE canales_reserva (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    nombre VARCHAR(100) NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE(user_id, nombre)
);

/* Vincular reservas a un cliente y un canal (columnas opcionales) */
ALTER TABLE reservas
    ADD COLUMN cliente_id UUID REFERENCES clientes(id) ON DELETE SET NULL,
    ADD COLUMN canal_id UUID REFERENCES canales_reserva(id) ON DELETE SET NULL,
    ADD COLUMN no_show BOOLEAN NOT NULL DEFAULT FALSE;

CREATE INDEX idx_reservas_cliente ON reservas(cliente_id);
CREATE INDEX idx_reservas_no_show ON reservas(user_id, no_show);

/* ========== MÉTODO DE PAGO "OTROS" ========== */

ALTER TABLE ventas DROP CONSTRAINT IF EXISTS ventas_metodo_pago_check;
ALTER TABLE ventas ADD CONSTRAINT ventas_metodo_pago_check
    CHECK (metodo_pago IN ('efectivo', 'tarjeta', 'transferencia', 'otros'));

/* También para gastos que la estructura lo permita */
ALTER TABLE gastos DROP CONSTRAINT IF EXISTS gastos_metodo_pago_check;
ALTER TABLE gastos ADD CONSTRAINT gastos_metodo_pago_check
    CHECK (metodo_pago IN ('efectivo', 'tarjeta', 'transferencia', 'otros', ''));

/* Estado no_show para reservas */
ALTER TABLE reservas DROP CONSTRAINT IF EXISTS reservas_estado_check;
ALTER TABLE reservas ADD CONSTRAINT reservas_estado_check
    CHECK (estado IN ('pendiente', 'confirmada', 'cancelada', 'completada', 'no_show'));
