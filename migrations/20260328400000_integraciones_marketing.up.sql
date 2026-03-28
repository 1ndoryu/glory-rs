/* [283A-23] Tabla para almacenar credentials de integraciones de marketing por usuario.
 * Cada restaurante/usuario tiene sus propias credenciales de terceros. */
CREATE TABLE integraciones_marketing (
    id UUID PRIMARY KEY,
    user_id UUID NOT NULL UNIQUE REFERENCES users(id) ON DELETE CASCADE,

    /* SMTP (para email de campañas y recordatorios) */
    smtp_host VARCHAR(255),
    smtp_port INTEGER,
    smtp_user VARCHAR(255),
    smtp_password VARCHAR(255),
    smtp_from_email VARCHAR(255),
    smtp_from_name VARCHAR(100),

    /* Twilio (para SMS) */
    twilio_account_sid VARCHAR(100),
    twilio_auth_token VARCHAR(100),
    twilio_from_number VARCHAR(20),

    /* Meta WhatsApp Business API */
    meta_waba_id VARCHAR(100),
    meta_business_app_id VARCHAR(100),
    meta_access_token TEXT,

    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now()
);
