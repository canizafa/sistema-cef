-- Add migration script here
CREATE TABLE IF NOT EXISTS actividad (
    id_actividad TEXT PRIMARY KEY,
    nombre varchar(100) not null,
    descripcion text not null
)
