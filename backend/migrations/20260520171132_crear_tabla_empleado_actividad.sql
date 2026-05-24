CREATE TABLE IF NOT EXISTS empleado_actividad (
    dni_empleado integer,
    id_actividad TEXT,
    foreign key (dni_empleado) references empleado(dni_empleado),
    foreign key (id_actividad) references actividad(id_actividad),
    primary key (dni_empleado, id_actividad)
)
