-- Add migration script here
CREATE TABLE IF NOT EXISTS reserva (
    id_reserva INTEGER PRIMARY KEY AUTOINCREMENT,
    estado varchar(30),
    tipo varchar(50),
    fecha_reserva date not null,
    id_clase integer,
    foreign key (id_clase) references clase(id_clase)
)
