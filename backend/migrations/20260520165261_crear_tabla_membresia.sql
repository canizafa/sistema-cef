-- Add migration script here
CREATE TABLE IF NOT EXISTS membresias (
    id_membresia TEXT PRIMARY KEY,
    tipo TEXT NOT NULL,
    dni_cliente INTEGER NOT NULL,
    id_actividad TEXT NOT NULL,
    estado VARCHAR(30) NOT NULL,
    fecha_inicio TEXT NOT NULL,
    fecha_fin TEXT,
    FOREIGN KEY (dni_cliente) REFERENCES cliente(dni_cliente),
    FOREIGN KEY (id_actividad) REFERENCES actividad(id_actividad)
)
