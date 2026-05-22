use axum::extract::State;
use sqlx::SqlitePool;

use crate::domain::Empleado;
use crate::errors::ApiError;

pub struct EmpleadoRepository;

impl EmpleadoRepository {
    pub async fn create_empleado(
        State(pool): State<SqlitePool>,
        empleado: Empleado,
    ) -> Result<Option<Empleado>, ApiError> {
        todo!()
    }
    pub async fn list_empleados(
        State(pool): State<SqlitePool>,
    ) -> Result<Option<Vec<Empleado>>, ApiError> {
        todo!()
    }
    pub async fn get_by_email(
        State(pool): State<SqlitePool>,
        email: String,
    ) -> Result<Option<Empleado>, ApiError> {
        todo!()
    }
    pub async fn get_by_dni(
        State(pool): State<SqlitePool>,
        dni: i32,
    ) -> Result<Option<Empleado>, ApiError> {
        todo!()
    }
    pub async fn update_empleado(
        State(pool): State<SqlitePool>,
        dni: i32,
        empleado: Empleado,
    ) -> Result<Option<Empleado>, ApiError> {
        todo!()
    }
    pub async fn delete_empleado(
        State(pool): State<SqlitePool>,
        dni: i32,
    ) -> Result<Option<Empleado>, ApiError> {
        todo!()
    }
}
