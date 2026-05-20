CREATE TABLE IF NOT EXISTS empleado_actividad (
    dni_empleado integer,
    id_actividad integer,
    foreign key (dni_empleado) references profesor(dni_empleado),
    foreign key (id_actividad) references clase(id_actividad),
    primary key (dni_empleado, id_actividad)
)
