-- Add migration script here
CREATE TABLE IF NOT EXISTS clase (
    id_clase TEXT PRIMARY KEY,
    dia TEXT not null,
    horario TEXT not null,
    cupo_maximo INTEGER,
    cupo_base INTEGER not null,
    estado VARCHAR(30) not null,
    descripcion VARCHAR(255),
    id_actividad TEXT not null,
    id_sala TEXT not null,
    dni_profesor INTEGER,
    FOREIGN KEY (id_actividad) REFERENCES actividad(id_actividad),
    FOREIGN KEY (id_sala) REFERENCES sala(id_sala),
    FOREIGN KEY (dni_profesor) REFERENCES profesor(dni_profesor)
)
