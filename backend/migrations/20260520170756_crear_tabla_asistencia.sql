-- Add migration script here
CREATE TABLE IF NOT EXISTS asistencia (
    id_asistencia TEXT PRIMARY KEY,
    fecha date not null,
    metodo varchar(50),
    id_reserva TEXT,
    foreign key (id_reserva) references reserva(id_reserva)
)
