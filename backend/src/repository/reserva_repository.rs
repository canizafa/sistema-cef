use axum::extract::State;
use sqlx::SqlitePool;

pub struct ReservaRepository;

impl ReservaRepository {
    pub async fn create_reserva(State(pool): State<SqlitePool>) {}
    pub async fn list_reservas(State(pool): State<SqlitePool>) {}
    pub async fn get_reserva(State(pool): State<SqlitePool>, id: i32) {}
    pub async fn update_reserva(State(pool): State<SqlitePool>, id: i32) {}
    pub async fn delete_reserva(State(pool): State<SqlitePool>, id: i32) {}
}
// pub async fn crear_reserva(
//     State(pool): State<SqlitePool>,
//     Json(reserva): Json<CrearReserva>,
// ) -> String {
//     sqlx::query(
//         "INSERT INTO Reserva
//         (fecha, estado, dniCliente, idClase)
//         VALUES (?,?, ?, ?, ?)",
//     )
//     .bind(&reserva.fecha)
//     .bind(&reserva.estado)
//     .bind(reserva.dni_cliente)
//     .bind(reserva.id_clase)
//     .execute(&pool)
//     .await
//     .unwrap();
//     "reserva creada".to_string()
// }
