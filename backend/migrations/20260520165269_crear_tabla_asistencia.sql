-- Add migration script here
CREATE TABLE IF NOT EXISTS asistencia (
    id_asistencia TEXT PRIMARY KEY,
    fecha DATE not null,
    metodo VARCHAR(50),
    id_reserva TEXT,
    FOREIGN KEY (id_reserva) REFERENCES reserva(id_reserva)
)
