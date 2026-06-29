# Documentación de la API Backend

Este archivo describe las rutas disponibles, los métodos HTTP y los datos que el frontend debe enviar/recibir.

## Base URL

- Base URL: `http://<host>:<port>/api`
- Ejemplo local: `http://localhost:8000/api` (el puerto se configura en el backend)
- Content-Type: `application/json`
- Fechas deben enviarse en formato `YYYY-MM-DD`

## Headers recomendados

- `Content-Type: application/json`
- `Accept: application/json`

## Errores comunes

Las rutas pueden devolver errores con distintos códigos HTTP. El formato general es:

- Validación: `422 Unprocessable Entity`
  ```json
  {
    "error": "errores de validación",
    "details": [
      { "field": "campo", "message": "mensaje de error" }
    ]
  }
  ```
- Conflicto: `409 Conflict`
  ```json
  { "error": "Ya existe un registro con esos datos" }
  ```
- No encontrado: `404 Not Found`
  ```json
  { "error": "El recurso solicitado no existe" }
  ```
- No autorizado: `401 Unauthorized`
  ```json
  { "error": "Email o contraseña incorrectos" }
  ```
- Prohibido: `403 Forbidden`
  ```json
  { "error": "Acceso prohibido" }
  ```
- Error interno: `500 Internal Server Error`
  ```json
  { "error": "error interno del servidor" }
  ```
- Servicio no disponible: `503 Service Unavailable`
  ```json
  { "error": "servicio no disponible, intentá más tarde" }
  ```

Los errores de validación usan la estructura `FieldError` con campos:
- `field`: nombre del campo con el problema.
- `message`: descripción del error.

Ejemplo de lista de errores de validación:

```json
{
  "error": "errores de validación",
  "details": [
    { "field": "email", "message": "email es requerido" },
    { "field": "password", "message": "password es requerido" }
  ]
}
```

## Rutas generales

### Health check

- `GET /api/health`
- Respuesta: `200 OK`
- Uso: verificar que el backend está vivo.

## Autenticación

### POST /api/auth/login

- Body:
```json
{
  "email": "usuario@ejemplo.com",
  "password": "clave123"
}
```
- Respuesta:
```json
{
  "dni": 12345678,
  "email": "usuario@ejemplo.com",
  "access_token": "<jwt>",
  "rol": "cliente"
}
```
- Errores posibles:
  - `401 Unauthorized`
    ```json
    { "error": "Email o contraseña incorrectos" }
    ```
  - `500 Internal Server Error`
    ```json
    { "error": "Error de hash de contraseña" }
    ```

### POST /api/auth/reset-password

- Body:
```json
{
  "email": "usuario@ejemplo.com"
}
```
- Respuesta: `200 OK`
- Uso: enviar correo de restablecimiento de contraseña para cliente o empleado.
- Errores posibles:
  - `404 Not Found`
    ```json
    { "error": "Usuario no encontrado" }
    ```
  - `500 Internal Server Error`
    ```json
    { "error": "Error de hash de contraseña" }
    ```

### PUT /api/auth/change-password/{dni}

- Path param:
  - `dni`: DNI del usuario (cliente o empleado).
- Body:
```json
{
  "dni_cliente": 12345678,
  "old_password": "viejaClave",
  "new_password": "nuevaClave123"
}
```
- Respuesta: `200 OK`
- Errores posibles:
  - `404 Not Found`
    ```json
    { "error": "Usuario no encontrado" }
    ```
  - `401 Unauthorized`
    ```json
    { "error": "Clave actual incorrecta, intente nuevamente" }
    ```
  - `500 Internal Server Error`
    ```json
    { "error": "Error de hash de contraseña" }
    ```

## Clientes

### POST /api/clientes/create

- Body:
```json
{
  "dni": 12345678,
  "nombre_apellido": "Juan Pérez",
  "password": "clave123",
  "email": "juan@ejemplo.com",
  "telefono": "+541234567890",
  "fecha_nacimiento": "1990-01-01",
  "estado": "alta",
  "ficha_medica": {
    "enfermedades": false,
    "operaciones_quirurgicas": false,
    "detalle": "sin detalles"
  }
}
```
- Respuesta:
```json
{
  "dni": 12345678,
  "nombre_apellido": "Juan Pérez",
  "email": "juan@ejemplo.com",
  "telefono": "+541234567890",
  "fecha_nacimiento": "1990-01-01",
  "estado": "alta",
  "rol": "cliente",
  "motivo_eliminacion": null,
  "id_ficha": "..."
}
```
- Errores posibles:
  - `409 Conflict`
    ```json
    { "error": "El email ya está registrado" }
    ```
  - `422 Unprocessable Entity`
    ```json
    {
      "error": "errores de validación",
      "details": [
        { "field": "dni", "message": "DNI demasiado corto" },
        { "field": "password", "message": "Contraseña muy corta" },
        { "field": "nombre_apellido", "message": "Nombre no puede ser vacío" },
        { "field": "email", "message": "Email no puede ser vacío" },
        { "field": "telefono", "message": "Teléfono no puede ser vacío" },
        { "field": "fecha_nacimiento", "message": "Edad insuficiente" }
      ]
    }
    ```
  - `500 Internal Server Error`
    ```json
    { "error": "Error de hash de contraseña" }
    ```

### GET /api/clientes/get-cliente/{id}

- Path param:
  - `id`: DNI del cliente
- Respuesta: arreglo JSON con cliente y ficha médica:
```json
[
  { "dni": 12345678, "nombre_apellido": "Juan Pérez", ... },
  { "id_ficha": "...", "enfermedades": false, "operaciones_quirurgicas": false, "detalle": "sin detalles" }
]
```
- Errores posibles:
  - `404 Not Found`
    ```json
    { "error": "El recurso solicitado no existe" }
    ```
  - `500 Internal Server Error`
    ```json
    { "error": "error interno del servidor" }
    ```

### PUT /api/clientes/update-cliente

- Body:
```json
{
  "dni": 12345678,
  "nombre_apellido": "Juan Pérez",
  "email": "juan@ejemplo.com",
  "telefono": "+541234567890",
  "fecha_nacimiento": "1990-01-01",
  "motivo_eliminacion": null,
  "estado": "alta",
  "id_ficha": "uuid-ficha"
}
```
- Respuesta: objeto Cliente actualizado.
- Nota: la implementación actual solo actualiza `nombre_apellido`.

### PUT /api/clientes/update-password/

- Body:
```json
{
  "email": "juan@ejemplo.com",
  "old_password": "clave123",
  "new_password": "nuevo123"
}
```
- Respuesta: `200 OK`
- Errores posibles:
  - `404 Not Found`
    ```json
    { "error": "El recurso solicitado no existe" }
    ```
  - `401 Unauthorized`
    ```json
    { "error": "Clave actual incorrecta, intente nuevamente" }
    ```
  - `500 Internal Server Error`
    ```json
    { "error": "Error de hash de contraseña" }
    ```

### PUT /api/clientes/reset-password/{email}

- Path param:
  - `email`: email del cliente
- Respuesta: `200 OK`
- Errores posibles:
  - `404 Not Found`
    ```json
    { "error": "Usuario no encontrado" }
    ```
  - `500 Internal Server Error`
    ```json
    { "error": "error interno del servidor" }
    ```

### DELETE /api/clientes/delete-cliente

- Body:
```json
{
  "dni": 12345678,
  "estado": "baja",
  "motivo_eliminacion": "cliente eliminado"
}
```
- Respuesta: `200 OK`

### GET /api/clientes/get-all

- Respuesta: lista de clientes.

## Empleados

### POST /api/empleados/create

- Body:
```json
{
  "dni": 12345678,
  "nombre_apellido": "María Gómez",
  "password": "clave123",
  "mail": "maria@ejemplo.com",
  "genero": "femenino",
  "estado": "alta",
  "rol": "empleado"
}
```
- Respuesta: objeto Empleado.

### GET /api/empleados/get-empleado-by-email/{id}

- Path param: `id` es email del empleado
- Respuesta: objeto Empleado.

### GET /api/empleados/get-empleado-by-dni/{id}

- Path param: `id` es DNI del empleado
- Respuesta: objeto Empleado.

### PUT /api/empleados/update-empleado/{id}

- Path param: `id` es DNI del empleado
- Body:
```json
{
  "dni": 12345678,
  "nombre_apellido": "María Gómez",
  "mail": "maria@ejemplo.com",
  "genero": "femenino",
  "estado": "alta",
  "rol": "empleado"
}
```
- Respuesta: objeto Empleado actualizado.

### DELETE /api/empleados/delete-empleado/{id}

- Path param:
  - `id`: DNI del empleado
- Body:
```json
{
  "dni": 12345678,
  "estado": "baja",
  "motivo_eliminacion": "empleado eliminado"
}
```
- Respuesta: `200 OK`
- Nota: el path param `id` existe en la ruta, pero el handler consume el identificador del JSON.

### GET /api/empleados/get-all

- Respuesta: lista de empleados.

## Reservas

### POST /api/reservas/create

- Body:
```json
{
  "fecha": "2026-06-15",
  "tipo": "clase",
  "estado": "alta",
  "dni_cliente": 12345678,
  "id_clase": "uuid-clase"
}
```
- Respuesta: objeto Reserva.

### GET /api/reservas/get-reserva/{id}

- Path param: `id` es el ID de la reserva
- Respuesta: objeto Reserva.

### PUT /api/reservas/update-reserva/{id}

- Path param: `id` es el ID de la reserva
- Body: mismo esquema que POST `/create`
- Respuesta: objeto Reserva actualizado.

### DELETE /api/reservas/delete-reserva/{id}

- Path param: `id` es el ID de la reserva
- Respuesta: `200 OK`

### GET /api/reservas/get-all

- Respuesta: lista de reservas.

## Membresías

### POST /api/membresias/create

- Body:
```json
{
  "id_actividad": "uuid-actividad",
  "tipo": "premium",
  "dni_cliente": 12345678,
  "estado": "alta",
  "fecha_inicio": "2026-06-15",
  "fecha_fin": "2026-12-15"
}
```
- Respuesta: objeto Membresía.

### GET /api/membresias/get-membresia-dni/{id}

- Path param: `id` es DNI del cliente
- Respuesta: lista de Membresías.

### GET /api/membresias/get-membresia-id/{id}

- Path param: `id` es ID de la membresía
- Respuesta: objeto Membresía.

### PUT /api/membresias/update-membresia/{id}

- Path param: `id` es ID de la membresía
- Body: mismo esquema que POST `/create`
- Respuesta: objeto Membresía actualizado.

### DELETE /api/membresias/delete-membresia/{id}

- Path param: `id` es ID de la membresía
- Respuesta: `200 OK`

### GET /api/membresias/get-all

- Respuesta: lista de membresías.

## Pagos

### POST /api/pagos/create

- Body:
```json
{
  "titulo": "Pago mensual",
  "monto": 1500.0,
  "fecha": "2026-06-15",
  "hora": "10:30",
  "sena": true,
  "id_membresia": "uuid-membresia",
  "reserva_paga": "uuid-reserva"
}
```
- Respuesta:
```json
{
  "init_point": "https://...",
  "sandbox_init_point": "https://..."
}
```
- Errores posibles:
  - `422 Unprocessable Entity`
    ```json
    {
      "error": "errores de validación",
      "details": [
        { "field": "monto", "message": "El monto debe ser mayor a 0" }
      ]
    }
    ```
  - `500 Internal Server Error`
    ```json
    { "error": "error interno del servidor" }
    ```

## Clases

### POST /api/clase/create

- Body:
```json
{
  "dia": "2026-06-15",
  "horario": "10:00-11:00",
  "cupo_base": 15,
  "estado": "alta",
  "id_actividad": "uuid-actividad",
  "id_sala": "uuid-sala",
  "dni_profesor": 12345678,
  "descripcion": "Clase de yoga"
}
```
- Respuesta: objeto Clase.

### GET /api/clase/get-clase/{id}

- Path param: `id` es ID de la clase
- Respuesta: objeto Clase.

### PUT /api/clase/update-clase/{id}

- Path param: `id` es ID de la clase
- Body: mismo esquema que POST `/create`
- Respuesta: objeto Clase actualizado.

### DELETE /api/clase/delete-clase/{id}

- Path param: `id` es ID de la clase
- Respuesta: `200 OK`

### GET /api/clase/get-all

- Respuesta: lista de clases.

## Asistencias

### POST /api/asistencia/create

- Body:
```json
{
  "fecha": "2026-06-15",
  "metodo": "presencial",
  "id_reserva": "uuid-reserva"
}
```
- Respuesta: objeto Asistencia.

### GET /api/asistencia/get-asistencia/{id}

- Path param: `id` es ID de la asistencia
- Respuesta: objeto Asistencia.

### PUT /api/asistencia/update-asistencia/{id}

- Path param: `id` es ID de la asistencia
- Body: mismo esquema que POST `/create`
- Respuesta: objeto Asistencia actualizado.

### DELETE /api/asistencia/delete-asistencia/{id}

- Path param: `id` es ID de la asistencia
- Respuesta: `200 OK`

## Profesores

### POST /api/profesores/create-profesor

- Body:
```json
{
  "dni": 12345678,
  "nombre_completo": "Lucía Díaz",
  "genero": "femenino",
  "estado": "alta"
}
```
- Respuesta: objeto Profesor.

### GET /api/profesores/get-profesor/{dni}

- Path param: `dni` es DNI del profesor
- Respuesta: objeto Profesor.

### PUT /api/profesores/update-profesor/{dni}

- Path param: `dni` es DNI del profesor
- Body: mismo esquema que POST `/create-profesor`
- Respuesta: objeto Profesor actualizado.

### DELETE /api/profesores/delete-profesor/{dni}

- Path param: `dni` es DNI del profesor
- Respuesta: `200 OK`

### GET /api/profesores/get-all

- Respuesta: lista de profesores.

## Salas

### POST /api/salas/create/

- Body:
```json
{
  "numero": 1,
  "capacidad_maxima": 30
}
```
- Respuesta: objeto Sala.

### GET /api/salas/get-all/

- Respuesta: lista de salas.

### GET /api/salas/get-sala/{id}

- Path param: `id` es ID de la sala
- Respuesta: objeto Sala.

## Actividades

### POST /api/actividades/create

- Body:
```json
{
  "nombre": "Yoga",
  "descripcion": "Clase de yoga suave"
}
```
- Respuesta: objeto Actividad.
- Status: `201 Created`

### GET /api/actividades/get-actividad/{id}

- Path param: `id` es ID de la actividad
- Respuesta: objeto Actividad.

### GET /api/actividades/get-actividades

- Respuesta: lista de actividades.

### PUT /api/actividades/update/{id}

- Path param: `id` es ID de la actividad
- Body: mismo esquema que POST `/create`
- Respuesta: objeto Actividad actualizado.

### DELETE /api/actividades/delete/{id}

- Path param: `id` es ID de la actividad
- Respuesta: `200 OK`

## Notificaciones

### POST /api/notificaciones/

- Body:
```json
{
  "mail": "usuario@ejemplo.com",
  "motivo": "Recordatorio",
  "cuerpo": "Este es el cuerpo del correo"
}
```
- Respuesta: `200 OK`

## Notas importantes

- Los valores permitidos para `estado` son: `alta`, `baja`, `sin_cupo`.
- Los valores permitidos para `rol` son: `profesor`, `empleado`, `cliente`, `duenio`.
- Los valores permitidos para `genero` son: `masculino`, `femenino`, `otro`.
- Si un endpoint usa `id` en la ruta, revise si es un DNI (`i64`) o un UUID/ID de texto.
- La funcionalidad de `lista_espera` no está implementada en las rutas actualmente.
