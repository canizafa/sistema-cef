-- Add migration script here
CREATE TABLE IF NOT EXISTS reserva (
    id_reserva TEXT PRIMARY KEY,
    estado VARCHAR(30) NOT NULL,
    tipo VARCHAR(50) NOT NULL,-->representa abono o membresia
    fecha_reserva TEXT not null,
    dni_cliente INTEGER not null, --en la tabla cliente se expresa como INTEGER
    id_clase TEXT NOT NULL,
    horario TEXT NOT NULL,
    FOREIGN KEY (dni_cliente) REFERENCES cliente(dni_cliente), -- reserva pertenece a un cliente
    FOREIGN KEY (id_clase) REFERENCES clase(id_clase) -- reserva pertenece a una clase
)
