use crate::errors::ApiError;
use axum::extract::State;
use sqlx::SqlitePool;

use crate::domain::Clase;

pub struct ClaseRepository;

impl ClaseRepository {
    /* el flujo sería:
       hacer el INSERT
       recuperar la fila insertada
       devolver Clase
    */
    pub async fn create_clase(
        State(pool): State<SqlitePool>,
        clase: Clase,
    ) -> Result<Option<Clase>, ApiError> {
<<<<<<< HEAD
        sqlx::query_as!(Clase,
            "INSERT INTO clase (dia, horario, cupo_profe, cupo_maximo, estado, descripcion, id_sala, dni_profesor)
                VALUES (?, ?, ?, ?, ?, ?, ?, ?)
                RETURNING id_clase, dia, horario, cupo_profe, cupo_maximo, estado, descripcion, id_sala, dni_profesor",
=======
        sqlx::query_as!(
            Clase,
            "INSERT INTO clase
            (dia, horario, cupo_profe, cupo_maximo, estado, descripcion, id_sala, dni_profesor)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?)
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
>>>>>>> 0070171 (se arreglaron las funciones)
            clase.get_dia(),
            clase.get_horario(),
            clase.get_cupo_profe(),
            clase.get_cupo_maximo(),
<<<<<<< HEAD
            clase.get_estado(),
            clase.get_descripcion(),
            clase.get_id_sala(),
            clase.get_dni_profesor())
            .fetch_one(&pool)
            .await
            .map_err(|e| ApiError::DatabaseError(e))
=======
            clase.get_estado() as _,
            clase.get_descripcion(),
            clase.get_id_sala(),
            clase.get_dni_profesor()
        )
        .fetch_optional(&pool)
        .await
        .map_err(ApiError::DatabaseError)
>>>>>>> 0070171 (se arreglaron las funciones)
    }

    pub async fn list_clases(
        State(pool): State<SqlitePool>,
    ) -> Result<Option<Vec<Clase>>, ApiError> {
        sqlx::query_as!(
            Clase,
<<<<<<< HEAD
            "SELECT id_clase, dia,horario,
                cupo_profe,cupo_maximo,estado,
                descripcion,id_sala,dni_profesor
=======
            "SELECT
                id_clase,
                dia,
                horario,
                cupo_profe,
                cupo_maximo,
                estado,
                descripcion,
                id_sala,
                dni_profesor
>>>>>>> 0070171 (se arreglaron las funciones)
            FROM clase"
        )
        .fetch_all(&pool)
        .await
<<<<<<< HEAD
        .map_err(|e| ApiError::DatabaseError(e))
=======
        .map_err(ApiError::DatabaseError)
>>>>>>> 0070171 (se arreglaron las funciones)
        .map(Some)
    }

    pub async fn get_by_id(
        State(pool): State<SqlitePool>,
        id: i32,
    ) -> Result<Option<Clase>, ApiError> {
        sqlx::query_as!(
            Clase,
<<<<<<< HEAD
            "SELECT id_clase,dia,horario,
                cupo_profe,cupo_maximo,
                estado,descripcion,id_sala,
=======
            "SELECT
                id_clase,
                dia,
                horario,
                cupo_profe,
                cupo_maximo,
                estado,
                descripcion,
                id_sala,
>>>>>>> 0070171 (se arreglaron las funciones)
                dni_profesor
            FROM clase
            WHERE id_clase = ?",
            id
        )
        .fetch_optional(&pool)
        .await
<<<<<<< HEAD
        .map_err(|e| ApiError::DatabaseError(e))
    }
    pub async fn update_clase(
        //editar clase
=======
        .map_err(ApiError::DatabaseError)
    }

    pub async fn update_clase(
>>>>>>> 0070171 (se arreglaron las funciones)
        State(pool): State<SqlitePool>,
        id: i32,
        clase: Clase,
    ) -> Result<Option<Clase>, ApiError> {
        sqlx::query_as!(
<<<<<<< HEAD
            "UPDATE clase
                SET dia = ?, horario = ?, cupo_profe = ?, cupo_maximo = ?,
                estado = ?, descripcion = ?, id_sala = ?, dni_profesor = ?
                WHERE id_clase = ?",
            clase.get_dia(),
            clase.get_horario(),
            clase.get_cupo_profe(),
            clase.get_cupo_maximo(),
            clase.get_estado(), //falta castear
            clase.get_descripcion(),
            clase.get_id_sala(),
            clase.get_dni_profesor(),
            id
        )
        .await
        .map_err(ApiError::DatabaseError)?;

        let clase_actualizada = sqlx::query_as!(
            Clase,
            "SELECT id_clase,
=======
            Clase,
            "UPDATE clase
            SET
                dia = ?,
                horario = ?,
                cupo_profe = ?,
                cupo_maximo = ?,
                estado = ?,
                descripcion = ?,
                id_sala = ?,
                dni_profesor = ?
            WHERE id_clase = ?
            RETURNING
                id_clase,
>>>>>>> 0070171 (se arreglaron las funciones)
                dia,
                horario,
                cupo_profe,
                cupo_maximo,
                estado,
                descripcion,
                id_sala,
<<<<<<< HEAD
                dni_profesor
            FROM clase
            WHERE id_clase = ?",
=======
                dni_profesor",
            clase.get_dia(),
            clase.get_horario(),
            clase.get_cupo_profe(),
            clase.get_cupo_maximo(),
            clase.get_estado() as _,
            clase.get_descripcion(),
            clase.get_id_sala(),
            clase.get_dni_profesor(),
>>>>>>> 0070171 (se arreglaron las funciones)
            id
        )
        .fetch_optional(&pool)
        .await
<<<<<<< HEAD
        .map_err(ApiError::DatabaseError)?;

        Ok(clase_actualizada)
=======
        .map_err(ApiError::DatabaseError)
>>>>>>> 0070171 (se arreglaron las funciones)
    }

    pub async fn delete_clase(
        State(pool): State<SqlitePool>,
        id: i32,
    ) -> Result<Option<Clase>, ApiError> {
        sqlx::query_as!(
            Clase,
            "DELETE FROM clase
<<<<<<< HEAD
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
=======
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
>>>>>>> 0070171 (se arreglaron las funciones)
            id
        )
        .fetch_optional(&pool)
        .await
<<<<<<< HEAD
        .map_err(|e| ApiError::DatabaseError(e))
=======
        .map_err(ApiError::DatabaseError)
>>>>>>> 0070171 (se arreglaron las funciones)
    }
}
