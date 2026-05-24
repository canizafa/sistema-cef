CREATE TABLE IF NOT EXISTS cliente (
    dni_cliente INTEGER primary key,
    nombre_completo VARCHAR(100) not null,
    email VARCHAR(100) unique,
    telefono VARCHAR(30),
    fecha_nacimiento DATE,
    estado VARCHAR(30),
    password VARCHAR(100) not null,
    id_ficha TEXT,
    FOREIGN KEY (id_ficha) REFERENCES ficha_medica(id_ficha)
)
