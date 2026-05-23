import { useNavigate } from 'react-router-dom'
import { ClaseCardRecepcionista } from '@/components/clases/ClaseCardRecepcionista'

type EstadoClase = 'activa' | 'inactiva'

interface Clase {
  idClase: number
  descripcion: string
  dia: string
  horario: string
  idSala: number
  cupoMaximo: number
  estado: EstadoClase
}

const clasesIniciales: Clase[] = [
  {
    idClase: 1,
    descripcion: 'CrossFit',
    dia: 'Lunes / Miércoles / Viernes',
    horario: '18:00 hs',
    idSala: 1,
    cupoMaximo: 12,
    estado: 'activa',
  },
  {
    idClase: 2,
    descripcion: 'Yoga',
    dia: 'Martes / Jueves',
    horario: '09:00 hs',
    idSala: 2,
    cupoMaximo: 10,
    estado: 'activa',
  },
  {
    idClase: 3,
    descripcion: 'Spinning',
    dia: 'Lunes / Miércoles / Viernes',
    horario: '19:30 hs',
    idSala: 3,
    cupoMaximo: 15,
    estado: 'activa',
  },
  {
    idClase: 4,
    descripcion: 'Funcional',
    dia: 'Martes / Jueves / Sábado',
    horario: '18:00 hs',
    idSala: 1,
    cupoMaximo: 15,
    estado: 'activa',
  },
]

export default function ClasesPage() {
  const navigate = useNavigate()

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
        <button
          onClick={() => navigate('/admin/clases/nueva')}
          className="bg-destructive text-white text-sm font-medium px-4 py-2 rounded-lg hover:opacity-90 transition-opacity"
        >
          + Nueva Clase
        </button>
      </div>

      <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
        {clasesIniciales.map((clase) => (
          <ClaseCardRecepcionista
            key={clase.idClase}
            idClase={clase.idClase}
            descripcion={clase.descripcion}
            dia={clase.dia}
            horario={clase.horario}
            idSala={clase.idSala}
            cupoMaximo={clase.cupoMaximo}
            estado={clase.estado}
            onEditar={() => handleEditar(clase.idClase)}
            onVerReservas={() => handleVerReservas(clase.idClase)}
          />
        ))}
      </div>
    </main>
  )
}