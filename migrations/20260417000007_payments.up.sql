BEGIN;

CREATE TABLE IF NOT EXISTS suscripciones (
    id                       SERIAL PRIMARY KEY,
    usuario_id               INT NOT NULL REFERENCES usuarios_ext(id) ON DELETE CASCADE,
    plan                     VARCHAR(20) NOT NULL DEFAULT 'free',
    estado                   VARCHAR(30) NOT NULL DEFAULT 'activa'
                             CHECK (estado IN ('activa', 'cancelada', 'vencida', 'periodo_prueba')),
    stripe_subscription_id   VARCHAR(100),
    inicio_at                TIMESTAMPTZ,
    fin_at                   TIMESTAMPTZ,
    created_at               TIMESTAMPTZ DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_suscripciones_usuario ON suscripciones (usuario_id);

CREATE TABLE IF NOT EXISTS transacciones (
    id                    SERIAL PRIMARY KEY,
    comprador_id          INT NOT NULL REFERENCES usuarios_ext(id) ON DELETE CASCADE,
    creador_id            INT REFERENCES usuarios_ext(id) ON DELETE SET NULL,
    sample_id             INT REFERENCES samples(id) ON DELETE SET NULL,
    tipo                  VARCHAR(30) NOT NULL
                          CHECK (tipo IN ('suscripcion', 'compra_sample', 'payout', 'descarga')),
    monto                 DECIMAL(10, 2) NOT NULL,
    moneda                VARCHAR(3) NOT NULL DEFAULT 'USD',
    pago_creador          DECIMAL(10, 2) DEFAULT 0,
    comision_plataforma   DECIMAL(10, 2) DEFAULT 0,
    estado                VARCHAR(30) NOT NULL DEFAULT 'pendiente'
                          CHECK (estado IN ('completada', 'completed', 'pendiente', 'fallida', 'reembolsada')),
    stripe_payment_id     VARCHAR(100),
    idempotency_key       VARCHAR(100),
    tipo_descarga         VARCHAR(30),
    created_at            TIMESTAMPTZ DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_transacciones_comprador ON transacciones (comprador_id);
CREATE INDEX IF NOT EXISTS idx_transacciones_vendedor  ON transacciones (creador_id);
CREATE INDEX IF NOT EXISTS idx_transacciones_comprador_sample_opt
    ON transacciones (comprador_id, sample_id) WHERE tipo = 'compra_sample';
CREATE UNIQUE INDEX IF NOT EXISTS uq_stripe_payment_id
    ON transacciones (stripe_payment_id) WHERE stripe_payment_id IS NOT NULL;
CREATE UNIQUE INDEX IF NOT EXISTS uq_compra_sample_por_usuario
    ON transacciones (comprador_id, sample_id)
    WHERE tipo = 'compra_sample' AND estado IN ('completada', 'completed');
CREATE UNIQUE INDEX IF NOT EXISTS uq_transacciones_idempotency
    ON transacciones (idempotency_key) WHERE idempotency_key IS NOT NULL;

COMMIT;


