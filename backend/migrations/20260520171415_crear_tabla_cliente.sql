CREATE TABLE IF NOT EXISTS cliente (
    dni_cliente integer primary key,
    nombre_completo varchar(100) not null,
    email varchar(100) unique,
    telefono varchar(30),
    fecha_nacimiento date,
    estado varchar(30),
    password varchar(100) not null,
    id_ficha integer,
    foreign key (id_ficha) references ficha_medica(id_ficha)
)
