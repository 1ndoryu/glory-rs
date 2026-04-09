/* [094A-6] Campos para botones CTA en mensajes WhatsApp.
 * telefono_restaurante: número que usa el botón "Llámanos".
 * url_reservas: URL que usa el botón "Reserva ya". */
ALTER TABLE configuracion_restaurante
    ADD COLUMN telefono_restaurante VARCHAR(30) NOT NULL DEFAULT '',
    ADD COLUMN url_reservas VARCHAR(500) NOT NULL DEFAULT '';
