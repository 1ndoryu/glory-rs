/* [263A-24] Tabla de plantillas WhatsApp para Meta Business API.
 * Estado fluye: borrador → enviada (a Meta) → aprobada/rechazada.
 * Archivos adjuntos almacenan URL (upload a S3/local previo al envío).
 * Gotcha: Meta limita categorías (MARKETING, UTILITY, AUTHENTICATION). */

CREATE TABLE IF NOT EXISTS plantillas_whatsapp (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id),

    /* Datos de la plantilla */
    nombre VARCHAR(255) NOT NULL,
    categoria VARCHAR(50) NOT NULL DEFAULT 'MARKETING',
    idioma VARCHAR(10) NOT NULL DEFAULT 'es',
    cuerpo_mensaje TEXT NOT NULL DEFAULT '',
    cabecera_texto VARCHAR(255),
    pie_texto VARCHAR(60),

    /* Archivos adjuntos (URLs) */
    cabecera_media_url TEXT,
    cabecera_media_tipo VARCHAR(20),

    /* Estado de aprobación por Meta */
    estado VARCHAR(20) NOT NULL DEFAULT 'borrador'
        CHECK (estado IN ('borrador', 'enviada', 'aprobada', 'rechazada')),
    meta_template_id VARCHAR(255),
    meta_razon_rechazo TEXT,
    meta_enviada_at TIMESTAMPTZ,
    meta_respondida_at TIMESTAMPTZ,

    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_plantillas_wa_user ON plantillas_whatsapp(user_id);
CREATE INDEX idx_plantillas_wa_estado ON plantillas_whatsapp(estado);
