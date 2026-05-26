-- Add migration script here
CREATE TABLE IF NOT EXISTS reserva (
    id_reserva TEXT PRIMARY KEY,
    estado VARCHAR(30) NOT NULL,
    tipo VARCHAR(50) NOT NULL,
    fecha_reserva TEXT not null,
    dni_cliente TEXT not null,
    id_clase TEXT NOT NULL,
    FOREIGN KEY (id_clase) REFERENCES clase(id_clase)
)
