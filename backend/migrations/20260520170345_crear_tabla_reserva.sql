-- Add migration script here
CREATE TABLE IF NOT EXISTS reserva (
    id_reserva TEXT PRIMARY KEY,
    estado varchar(30),
    tipo varchar(50),
    fecha_reserva TEXT not null,
    id_clase TEXT,
    foreign key (id_clase) references clase(id_clase)
)
