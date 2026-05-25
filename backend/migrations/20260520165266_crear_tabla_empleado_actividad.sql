CREATE TABLE IF NOT EXISTS empleado_actividad (
    dni_empleado INTEGER,
    id_actividad TEXT,
    FOREIGN KEY (dni_empleado) REFERENCES empleado(dni_empleado),
    FOREIGN KEY (id_actividad) REFERENCES actividad(id_actividad),
    PRIMARY KEY (dni_empleado, id_actividad)
)
