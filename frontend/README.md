# Sistema CEF — Frontend

Frontend del sistema de gestión del Centro de Educación Física.
Desarrollado con React + TypeScript + Vite.

## Requisitos

- Node.js 18+
- npm

## Instalación

```bash
cd frontend
npm install
```

## Configuración

Crear el archivo `.env.local` en la carpeta `frontend/`:

```
VITE_API_URL=http://localhost:8081
```

## Comandos

```bash
# Levantar servidor de desarrollo
npm run dev

# Correr tests
npm run test

# Build de producción
npm run build
```

## Estructura

```
src/
├── components/     # Componentes reutilizables
├── context/        # Estado global (AuthContext)
├── pages/          # Páginas de la app
└── services/       # Comunicación con el backend
```

## Usuario de prueba

- **Email:** admin@cef.com
- **Password:** admin123
- **Rol:** dueño
