# Documentación de la API Backend

Este archivo describe las rutas disponibles, los métodos HTTP y los datos que el frontend debe enviar/recibir.

## Base URL

- Base URL: `http://<host>:<port>/api`
- Ejemplo local: `http://localhost:8000/api`
- Content-Type: `application/json`
- Fechas deben enviarse en formato `YYYY-MM-DD`

## Headers recomendados

- `Content-Type: application/json`
- `Accept: application/json`

## Formato de errores

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

## Rutas generales

### Health check

- `GET /api/health`
- Respuesta: `200 OK`
- Uso: verificar que el backend está activo.

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
  "rol": "cliente",
  "estado": "alta"
}
```

### POST /api/auth/reset-password

- Body:
```json
{
  "email": "usuario@ejemplo.com"
}
```
- Respuesta: `200 OK`
- Uso: solicitar envío de correo de restablecimiento.

### PUT /api/auth/change-password/{dni}

- Path param:
  - `dni`: DNI del usuario.
- Body:
```json
{
  "dni_cliente": 12345678,
  "old_password": "viejaClave",
  "new_password": "nuevaClave123"
}
```
- Respuesta: `200 OK`

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
  "id_ficha": "<uuid>",
  "creditos": 0,
  "fecha_notificacion": null
}
```

### GET /api/clientes/get-cliente/{id}

- Path param:
  - `id`: DNI del cliente.
- Respuesta:
```json
[
  {
    "dni": 12345678,
    "nombre_apellido": "Juan Pérez",
    "email": "juan@ejemplo.com",
    "telefono": "+541234567890",
    "fecha_nacimiento": "1990-01-01",
    "estado": "alta",
    "rol": "cliente",
    "motivo_eliminacion": null,
    "id_ficha": "<uuid>",
    "creditos": 0,
    "fecha_notificacion": null
  },
  {
    "id_ficha": "<uuid>",
    "enfermedades": false,
    "operaciones_quirurgicas": false,
    "detalle": "sin detalles"
  }
]
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
  "id_ficha": "<uuid>"
}
```
- Respuesta: `ClienteResponse` actualizado.

### PUT /api/clientes/update-password/

- Body:
```json
{
  "email": "juan@ejemplo.com",
  "old_password": "clave123",
  "new_password": "nuevaClave123"
}
```
- Respuesta: `200 OK`

### PUT /api/clientes/reset-password/{email}

- Path param:
  - `email`: email del cliente.
- Respuesta: `200 OK`

### DELETE /api/clientes/delete-cliente

- Body:
```json
{
  "dni": 12345678,
  "estado": "eliminado",
  "motivo_eliminacion": "motivo"
}
```
- Respuesta: `200 OK`

### GET /api/clientes/get-all

- Respuesta: arreglo de `ClienteResponse`.

## Empleados

### POST /api/empleados/create

- Body:
```json
{
  "dni": 87654321,
  "nombre_apellido": "María López",
  "password": "clave123",
  "mail": "maria@ejemplo.com",
  "genero": "femenino",
  "estado": "alta",
  "rol": "empleado"
}
```
- Respuesta: `EmpleadoResponse`.

### GET /api/empleados/get-empleado-by-email/{id}

- Path param:
  - `id`: email del empleado.
- Respuesta: `EmpleadoResponse`.

### GET /api/empleados/get-empleado-by-dni/{id}

- Path param:
  - `id`: DNI del empleado.
- Respuesta: `EmpleadoResponse`.

### PUT /api/empleados/update-empleado/{id}

- Path param:
  - `id`: DNI del empleado.
- Body:
```json
{
  "dni": 87654321,
  "nombre_apellido": "María López",
  "mail": "maria@ejemplo.com",
  "genero": "femenino",
  "estado": "alta",
  "rol": "empleado",
  "motivo_eliminacion": null
}
```
- Respuesta: `EmpleadoResponse` actualizado.

### DELETE /api/empleados/delete-empleado/{id}

- Path param:
  - `id`: DNI del empleado.
- Body:
```json
{
  "dni": 87654321,
  "estado": "eliminado",
  "motivo_eliminacion": "motivo"
}
```
- Respuesta: `200 OK`

### GET /api/empleados/get-all

- Respuesta: arreglo de `EmpleadoResponse`.

## Profesores

### POST /api/profesores/create-profesor

- Body:
```json
{
  "dni": 11223344,
  "nombre_completo": "Carlos Díaz",
  "genero": "masculino",
  "estado": "alta"
}
```
- Respuesta: `ProfesorResponse`.

### GET /api/profesores/get-profesor/{dni}

- Path param:
  - `dni`: DNI del profesor.
- Respuesta: `ProfesorResponse`.

### GET /api/profesores/get-all

- Respuesta: arreglo de `ProfesorResponse`.

### PUT /api/profesores/update-profesor/{dni}

- Path param:
  - `dni`: DNI del profesor.
- Body: mismo esquema que `CreateProfesorRequest`.
- Respuesta: `ProfesorResponse` actualizado.

### DELETE /api/profesores/delete-profesor/{dni}

- Path param:
  - `dni`: DNI del profesor.
- Body:
```json
{
  "profesor_dni": 11223344,
  "estado": "eliminado",
  "motivo_eliminacion": "motivo"
}
```
- Respuesta: `{}`

## Salas

### POST /api/salas/create/

- Body:
```json
{
  "numero": 1,
  "capacidad_maxima": 30
}
```
- Respuesta: `SalaResponse`.

### GET /api/salas/get-all/

- Respuesta: arreglo de `SalaResponse`.

### GET /api/salas/get-sala/{id}

- Path param:
  - `id`: ID de la sala.
- Respuesta: `SalaResponse`.

## Actividades

### POST /api/actividades/create

- Body:
```json
{
  "nombre": "Pilates",
  "descripcion": "Entrenamiento de cuerpo completo"
}
```
- Respuesta: `ActividadResponse`.

### GET /api/actividades/get-actividad/{id}

- Path param:
  - `id`: ID de la actividad.
- Respuesta: `ActividadResponse`.

### GET /api/actividades/get-actividades

- Respuesta: arreglo de `ActividadResponse`.

### DELETE /api/actividades/delete/{id}

- Path param:
  - `id`: ID de la actividad.
- Respuesta: `200 OK`

### PUT /api/actividades/update/{id}

- Path param:
  - `id`: ID de la actividad.
- Body: mismo esquema que `CreateActividadRequest`.
- Respuesta: `ActividadResponse` actualizado.

## Clases

### POST /api/clase/create

- Body:
```json
{
  "dia": "2026-07-01",
  "horario": "18:00",
  "cupo_base": 20,
  "estado": "alta",
  "id_actividad": "<uuid-actividad>",
  "id_sala": "<uuid-sala>",
  "dni_profesor": 11223344,
  "descripcion": "Clase de pilates"
}
```
- Respuesta: `ClaseResponse`.

### GET /api/clase/get-clase/{id}

- Path param:
  - `id`: ID de la clase.
- Respuesta: `ClaseResponse`.

### PUT /api/clase/update-clase/{id}

- Path param:
  - `id`: ID de la clase.
- Body:
```json
{
  "id_clase": "<uuid-clase>",
  "dni_profesor": 11223344,
  "estado": "sin_cupo"
}
```
- Respuesta: `ClaseResponse` actualizado.

### DELETE /api/clase/delete-clase/{id}

- Path param:
  - `id`: ID de la clase.
- Respuesta: `200 OK`

### GET /api/clase/get-all

- Respuesta: arreglo de `ClaseResponse`.

## Reservas

### POST /api/reservas/create

- Body:
```json
{
  "fecha": "2026-07-10",
  "tipo": "presencial",
  "estado": "confirmada",
  "dni_cliente": 12345678,
  "id_clase": "<uuid-clase>"
}
```
- Respuesta: `ReservaResponse`.

### GET /api/reservas/get-reserva/{id}

- Path param:
  - `id`: ID de la reserva.
- Respuesta: `ReservaResponse`.

### PUT /api/reservas/update-reserva/{id}

- Path param:
  - `id`: ID de la reserva.
- Body: mismo esquema que `CreateReservaRequest`.
- Respuesta: `ReservaResponse` actualizado.

### DELETE /api/reservas/delete-reserva/{id}

- Path param:
  - `id`: ID de la reserva.
- Respuesta: `200 OK`

### GET /api/reservas/get-all

- Respuesta: arreglo de `ReservaResponse`.

## Asistencia

### POST /api/asistencia/create

- Body:
```json
{
  "fecha": "2026-07-10",
  "metodo": "presencial",
  "id_reserva": "<uuid-reserva>"
}
```
- Respuesta: `AsistenciaResponse`.

### GET /api/asistencia/get-asistencia/{id}

- Path param:
  - `id`: ID de la asistencia.
- Respuesta: `AsistenciaResponse`.

### PUT /api/asistencia/update-asistencia/{id}

- Path param:
  - `id`: ID de la asistencia.
- Body: mismo esquema que `CreateAsistenciaRequest`.
- Respuesta: `AsistenciaResponse` actualizado.

### DELETE /api/asistencia/delete-asistencia/{id}

- Path param:
  - `id`: ID de la asistencia.
- Respuesta: `200 OK`

### GET /api/asistencia/get-asistencia-by-reserva/{id_reserva}

- Path param:
  - `id_reserva`: ID de la reserva.
- Respuesta: `AsistenciaResponse`.

## Membresías

### POST /api/membresias/create

- Body:
```json
{
  "id_actividad": "<uuid-actividad>",
  "horario": "18:00",
  "tipo": "mensual",
  "dni_cliente": 12345678,
  "estado": "activo",
  "fecha_inicio": "2026-07-01",
  "fecha_fin": "2026-07-31",
  "aceptar_espera": true
}
```
- Respuesta: `MembresiaResponse`.

### GET /api/membresias/get-membresia-dni/{id}

- Path param:
  - `id`: DNI del cliente.
- Respuesta: arreglo de `MembresiaResponse`.

### GET /api/membresias/get-membresia-id/{id}

- Path param:
  - `id`: ID de la membresía.
- Respuesta: `MembresiaResponse`.

### PUT /api/membresias/update-membresia/{id}

- Path param:
  - `id`: ID de la membresía.
- Body: mismo esquema que `CreateMembresiaRequest`.
- Respuesta: `MembresiaResponse` actualizado.

### DELETE /api/membresias/delete-membresia/{id}

- Path param:
  - `id`: ID de la membresía.
- Respuesta: `200 OK`

### GET /api/membresias/get-all

- Respuesta: arreglo de `MembresiaResponse`.

## Pagos

### POST /api/pagos/create

- Body:
```json
{
  "titulo": "Pago de membresía",
  "monto": 5000.0,
  "fecha": "2026-07-01",
  "hora": "14:00",
  "sena": false,
  "id_membresia": "<uuid-membresia>",
  "reserva_paga": "<uuid-reserva>"
}
```
- Respuesta:
```json
{
  "init_point": "https://www.mercadopago.com/checkout/v1/redirect?pref_id=...",
  "sandbox_init_point": "https://sandbox.mercadopago.com/checkout/v1/redirect?pref_id=..."
}
```

## Notificaciones

### POST /api/notificaciones/notify

- Body:
```json
{
  "email": "cliente@ejemplo.com",
  "motivo": "Recordatorio de clase",
  "cuerpo": "Tu clase empieza mañana a las 18:00"
}
```
- Respuesta: `200 OK`

### PUT /api/notificaciones/update-date

- Body:
```json
{
  "email": "cliente@ejemplo.com",
  "fecha": "2026-07-02"
}
```
- Respuesta: `200 OK`

## Lista de espera

### POST /api/lista-espera/create

- Body:
```json
{
  "id_clase": "<uuid-clase>",
  "tipo": "general"
}
```
- Respuesta: `ListaEsperaResponse`.

### DELETE /api/lista-espera/delete/{id}

- Path param:
  - `id`: ID de la lista de espera.
- Respuesta: `200 OK`

### GET /api/lista-espera/get-by-id/{id}

- Path param:
  - `id`: ID de la lista de espera.
- Respuesta: `ListaEsperaResponse`.

### GET /api/lista-espera/get-all

- Respuesta: arreglo de `ListaEsperaResponse`.

### POST /api/lista-espera/insert-user/{id_clase}

- Path param:
  - `id_clase`: ID de la clase.
- Body:
```json
{
  "id_espera": "<uuid-espera>",
  "dni_cliente": 12345678,
  "fecha_ingreso": "2026-07-01"
}
```
- Respuesta: `200 OK`

### GET /api/lista-espera/get-clientes/{id_espera}

- Path param:
  - `id_espera`: ID de la lista de espera.
- Respuesta: arreglo de `ClienteListaEsperaResponse`.

### GET /api/lista-espera/get-next/{id_espera}

- Path param:
  - `id_espera`: ID de la lista de espera.
- Respuesta: `ClienteListaEsperaResponse`.

### POST /api/lista-espera/delete

- Body:
```json
{
  "id_espera": "<uuid-espera>",
  "dni_cliente": 12345678
}
```
- Respuesta: `200 OK`

## Estadísticas

### GET /api/estadisticas/get-clase-mas-concurrida

- Query params:
  - `fecha_desde`: `YYYY-MM-DD`
  - `fecha_hasta`: `YYYY-MM-DD`
- Respuesta:
```json
{
  "id_clase": "<uuid-clase>",
  "descripcion": "Pilates",
  "cantidad": 25
}
```

### GET /api/estadisticas/get-clase-mas-cancelada

- Query params:
  - `fecha_desde`: `YYYY-MM-DD`
  - `fecha_hasta`: `YYYY-MM-DD`
- Respuesta:
```json
{
  "id_clase": "<uuid-clase>",
  "descripcion": "Yoga",
  "cantidad": 3
}
```

### GET /api/estadisticas/get-recaudacion

- Query params:
  - `fecha_desde`: `YYYY-MM-DD`
  - `fecha_hasta`: `YYYY-MM-DD`
- Respuesta:
```json
{
  "total": 123456.78
}
```

## Valores permitidos

- `EstadoUsuario`: `alta`, `baja`, `eliminado`
- `RolUsuario`: `empleado`, `cliente`, `duenio`
- `GeneroUsuario`: `masculino`, `femenino`, `otro`
- `EstadoClase`: `alta`, `sin_cupo`, `extendido`
- `EstadoMembresia`: `activo`, `cancelado`
