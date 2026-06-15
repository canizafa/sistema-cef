-- Add migration script here
CREATE TABLE IF NOT EXISTS profesor (
    dni_profesor INTEGER PRIMARY KEY,
    nombre VARCHAR(100) not null,
    genero VARCHAR(30) not null,
    estado VARCHAR(30) not null,
    motivo_eliminacion VARCHAR(255)
)
