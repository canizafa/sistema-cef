-- Add migration script here
CREATE TABLE IF NOT EXISTS membresias (
    id_membresia INTEGER PRIMARY KEY AUTOINCREMENT,
    tipo VARCHAR(50) NOT NULL,
    estado VARCHAR(30) NOT NULL,
    fecha_inicio DATE NOT NULL,
    fecha_fin DATE NULL
)
