CREATE TABLE IF NOT EXISTS reserva_actividad (
    id_reserva TEXT,
    id_actividad TEXT,
    foreign key (id_reserva) references reserva(id_reserva),
    foreign key (id_actividad) references actividad(id_actividad),
    primary key (id_reserva, id_actividad)
)
