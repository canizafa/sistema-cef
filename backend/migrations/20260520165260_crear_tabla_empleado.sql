-- Add migration script here
CREATE TABLE IF NOT EXISTS empleado (
    dni_empleado INTEGER PRIMARY KEY,
    nombre_apellido VARCHAR(100) NOT NULL,
    mail VARCHAR(100) NOT NULL UNIQUE,
    password VARCHAR(255) NOT NULL DEFAULT '123456',
    genero VARCHAR(30) NOT NULL,
    estado VARCHAR(30) NOT NULL,
    motivo_eliminacion VARCHAR(255),
    rol VARCHAR(30) NOT NULL
);
