/* [303A-1] Agregar meta_phone_number_id a integraciones_marketing.
 * La API de mensajes de Meta usa Phone-Number-ID (no WABA ID) para enviar mensajes.
 * POST /{Version}/{Phone-Number-ID}/messages */
ALTER TABLE integraciones_marketing
    ADD COLUMN meta_phone_number_id VARCHAR(100);
