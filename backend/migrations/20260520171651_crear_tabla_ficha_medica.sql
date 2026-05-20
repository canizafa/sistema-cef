-- Add migration script here
CREATE TABLE IF NOT EXISTS ficha_medica (
    id_ficha integer primary key autoincrement,
    enfermedades boolean not null,
    operaciones_quirurgicas boolean not null,
    detalles varchar(100)
)
