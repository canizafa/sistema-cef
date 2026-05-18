use crate::models::traits::Usuario;
use serde::Deserialize;

//La creación del alumno no tiene ID
#[derive(Deserialize, Debug)]
pub struct CrearAlumno {
    //1 alumno
    pub nombre: String,
    pub apellido: String,
    pub dni: String,
    pub mail: String,
    //planilla deberia ser otra entidad
}

// la respuesta del alumno tiene ID cuando viene de la base de datos
#[derive(Deserialize, Debug)]
pub struct Alumno {
    pub id: u64,
    pub nombre: String,
    pub apellido: String,
    pub dni: String,
    pub mail: String,
    //planilla deberia ser otra entidad
}

impl Usuario for CrearAlumno {}
