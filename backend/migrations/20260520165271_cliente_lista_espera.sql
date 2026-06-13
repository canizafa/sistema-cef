-- Add migration script here
CREATE TABLE IF NOT EXISTS cliente_lista_espera (
    id_espera TEXT NOT NULL,
    dni_cliente INTEGER NOT NULL,
    fecha_ingreso TEXT NOT NULL,
    FOREIGN KEY (id_espera) REFERENCES lista_de_espera(id_espera),
    FOREIGN KEY (dni_cliente) REFERENCES cliente(dni_cliente),
    PRIMARY KEY (id_espera, dni_cliente)-->declaro como clave primaria esto que DEBE SER UNICO para que un cliente no se pueda ingresar dos veces
);
