/* [024A-1] Vincular campañas con plantillas WhatsApp aprobadas por Meta.
 * Cuando una campaña incluye canal 'whatsapp', debe asociarse a una plantilla
 * aprobada para usar la Template Message API de Meta. */
ALTER TABLE campanas
    ADD COLUMN plantilla_whatsapp_id UUID REFERENCES plantillas_whatsapp(id) ON DELETE SET NULL;
