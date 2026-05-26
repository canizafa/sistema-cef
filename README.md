# Sistema CEF
Sistema para el Centro de actividades CEF (Grupo 4)

---

# Endpoints API

Base: `http://localhost:8081`

## Activos

### Health

| Método | Ruta |
|--------|------|
| GET | `/api/health` |

### Auth

| Método | Ruta |
|--------|------|
| POST | `/api/auth/login` |
| POST | `/api/auth/reset-password` |
| POST | `/api/auth/register-empleado` |
| POST | `/api/auth/register-cliente` |

### Clientes

| Método | Ruta |
|--------|------|
| POST | `/api/clientes/create` |
| GET | `/api/clientes/get-cliente/{id}` |
| POST | `/api/clientes/update-cliente/{id}` |
| POST | `/api/clientes/delete-cliente/{id}` |
| GET | `/api/clientes/get-all` |

### Empleados

| Método | Ruta |
|--------|------|
| GET | `/api/empleados/get-empleado/{id}` |
| POST | `/api/empleados/update-empleado/{id}` |
| POST | `/api/empleados/delete-empleado/{id}` |
| GET | `/api/empleados/get-all` |

### Profesores

| Método | Ruta |
|--------|------|
| POST | `/api/profesores/create-profesor` |
| GET | `/api/profesores/get-profesor/{dni}` |
| GET | `/api/profesores/get-all` |
| POST | `/api/profesores/update-profesor/{dni}` |
| DELETE | `/api/profesores/delete-profesor/{dni}` |

### Clases

| Método | Ruta |
|--------|------|
| POST | `/api/clase/create` |
| GET | `/api/clase/get-clase/{id}` |
| POST | `/api/clase/update-clase/{id}` |
| POST | `/api/clase/delete-clase/{id}` |
| GET | `/api/clase/get-all` |

### Reservas

| Método | Ruta |
|--------|------|
| POST | `/api/reservas/create` |
| GET | `/api/reservas/get-reserva/{id}` |
| PUT | `/api/reservas/update-reserva/{id}` |
| DELETE | `/api/reservas/delete-reserva/{id}` |
| GET | `/api/reservas/get-all` |

### Asistencias

| Método | Ruta |
|--------|------|
| POST | `/api/asistencia/create` |
| GET | `/api/asistencia/get-asistencia/{id}` |
| POST | `/api/asistencia/update-asistencia/{id}` |
| POST | `/api/asistencia/delete-asistencia/{id}` |
| GET | `/api/asistencia/get-all` |

### Membresías

| Método | Ruta |
|--------|------|
| POST | `/api/membresias/create` |
| GET | `/api/membresias/get-membresia-id/{id}` |
| GET | `/api/membresias/get-membresia-dni/{id}` |
| POST | `/api/membresias/update-membresia/{id}` |
| DELETE | `/api/membresias/delete-membresia/{id}` |
| GET | `/api/membresias/get-all` |

### Pagos

| Método | Ruta |
|--------|------|
| POST | `/api/pagos/create` |

---

### Actividades (`/api/actividades` sugerido)

| Método | Ruta |
|--------|------|
| POST | `/create` |
| GET | `/get-actividad/{id}` |
| GET | `/get-actividades` |
| PUT | `/update/{id}` |
| DELETE | `/delete/{id}` |

### Salas (`/api/salas` sugerido)

| Método | Ruta |
|--------|------|
| POST | `/` |
| GET | `/` |
| GET | `/{id}` |
| PUT | `/{id}` |
| DELETE | `/{id}` |


# Backend — Guía de instalación y ejecución

## Requisitos previos

### 1. Instalar Rust

Instalá Rust usando `rustup`, el instalador oficial:

**Linux / macOS:**
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

**Windows:**  
Descargá e instalá desde [https://rustup.rs](https://rustup.rs)

Verificá la instalación:
```bash
rustc --version
cargo --version
```

---

### 2. Instalar SQLx CLI

SQLx CLI es necesaria para manejar la base de datos y las migraciones:

```bash
cargo install sqlx-cli --no-default-features --features sqlite
```

Verificá la instalación:
```bash
sqlx --version
```

---

### 3. Configurar el archivo `.env`

Creá un archivo `.env` en la raíz del proyecto con el siguiente contenido:

```env
DATABASE_URL=sqlite://database.db
```

> El archivo `database.db` se crea automáticamente en el siguiente paso.

---

### 4. Crear la base de datos

```bash
sqlx database create
```

Esto genera el archivo `database.db` en la raíz del proyecto.

---

### 5. Correr el proyecto

```bash
cargo run
```

Las migraciones se ejecutan **automáticamente** al iniciar el servidor. No hace falta correrlas manualmente.

El servidor queda disponible en:
```
http://localhost:3000
```

---

## Estructura del proyecto

```
├── src/
│   ├── main.rs
│   └── routes/
├── migrations/       # Archivos SQL de migraciones
├── .env              # Variables de entorno (crear manualmente)
├── .env.example      # Ejemplo de variables de entorno
├── Cargo.toml
└── database.db       # Se genera automáticamente
```

---

## Comandos útiles

| Comando | Descripción |
|---|---|
| `cargo run` | Inicia el servidor |
| `cargo build --release` | Compila en modo producción |
| `sqlx migrate run` | Corre migraciones manualmente |
| `sqlx migrate revert` | Revierte la última migración |
| `sqlx migrate info` | Estado de las migraciones |
| `sqlx database drop` | Elimina la base de datos |
| `sqlx database reset` | Elimina, recrea y migra |

---

## Solución de problemas

**Error: `DATABASE_URL` no encontrada**  
Asegurate de haber creado el archivo `.env` en la raíz del proyecto.

**Error: `sqlx` no reconocido**  
Verificá que `~/.cargo/bin` esté en tu `PATH`. Podés agregarlo con:
```bash
export PATH="$HOME/.cargo/bin:$PATH"
```

**Error de compilación en macros de SQLx**  
SQLx valida las queries contra la base de datos en tiempo de compilación. Asegurate de que el archivo `database.db` exista y las migraciones hayan corrido antes de compilar, o usá:
```bash
cargo build --features sqlx/offline
```
