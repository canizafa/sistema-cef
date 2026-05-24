use sqlx::SqlitePool;

use crate::domain::Empleado;
use crate::errors::ApiError;

pub struct EmpleadoRepository;

impl EmpleadoRepository {
    pub async fn create_empleado(
        pool: &SqlitePool,
        empleado: Empleado,
    ) -> Result<Option<Empleado>, ApiError> {
        todo!()
    }
    pub async fn list_empleados(pool: &SqlitePool) -> Result<Option<Vec<Empleado>>, ApiError> {
        todo!()
    }
    pub async fn get_by_email(
        pool: &SqlitePool,
        email: String,
    ) -> Result<Option<Empleado>, ApiError> {
        todo!()
    }
    pub async fn get_by_dni(pool: &SqlitePool, dni: i32) -> Result<Option<Empleado>, ApiError> {
        todo!()
    }
    pub async fn update_empleado(
        pool: &SqlitePool,
        dni: i32,
        empleado: Empleado,
    ) -> Result<Option<Empleado>, ApiError> {
        todo!()
    }
    pub async fn delete_empleado(
        pool: &SqlitePool,
        dni: i32,
    ) -> Result<Option<Empleado>, ApiError> {
        todo!()
    }
}
