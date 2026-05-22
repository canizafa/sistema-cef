import { ClaseCardRecepcionista } from '@/components/clases/ClaseCardRecepcionista'
 
type EstadoClase = 'activa' | 'inactiva'
 
interface Clase {
  id: number
  nombre: string
  profesor: string
  tipo: string
  horario: string
  duracionMin: number
  cupoMaximo: number
  estado: EstadoClase
}
 
const clasesIniciales: Clase[] = [
  {
    id: 1,
    nombre: 'CrossFit',
    profesor: 'Juan Pérez',
    tipo: 'Fuerza',
    horario: 'Lun/Mié/Vie · 18:00 hs',
    duracionMin: 60,
    cupoMaximo: 12,
    estado: 'activa',
  },
  {
    id: 2,
    nombre: 'Yoga',
    profesor: 'María González',
    tipo: 'Flexibilidad',
    horario: 'Mar/Jue · 09:00 hs',
    duracionMin: 45,
    cupoMaximo: 10,
    estado: 'activa',
  },
  {
    id: 3,
    nombre: 'Spinning',
    profesor: 'Laura Martínez',
    tipo: 'Cardio',
    horario: 'Lun/Mié/Vie · 19:30 hs',
    duracionMin: 45,
    cupoMaximo: 15,
    estado: 'activa',
  },
  {
    id: 4,
    nombre: 'Funcional',
    profesor: 'Carlos Ruiz',
    tipo: 'Fuerza',
    horario: 'Mar/Jue/Sáb · 18:00 hs',
    duracionMin: 50,
    cupoMaximo: 15,
    estado: 'activa',
  },
]
 
export default function ClasesPage() {
  const handleEditar = (id: number) => {
    // Aquí iría la navegación a /admin/clases/:id/editar
    console.log('Editar clase id:', id)
  }
 
  const handleVerReservas = (id: number) => {
    // Aquí iría la navegación o modal de reservas
    console.log('Ver reservas de clase id:', id)
  }
 
  return (
    <main className="p-4 md:p-8 bg-background min-h-screen">
      <div className="flex items-center justify-between mb-6">
        <div>
          <h1 className="text-2xl font-bold text-primary">Gestión de Clases</h1>
          <p className="text-sm text-muted mt-1">Administrá las clases y horarios</p>
        </div>
        <button className="bg-destructive text-white text-sm font-medium px-4 py-2 rounded-lg hover:opacity-90 transition-opacity">
          + Nueva Clase
        </button>
      </div>
 
      <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
        {clasesIniciales.map((clase) => (
          <ClaseCardRecepcionista
            key={clase.id}
            nombre={clase.nombre}
            profesor={clase.profesor}
            tipo={clase.tipo}
            horario={clase.horario}
            duracionMin={clase.duracionMin}
            cupoMaximo={clase.cupoMaximo}
            estado={clase.estado}
            onEditar={() => handleEditar(clase.id)}
            onVerReservas={() => handleVerReservas(clase.id)}
          />
        ))}
      </div>
    </main>
  )
}