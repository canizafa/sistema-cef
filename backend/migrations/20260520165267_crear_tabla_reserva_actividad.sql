CREATE TABLE IF NOT EXISTS reserva_actividad (
    id_reserva TEXT,
    id_actividad TEXT,
    FOREIGN KEY (id_reserva) REFERENCES reserva(id_reserva),
    FOREIGN KEY (id_actividad) REFERENCES actividad(id_actividad),
    PRIMARY KEY (id_reserva, id_actividad)
)
