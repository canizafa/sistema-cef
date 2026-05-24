use crate::errors::ApiError;
use axum::extract::State;
use sqlx::SqlitePool;

use crate::domain::Clase;

pub struct ClaseRepository;

impl ClaseRepository {
    /*el flujo sería:
    hacer el INSERT
    recuperar la fila insertada
    devolver Clase*/
    pub async fn create_clase(
        State(pool): State<SqlitePool>,
        clase: Clase,
    ) -> Result<Option<Clase>, ApiError> {
        sqlx::query("INSERT INTO clase ( dia, horario, cupo_profe, cupo_maximo, estado, descripcion, id_sala, dni_profesor)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?)",
            //saco el id clase
            clase.get_dia(),
            clase.get_horario(),
            clase.get_cupo_profe(),
            clase.get_cupo_maximo(),
            clase.get_estado(),//falta castear estado
            clase.get_descripcion(),
            clase.get_id_sala(),
            clase.get_dni_profesor())
             .execute(&pool)
             .await
             .map_err(|e| ApiError::DatabaseError(e))
             .map(|_| None)
    }
    pub async fn list_clases(
        State(pool): State<SqlitePool>,
    ) -> Result<Option<Vec<Clase>>, ApiError> {
        pub async fn list_clases(
            State(pool): State<SqlitePool>,
        ) -> Result<Option<Vec<Clase>>, ApiError> {
            sqlx::query_as!(Clase,
                    "SELECT id_clase, dia,horario,
                        cupo_profe,cupo_maximo,estado,
                        descripcion,id_sala,dni_profesor
                    FROM clase")
                .fetch_all(&pool)
                .await
                .map_err(|e| ApiError::DatabaseError(e))
                .map(Some)
    }
    pub async fn get_by_id(
        State(pool): State<SqlitePool>,
        id: i32,
    ) -> Result<Option<Clase>, ApiError> {
        sqlx::query_as!(Clase, "SELECT id_clase,dia,horario,
                    cupo_profe,cupo_maximo,
                    estado,descripcion,id_sala,
                    dni_profesor
                FROM clase
                WHERE id_clase = ?",
                id)
            .fetch_optional(&pool)
            .await
            .map_err(|e| ApiError::DatabaseError(e))
    }
    pub async fn update_clase(//editar clase
        State(pool): State<SqlitePool>,
        id: i32,
        clase: Clase,
    ) -> Result<Option<Clase>, ApiError> {// busca la clase cuyo id_clase(al final del where)
        sqlx::query_as!("UPDATE clase
               SET dia = ?, horario = ?, cupo_profe = ?, cupo_maximo = ?,
               estado = ?, descripcion = ?, id_sala = ?, dni_profesor = ?
               WHERE id_clase = ?",
               clase.get_dia(),
               clase.get_horario(),
               clase.get_cupo_profe(),
               clase.get_cupo_maximo(),
               clase.get_estado(),//falta castear
               clase.get_descripcion(),
               clase.get_id_sala(),
               clase.get_dni_profesor(),
               id)//sea igual al id recibido
           .execute(&pool)
           .await
           .map_err(ApiError::DatabaseError)?;

           let clase_actualizada= sqlx::query_as!(Clase,"SELECT id_clase,
                   dia,
                   horario,
                   cupo_profe,
                   cupo_maximo,
                   estado,
                   descripcion,
                   id_sala,
                   dni_profesor
               FROM clase
               WHERE id_clase = ?",
               id)
           .fetch_optional(&pool)
           .await
           .map_err(ApiError::DatabaseError)?;

           Ok(clase_actualizada)
    }
    pub async fn delete_clase(
        State(pool): State<SqlitePool>,
        id: i32,
    ) -> Result<Option<Clase>, ApiError> {
        sqlx::query_as!(Clase,"DELETE FROM clase
                WHERE id_clase = ?
                RETURNING
                    id_clase,
                    dia,
                    horario,
                    cupo_profe,
                    cupo_maximo,
                    estado,
                    descripcion,
                    id_sala,
                    dni_profesor",
                id
            )
            .fetch_optional(&pool)
            .await
            .map_err(|e| ApiError::DatabaseError(e))

    }
}
