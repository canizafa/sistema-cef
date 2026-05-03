# 📁 Guía del Repositorio — Sistema CEF

Bienvenidos al equipo. Este documento explica los pasos que **cada integrante debe seguir** para configurar el entorno de trabajo correctamente.

---

## 1. Clonar el repositorio

Abrí una terminal en la carpeta donde quieras tener el proyecto y ejecutá:

```bash
git clone https://github.com/canizafa/sistema-cef.git
cd sistema-cef
```

Al clonar, el repositorio queda automáticamente conectado a GitHub. Podés verificarlo con:

```bash
git remote -v
```

Deberías ver algo así:

```
origin  https://github.com/canizafa/sistema-cef.git (fetch)
origin  https://github.com/canizafa/sistema-cef.git (push)
```

---

## 2. Generar tu Token de GitHub

> ⚠️ **GitHub ya no acepta contraseñas** para operaciones Git. Necesitás un token de acceso personal.

Seguí estos pasos:

1. Entrá a [github.com](https://github.com) con tu cuenta
2. Click en tu **foto de perfil** → **Settings**
3. En el menú izquierdo, bajá hasta **Developer settings**
4. Entrá a **Personal access tokens → Tokens (classic)**
5. Click en **Generate new token (classic)**
6. Poné un nombre descriptivo, por ejemplo: `token-sistema-cef`
7. En permisos, tildá **`repo`** (el primero de la lista)
8. Click en **Generate token**
9. Copiá el token que aparece (empieza con `ghp_...`)

> 🔴 **MUY IMPORTANTE: El token se muestra UNA SOLA VEZ. Una vez que cerrás o recargás la página, no podés volver a verlo.**
> Guardalo en un lugar seguro antes de continuar (bloc de notas, gestor de contraseñas, etc.).

Cuando Git te pida credenciales al hacer `push` o `pull`, usás:

```
Username: tu-usuario-de-github
Password: ghp_xxxxxxxxxxxxxx   ← acá va el token, no tu contraseña
```

### Guardar el token para no ingresarlo cada vez

```bash
git config --global credential.helper store
```

La primera vez que lo ingreses, Git lo guarda y no te lo vuelve a pedir.

---

## 3. Crear tu rama personal

Cada integrante trabaja en **su propia rama**. Esto evita que los cambios de uno pisen los del otro.

Primero, traete todas las ramas remotas para verlas:

```bash
git fetch --all
git branch -r
```

Deberías ver algo así:

```
origin/main
origin/develop
origin/backend
origin/frontend
```

Luego, posicionarte en la rama que corresponde a tu área (**backend** o **frontend**):

```bash
git checkout backend
# o
git checkout frontend
```

Y desde ahí, creá tu rama personal:

```bash
git checkout -b nombre-apellido
```

Por ejemplo, si sos del equipo de backend:

```bash
git checkout backend
git checkout -b juan-garcia
```

Subí tu rama a GitHub para que quede registrada:

```bash
git push origin nombre-apellido
```

> ℹ️ Al crear tu rama desde `backend` o `frontend`, tu rama personal ya parte con la estructura inicial del proyecto. Cuando hagas un Pull Request, lo vas a hacer hacia la misma rama de donde partiste.

---

## 4. Estructura del repositorio

```
sistema-cef/
├── backend/        ← proyecto Rust (Actix-web / Axum)
│   ├── src/
│   │   └── main.rs
│   └── Cargo.toml
├── frontend/       ← proyecto React (Vite)
│   ├── src/
│   └── package.json
├── .gitignore
└── README.md
```

---

## 5. Flujo de trabajo diario

Antes de ponerte a trabajar, **siempre** traete los últimos cambios:

```bash
git pull origin backend   # si trabajás en backend
# o
git pull origin frontend  # si trabajás en frontend
```

Cuando termines tus cambios, los guardás y subís **a tu rama personal**:

```bash
# 1. Agregar los archivos modificados
git add .

# 2. Hacer el commit con un mensaje descriptivo
git commit -m "Agrego endpoint de login"

# 3. Subir a tu rama personal (nunca directo a backend/frontend/main)
git push origin nombre-apellido
```

> ℹ️ El `push` **siempre va a tu rama personal**. Para que tus cambios lleguen a `backend` o `frontend`, el siguiente paso es abrir un Pull Request.

---

## 6. Integrar tus cambios mediante un Pull Request

Un **Pull Request (PR)** es la forma de proponer que tus cambios se integren a una rama compartida. Permite que el resto del equipo los revise antes de aceptarlos.

### Pasos para abrir un PR:

1. Entrás al repositorio en [github.com/canizafa/sistema-cef](https://github.com/canizafa/sistema-cef)
2. GitHub suele mostrar un banner amarillo que dice **"Compare & pull request"** si subiste cambios recientemente → click ahí. Si no aparece, andá a **Pull requests → New pull request**
3. Configurás las ramas:
   - **base**: `backend` o `frontend` (a dónde querés que lleguen tus cambios)
   - **compare**: tu rama personal (`nombre-apellido`)
4. Ponés un título claro, por ejemplo: `"Agrego endpoint de login"`
5. Opcionalmente describís qué hiciste en el campo de descripción
6. Click en **Create pull request**
7. Avisás al grupo por WhatsApp para que alguien lo revise y haga el merge

### ¿Qué pasa después?

Un compañero revisa el PR. Si está todo bien, hace click en **Merge pull request** y tus cambios quedan integrados en `backend` o `frontend`.

El flujo completo es:

```
tu-rama  →  (Pull Request)  →  backend o frontend  →  (Pull Request)  →  main
```

---

## 7. Resumen de ramas

| Rama | Propósito |
|---|---|
| `main` | Código estable, no se toca directo |
| `backend` | Integración del equipo de backend |
| `frontend` | Integración del equipo de frontend |
| `nombre-apellido` | Tu rama personal de trabajo |

---

## ⚠️ Reglas del equipo

- **Nunca hacer push directo a `main`**
- Siempre hacer `git pull` antes de empezar a trabajar
- Mensajes de commit claros y en español: `"Agrego modelo de usuario"` ✅ — `"cambios"` ❌
- Si tenés conflictos, no los resuelvas solo: avisá al grupo

---

*Cualquier duda, consultá en el grupo de WhatsApp o abrí un Issue en el repositorio.*
