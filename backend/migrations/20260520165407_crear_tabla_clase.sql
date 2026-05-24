-- Add migration script here
CREATE TABLE IF NOT EXISTS clase (
    id_clase TEXT PRIMARY KEY,
    dia text not null,
    horario TEXT not null,
    cupo_profe integer,
    cupo_maximo integer not null,
    estado varchar(30) not null,
    descripcion varchar(255),
    id_actividad TEXT not null,
    id_sala TEXT not null,
    dni_profesor integer,
    FOREIGN KEY (id_actividad) REFERENCES actividad(id_actividad),
    FOREIGN KEY (id_sala) REFERENCES sala(id_sala),
    foreign key (dni_profesor) REFERENCES profesor(dni_profesor)
)
