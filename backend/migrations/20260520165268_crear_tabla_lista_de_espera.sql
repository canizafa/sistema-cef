CREATE TABLE IF NOT EXISTS lista_de_espera (
    id_espera TEXT PRIMARY KEY,
    tipo VARCHAR(50),
    fecha_ingreso TEXT not null,
    id_clase TEXT,
    FOREIGN KEY (id_clase) REFERENCES clase(id_clase)
)
