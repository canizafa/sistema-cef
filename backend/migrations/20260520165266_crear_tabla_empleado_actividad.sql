CREATE TABLE IF NOT EXISTS empleado_actividad (
    dni_empleado INTEGER not null,
    id_actividad TEXT not null,
    FOREIGN KEY (dni_empleado) REFERENCES empleado(dni_empleado),
    FOREIGN KEY (id_actividad) REFERENCES actividad(id_actividad),
    PRIMARY KEY (dni_empleado, id_actividad)
)
