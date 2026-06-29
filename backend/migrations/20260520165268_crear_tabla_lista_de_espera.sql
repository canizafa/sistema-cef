CREATE TABLE IF NOT EXISTS lista_de_espera (
    id_espera TEXT PRIMARY KEY,
    tipo VARCHAR(50) NOT NULL,
    id_clase TEXT NOT NULL,
    FOREIGN KEY (id_clase) REFERENCES clase(id_clase),
    UNIQUE(id_clase, tipo)
)
