use chrono::NaiveDate;

pub struct Register {
    pub dni: i32,
    pub nombre_apellido: String,
    pub email: String,
    pub telefono: String,
    pub fecha_nacimiento: NaiveDate,
}
