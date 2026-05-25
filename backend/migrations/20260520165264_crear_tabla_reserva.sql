-- Add migration script here
CREATE TABLE IF NOT EXISTS reserva (
    id_reserva TEXT PRIMARY KEY,
    estado VARCHAR(30),
    tipo VARCHAR(50),
    fecha_reserva TEXT not null,
    id_clase TEXT,
    FOREIGN KEY (id_clase) REFERENCES clase(id_clase)
)
