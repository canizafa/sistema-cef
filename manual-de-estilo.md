# 📋 Manual de Estilo — Sistema de Gestión de Turnos

> Guía de referencia para el equipo de frontend. Mantenerla actualizada es responsabilidad de todos.
> Stack: **React + Tailwind CSS + shadcn/ui**

---

## 1. Colores

Definimos los colores como variables CSS en `globals.css` y como extensión de Tailwind en `tailwind.config.js`.

### Paleta principal

| Nombre         | Token               | HEX       | Uso principal                                  |
|----------------|---------------------|-----------|------------------------------------------------|
| `primary`      | `bg-primary`        | `#1A1A1A` | Texto, headers, bordes principales             |
| `brand`        | `bg-brand`          | `#163F7A` | Botón primario, links, badges activos          |
| `destructive`  | `bg-destructive`    | `#D01F25` | Botón eliminar, alertas críticas, errores      |
| `background`   | `bg-background`     | `#FFFFFF` | Fondo general de la app                        |
| `surface`      | `bg-surface`        | `#F5F5F5` | Fondo de cards, sidebars, modales              |
| `border`       | `border-border`     | `#E0E0E0` | Bordes de inputs, cards, separadores           |
| `muted`        | `text-muted`        | `#6B7280` | Texto secundario, placeholders, labels         |

### Estados de feedback

| Nombre     | HEX       | Uso                        |
|------------|-----------|----------------------------|
| `success`  | `#16A34A` | Turno confirmado, éxito    |
| `warning`  | `#D97706` | Turno próximo, advertencia |
| `info`     | `#0284C7` | Información general        |
| `error`    | `#D01F25` | (mismo que destructive)    |

### Configuración en `tailwind.config.js`

```js
// tailwind.config.js
module.exports = {
  theme: {
    extend: {
      colors: {
        primary:     '#1A1A1A',
        brand:       '#163F7A',
        destructive: '#D01F25',
        surface:     '#F5F5F5',
        border:      '#E0E0E0',
        muted:       '#6B7280',
        success:     '#16A34A',
        warning:     '#D97706',
        info:        '#0284C7',
      },
    },
  },
}
```

---

## 2. Tipografía

Fuente: **Roboto** (via Google Fonts). Importar en `index.html` o `globals.css`.

```css
@import url('https://fonts.googleapis.com/css2?family=Roboto:wght@400;500;700&display=swap');

body {
  font-family: 'Roboto', sans-serif;
}
```

### Escala tipográfica

| Rol               | Clase Tailwind          | Tamaño | Peso     | Uso                              |
|-------------------|-------------------------|--------|----------|----------------------------------|
| Título de página  | `text-2xl font-bold`    | 24px   | 700      | Encabezado principal de vista    |
| Subtítulo         | `text-xl font-semibold` | 20px   | 600      | Sección o card destacada         |
| Texto normal      | `text-base`             | 16px   | 400      | Contenido general                |
| Texto pequeño     | `text-sm`               | 14px   | 400      | Metadatos, fechas, captions      |
| Label formulario  | `text-sm font-medium`   | 14px   | 500      | Etiqueta de campos               |
| Caption / ayuda   | `text-xs text-muted`    | 12px   | 400      | Texto de ayuda bajo inputs       |

> **Regla:** No usar tamaños fuera de esta escala salvo justificación acordada en equipo.

---

## 3. Espaciado

Usamos la escala base de **8px** de Tailwind. Sólo usar estos valores para márgenes y paddings:

| Token Tailwind | Valor |
|----------------|-------|
| `p-1` / `m-1` | 4px   |
| `p-2` / `m-2` | 8px   |
| `p-3` / `m-3` | 12px  |
| `p-4` / `m-4` | 16px  |
| `p-6` / `m-6` | 24px  |
| `p-8` / `m-8` | 32px  |
| `p-12`         | 48px  |

> **Regla:** El espaciado interno de los componentes (padding) usa `p-3` o `p-4`. El espaciado entre secciones usa `gap-6` o `gap-8`.

---

## 4. Componentes

Todos los componentes vienen de **shadcn/ui**. Instalación: `npx shadcn-ui@latest add [componente]`.

### 4.1 Botones

Usamos 3 variantes. No inventar variantes nuevas sin acordarlo.

| Variante      | Clase / Props shadcn         | Cuándo usarlo                          |
|---------------|------------------------------|----------------------------------------|
| **Primary**   | `variant="default"`          | Acción principal (Guardar, Confirmar)  |
| **Secondary** | `variant="outline"`          | Acción secundaria (Cancelar, Volver)   |
| **Danger**    | `variant="destructive"`      | Eliminar, cancelar turno               |

```tsx
// Ejemplos de uso
<Button variant="default">Confirmar turno</Button>
<Button variant="outline">Cancelar</Button>
<Button variant="destructive">Eliminar turno</Button>

// Con ícono (siempre a la izquierda del texto)
<Button variant="default">
  <PlusIcon className="w-4 h-4 mr-2" />
  Nuevo turno
</Button>
```

> **Tamaño por defecto:** `h-10 px-4 py-2` (ya viene en shadcn).
> **Tamaño pequeño** (tablas, listas): `size="sm"` → `h-8 px-3`.

---

### 4.2 Inputs y Formularios

```tsx
// Input estándar
<div className="space-y-1">
  <Label htmlFor="nombre">Nombre del paciente</Label>
  <Input id="nombre" placeholder="Ej: Juan Pérez" />
</div>

// Input con error
<div className="space-y-1">
  <Label htmlFor="fecha">Fecha</Label>
  <Input id="fecha" className="border-destructive" />
  <p className="text-xs text-destructive">La fecha es obligatoria</p>
</div>

// Select
<Select>
  <SelectTrigger>
    <SelectValue placeholder="Seleccionar profesional" />
  </SelectTrigger>
  <SelectContent>
    <SelectItem value="dr-garcia">Dr. García</SelectItem>
  </SelectContent>
</Select>
```

> **Regla:** Siempre usar `<Label>` asociado al input. Nunca usar placeholder como único indicador del campo.

---

### 4.3 Cards

Usadas para mostrar un turno, un profesional, o un resumen.

```tsx
<Card>
  <CardHeader>
    <CardTitle>Turno #1042</CardTitle>
    <CardDescription>Martes 14 de mayo · 10:30 hs</CardDescription>
  </CardHeader>
  <CardContent>
    <p className="text-sm">Dr. García — Cardiología</p>
  </CardContent>
  <CardFooter className="gap-2">
    <Button variant="default" size="sm">Confirmar</Button>
    <Button variant="outline" size="sm">Reprogramar</Button>
  </CardFooter>
</Card>
```

---

### 4.4 Alertas / Toasts

Para feedback al usuario usamos `toast` de shadcn (sonner).

```tsx
// Éxito
toast.success("Turno confirmado correctamente")

// Error
toast.error("No se pudo guardar el turno")

// Advertencia (usar description)
toast("Turno próximo a vencer", {
  description: "El turno vence en 30 minutos",
})
```

Para alertas inline (no toast) usamos el componente `<Alert>`:

```tsx
<Alert variant="destructive">
  <AlertTitle>Error</AlertTitle>
  <AlertDescription>No hay disponibilidad para esa fecha.</AlertDescription>
</Alert>
```

---

### 4.5 Modales / Dialogs

```tsx
<Dialog>
  <DialogTrigger asChild>
    <Button variant="destructive">Cancelar turno</Button>
  </DialogTrigger>
  <DialogContent className="max-w-md">
    <DialogHeader>
      <DialogTitle>¿Cancelar turno?</DialogTitle>
      <DialogDescription>
        Esta acción no se puede deshacer. El turno quedará disponible para otros pacientes.
      </DialogDescription>
    </DialogHeader>
    <DialogFooter>
      <Button variant="outline">Volver</Button>
      <Button variant="destructive">Sí, cancelar</Button>
    </DialogFooter>
  </DialogContent>
</Dialog>
```

> **Ancho máximo:** `max-w-md` (448px) para confirmaciones. `max-w-lg` para formularios.
> **Regla:** El botón destructivo/de confirmación siempre va a la derecha en el footer.

---

### 4.6 Badges / Estados de turno

```tsx
// Turno confirmado
<Badge className="bg-success text-white">Confirmado</Badge>

// Turno pendiente
<Badge variant="outline">Pendiente</Badge>

// Turno cancelado
<Badge className="bg-destructive text-white">Cancelado</Badge>

// Turno próximo
<Badge className="bg-warning text-white">Próximo</Badge>
```

---

## 5. Íconos

Librería: **Lucide React** (ya incluida con shadcn/ui).

```bash
npm install lucide-react
```

```tsx
import { CalendarDays, Clock, User, Trash2, Plus } from 'lucide-react'

// Tamaño estándar en texto: w-4 h-4
// Tamaño en encabezados o destacados: w-5 h-5
<CalendarDays className="w-4 h-4" />
```

> **Regla:** No mezclar con otras librerías de íconos (FontAwesome, Material Icons, etc.).

---

## 6. Responsive / Breakpoints

Usamos los breakpoints estándar de Tailwind. Para este proyecto, los relevantes son:

| Nombre   | Breakpoint | Descripción              |
|----------|------------|--------------------------|
| mobile   | `< 768px`  | Celular (default, sin prefijo) |
| desktop  | `md:` y +  | Tablet y computadora     |

```tsx
// Ejemplo: columnas
<div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
  ...
</div>

// Ejemplo: ocultar en mobile
<aside className="hidden md:block">...</aside>

// Ejemplo: padding adaptable
<main className="p-4 md:p-8">...</main>
```

> **Regla:** Diseñar primero para mobile (sin prefijo) y luego agregar overrides con `md:`. No usar `sm:` salvo casos muy específicos.

---

## 7. Estructura de carpetas sugerida (Frontend)

```
src/
├── components/
│   ├── ui/          ← Componentes de shadcn (no tocar)
│   ├── layout/      ← AdminLayout, Header, Footer     
│   └── shared/      ← Componentes propios reutilizables
├── pages/
│   ├── admin/       ← Pantallas de recepcionista/dueño 
│   ├── cliente/     ← Pantallas del usuario cliente     
│   └── public/      ← Pantallas sin autenticación       
├── context/         ← AuthContext y similares          
├── hooks/           ← Custom hooks
├── lib/             ← Utilidades, helpers
├── routes/          ← Definición de rutas               
└── styles/
    └── globals.css  ← Variables CSS y fuente
---

## 8. Convenciones de código

- Nombres de componentes en **PascalCase**: `TurnoCard.tsx`
- Nombres de archivos de hooks con prefijo `use`: `useTurnos.ts`
- Clases Tailwind: ordenar primero layout, luego spacing, luego visual (`flex gap-4 p-4 bg-surface rounded-lg`)
- Un componente por archivo, salvo subcomponentes muy pequeños

---

> 📌 **Última actualización:** Mayo 2025
> Si hacés un cambio relevante al diseño, actualizá este documento y avisá al equipo.
