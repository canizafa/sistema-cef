CREATE TABLE IF NOT EXISTS cliente (
    dni_cliente INTEGER primary key,
    nombre_completo VARCHAR(100) not null,
    email VARCHAR(100) unique not null,
    telefono VARCHAR(30) not null,
    fecha_nacimiento TEXT not null,
    estado VARCHAR(30) not null,
    password VARCHAR(100) not null,
    id_ficha TEXT not null,
    FOREIGN KEY (id_ficha) REFERENCES ficha_medica(id_ficha)
)
