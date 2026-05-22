use axum::extract::State;
use chrono::NaiveDate;
use sqlx::SqlitePool;

pub struct ListaDeEsperaRepository;

impl ListaDeEsperaRepository {
    pub async fn delete_lista_espera(State(pool): State<SqlitePool>, id_espera: i32) {}
    pub async fn add_lista_espera(
        State(pool): State<SqlitePool>,
        id_espera: i32,
        dni_cliente: i32,
        id_clase: i32,
        fecha: NaiveDate,
    ) {
    }
    pub async fn get_lista_espera(State(pool): State<SqlitePool>) {}
}
