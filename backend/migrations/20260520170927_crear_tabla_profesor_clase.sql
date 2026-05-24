CREATE TABLE IF NOT EXISTS profesor_clase (
    dni_profesor integer,
    id_clase TEXT,
    foreign key (dni_profesor) references profesor(dni_profesor),
    foreign key (id_clase) references clase(id_clase),
    primary key (dni_profesor, id_clase)
)
