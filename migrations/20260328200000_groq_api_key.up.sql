/* [283A-8] Añade campo groq_api_key a configuracion_restaurante
 * para almacenar la API key de Groq usada en digitalización de documentos. */
ALTER TABLE configuracion_restaurante
    ADD COLUMN groq_api_key TEXT DEFAULT NULL;
