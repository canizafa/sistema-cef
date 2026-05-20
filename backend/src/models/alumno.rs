use crate::models::ficha_medica::FichaMedica;
use chrono::NaiveDate;
use serde::Deserialize;

//La creación del alumno no tiene ID
#[derive(Deserialize, Debug)]
pub struct CrearAlumno {
    //1 alumno
    pub nombre: String,
    pub apellido: String,
    pub mail: String,
    pub dni: String,
    pub fecha_nacimiento: NaiveDate,
    pub telefono: String,
    pub estado: String,
    pub ficha_medica: FichaMedica, //planilla deberia ser otra entidad
}

// la respuesta del alumno tiene ID cuando viene de la base de datos
#[derive(Deserialize, Debug)]
pub struct Alumno {
    pub id: i32,
    pub nombre: String,
    pub apellido: String,
    pub mail: String,
    pub dni: String,
    pub fecha_nacimiento: NaiveDate,
    pub telefono: String,
    pub estado: String,
    //planilla deberia ser otra entidad
}
