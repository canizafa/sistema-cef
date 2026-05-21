-- Add migration script here
CREATE TABLE IF NOT EXISTS asistencia (
    id_asistencia INTEGER PRIMARY KEY AUTOINCREMENT,
    fecha date not null,
    metodo varchar(50),
    id_reserva integer,
    foreign key (id_reserva) references reserva(id_reserva)
)
