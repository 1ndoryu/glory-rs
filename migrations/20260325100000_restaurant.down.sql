/* Revertir migración de restaurante */
DROP TABLE IF EXISTS reservas;
DROP TABLE IF EXISTS gastos;
DROP TABLE IF EXISTS ventas;
DROP TABLE IF EXISTS categorias_gasto;
ALTER TABLE users DROP COLUMN IF EXISTS nombre;

/* Restaurar tabla de notas del template */
CREATE TABLE notes (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    title VARCHAR(255) NOT NULL,
    content TEXT NOT NULL DEFAULT '',
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
CREATE INDEX idx_notes_user_id ON notes(user_id);
