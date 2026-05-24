-- Add migration script here
CREATE TABLE IF NOT EXISTS clase (
    id_clase TEXT PRIMARY KEY,
    dia varchar(20) not null,
    horario TEXT not null,
    cupo_profe integer,
    cupo_maximo integer not null,
    estado varchar(30),
    descripcion varchar(255),
    id_actividad TEXT,
    id_sala TEXT,
    dni_profesor integer,
    FOREIGN KEY (id_actividad) REFERENCES actividad(id_actividad),
    FOREIGN KEY (id_sala) REFERENCES sala(id_sala),
    foreign key (dni_profesor) REFERENCES profesor(dni_profesor)
)
