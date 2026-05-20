-- Add migration script here
CREATE TABLE IF NOT EXISTS profesor (
    dni_profesor INTEGER PRIMARY KEY,
    nombre varchar(100) not null,
    genero varchar(30),
    estado varchar(30) not null,
)
