-- Add migration script here
CREATE TABLE IF NOT EXISTS clase (
    id_clase INTEGER PRIMARY KEY AUTOINCREMENT,
    dia varchar(20) not null,
    horario time not null,
    cupo_profe integer,
    cupo_maximo integer not null,
    estado varchar(30),
    id_actividad integer,
    id_sala integer,
    dni_profesor integer,
    FOREIGN KEY (id_actividad) REFERENCES actividad(id_actividad),
    FOREIGN KEY (id_sala) REFERENCES reserva(id_sala),
    foreign key (dni_profesor) REFERENCES profesor(dni_profesor)
)
