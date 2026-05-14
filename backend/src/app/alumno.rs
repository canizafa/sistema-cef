use serde::Deserialize;

#[derive(Deserialize)]
pub struct CreateAlumno {
    //1 alumno
    pub nombre: String,
    pub apellido: String,
    pub dni: String,
    pub mail: String,
    //planilla deberia ser otra entidad
}

#[derive(Deserialize)]
pub struct Alumno {
    pub id: u64,
    pub nombre: String,
    pub apellido: String,
    pub dni: String,
    pub mail: String,
    //planilla deberia ser otra entidad
}
