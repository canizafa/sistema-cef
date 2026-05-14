use serde::Deserialize;

#[derive(Deserialize)]
pub struct Alumno {
    //1 alumno
    pub nombre: String,
    pub apellido: String,
    pub dni: String,
    pub mail: String,
    //planilla deberia ser otra entidad
}
