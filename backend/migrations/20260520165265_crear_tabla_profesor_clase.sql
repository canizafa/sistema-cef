CREATE TABLE IF NOT EXISTS profesor_clase (
    dni_profesor INTEGER,
    id_clase TEXT,
    FOREIGN KEY (dni_profesor) REFERENCES profesor(dni_profesor),
    FOREIGN KEY (id_clase) REFERENCES clase(id_clase),
    PRIMARY KEY (dni_profesor, id_clase)
)
