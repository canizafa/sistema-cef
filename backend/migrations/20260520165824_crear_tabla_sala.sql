-- Add migration script here
CREATE TABLE IF NOT EXISTS sala (
    id_sala INTEGER PRIMARY KEY AUTOINCREMENT,
    numero integer not null,
    capacidad_maxima not null
)
