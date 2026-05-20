CREATE TABLE IF NOT EXISTS reserva_actividad (
    id_reserva integer,
    id_actividad integer,
    foreign key (id_reserva) references profesor(id_reserva),
    foreign key (id_actividad) references clase(id_actividad),
    primary key (id_reserva, id_actividad)
)
