-- Add migration script here
CREATE TABLE IF NOT EXISTS sala (
    id_sala TEXT PRIMARY KEY,
    numero integer not null,
    capacidad_maxima not null
)
