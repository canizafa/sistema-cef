-- Add migration script here
CREATE TABLE IF NOT EXISTS lista_de_espera (
    id_espera INTEGER PRIMARY KEY AUTOINCREMENT,
    tipo varchar(50)
    fecha_ingreso date not null,
    id_clase integer,
    -- clientes_en_espera: como se implementa la lista en sql??????
    foreign key (id_clase) references clase(id_clase)
)
