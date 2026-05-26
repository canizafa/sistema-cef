CREATE TABLE IF NOT EXISTS profesor_clase (
    dni_profesor INTEGER not null,
    id_clase TEXT not null,
    FOREIGN KEY (dni_profesor) REFERENCES profesor(dni_profesor),
    FOREIGN KEY (id_clase) REFERENCES clase(id_clase),
    PRIMARY KEY (dni_profesor, id_clase)
)
