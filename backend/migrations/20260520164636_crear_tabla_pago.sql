-- Add migration script here
CREATE TABLE IF NOT EXISTS pagos (
    id_pago INTEGER PRIMARY KEY AUTOINCREMENT,
    monto double NOT NULL monto > 0,
    fecha DATE NOT NULL,
    hora TIME NOT NULL,
    sena BOOLEAN,
    id_membresia INTEGER,
    reserva_paga INTEGER,
    FOREIGN KEY (id_membresia) REFERENCES membresia(id_membresia),
    FOREIGN KEY (reserva_paga) REFERENCES reserva(id_reserva)
)
