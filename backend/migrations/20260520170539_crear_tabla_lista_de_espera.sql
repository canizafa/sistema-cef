-- Add migration script here
CREATE TABLE IF NOT EXISTS lista_de_espera (
    id_espera TEXT PRIMARY KEY,
    tipo varchar(50),
    fecha_ingreso date not null,
    id_clase TEXT,
    -- CREAR UNA NUEVA TABLA ASI
    -- CREATE TABLE lista_espera_cliente (
      --  id_espera TEXT,
--        dni_cliente TEXT,
  --      posicion INTEGER,

    --    PRIMARY KEY (id_espera, dni_cliente),

      --  FOREIGN KEY (id_espera) REFERENCES lista_de_espera(id_espera),
        --FOREIGN KEY (dni_cliente) REFERENCES cliente(dni_cliente)
    --);
    foreign key (id_clase) references clase(id_clase)
)
