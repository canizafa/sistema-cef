use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

//aca arme dos structs porque uno representa a una clase cuando se crea
// y otro representa cuando un cliente quiere acceder al listado de clases
// es decir, un cliente no podria acceder a los id, al cupo profesor, al estado,etc
// deberia ir un nombre en la clase? ej: clase de yoga, funcional, otro
#[derive(Deserialize, Serialize)]
pub struct CrearClase {
    pub dia: String,
    pub horario: String,
    pub cupo_profe: i32,
    pub cupo_maximo: i32,
    pub estado: bool,
    pub id_actividad: i32,
    pub id_sala: i32,
    pub dni_profesor: i32,
}
#[derive(Serialize, FromRow)] //clase que puede ver el cliente fromrow transforma una tupla en struct para poder devolverlo
pub struct Clase {
    pub dia: String,
    pub horario: String,
    pub cupo_maximo: i32,
}
