-- Add migration script here
CREATE TABLE IF NOT EXISTS membresias (
    id_membresia TEXT PRIMARY KEY,
    tipo VARCHAR(50) NOT NULL,
    estado VARCHAR(30) NOT NULL,
    fecha_inicio TEXT NOT NULL,
    fecha_fin TEXT NULL
)
