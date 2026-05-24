-- Add migration script here
CREATE TABLE IF NOT EXISTS pagos (
    id_pago TEXT PRIMARY KEY,
    monto REAL NOT NULL CHECK(monto > 0),
    fecha TEXT NOT NULL,
    hora TEXT NOT NULL,
    sena BOOLEAN,
    id_membresia TEXT,
    reserva_paga TEXT,
    FOREIGN KEY (id_membresia) REFERENCES membresias(id_membresia),
    FOREIGN KEY (reserva_paga) REFERENCES reserva(id_reserva)
)
