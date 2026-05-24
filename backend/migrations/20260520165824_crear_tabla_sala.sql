-- Add migration script here
CREATE TABLE IF NOT EXISTS sala (
    id_sala TEXT PRIMARY KEY,
    numero INTEGER not null,
    capacidad_maxima INTEGER not null
)
