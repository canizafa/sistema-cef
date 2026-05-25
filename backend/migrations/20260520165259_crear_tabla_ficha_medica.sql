-- Add migration script here
CREATE TABLE IF NOT EXISTS ficha_medica (
    id_ficha TEXT PRIMARY KEY,
    enfermedades BOOLEAN not null,
    operaciones_quirurgicas BOOLEAN not null,
    detalles VARCHAR(100)
)
