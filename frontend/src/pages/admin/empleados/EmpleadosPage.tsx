import { useNavigate } from 'react-router-dom'
import { EmpleadoCard } from '@/components/empleados/EmpleadoCard'

type EstadoEmpleado = 'alta' | 'baja'
type RolEmpleado = 'duenio' | 'empleado' | 'profesor'

interface Empleado {
  dni: number
  nombreApellido: string
  mail: string
  genero: string
  estado: EstadoEmpleado
  rol: RolEmpleado
}

const empleadosIniciales: Empleado[] = [
  {
    dni: 12345678,
    nombreApellido: 'Ana García',
    mail: 'ana.garcia@cef.com',
    genero: 'Femenino',
    estado: 'alta',
    rol: 'empleado',
  },
  {
    dni: 23456789,
    nombreApellido: 'Carlos López',
    mail: 'carlos.lopez@cef.com',
    genero: 'Masculino',
    estado: 'alta',
    rol: 'empleado',
  },
  {
    dni: 34567890,
    nombreApellido: 'María Fernández',
    mail: 'maria.fernandez@cef.com',
    genero: 'Femenino',
    estado: 'baja',
    rol: 'empleado',
  },
]

export function EmpleadosPage() {
  const navigate = useNavigate()

  const handleEditar = (dni: number) => {
    console.log('Editar empleado dni:', dni)
  }

  const handleDesactivar = (dni: number) => {
    console.log('Desactivar empleado dni:', dni)
  }

  return (
    <main className="p-4 md:p-8 bg-background min-h-screen">
      <div className="flex items-center justify-between mb-6">
        <div>
          <h1 className="text-2xl font-bold text-primary">Gestión de Empleados</h1>
          <p className="text-sm text-muted mt-1">Administrá el personal del centro</p>
        </div>
        <button
          onClick={() => navigate('/admin/empleados/nuevo')}
          className="bg-destructive text-white text-sm font-medium px-4 py-2 rounded-lg hover:opacity-90 transition-opacity"
        >
          + Nuevo Empleado
        </button>
      </div>

      <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
        {empleadosIniciales.map((empleado) => (
          <EmpleadoCard
            key={empleado.dni}
            dni={empleado.dni}
            nombreApellido={empleado.nombreApellido}
            mail={empleado.mail}
            genero={empleado.genero}
            estado={empleado.estado}
            rol={empleado.rol}
            onEditar={() => handleEditar(empleado.dni)}
            onDesactivar={() => handleDesactivar(empleado.dni)}
          />
        ))}
      </div>
    </main>
  )
}