import { useEffect, useState } from 'react'
import { useNavigate } from 'react-router-dom'
import { EmpleadoCard } from '@/components/empleados/EmpleadoCard'
import { empleadoService } from '@/services/empleados.service'

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

export function EmpleadosPage() {
  const navigate = useNavigate()
  const [empleados, setEmpleados] = useState<Empleado[]>([])
  const [loading, setLoading] = useState(true)
  const [error, setError] = useState<string | null>(null)

  useEffect(() => {
    empleadoService.getEmpleados()
      .then((data) => {
        setEmpleados(data
          .filter((e: any) => e.rol === 'empleado')
          .map((e: any) => ({
            dni: e.dni,
            nombreApellido: e.nombre_apellido,
            mail: e.mail,
            genero: e.genero,
            estado: e.estado,
            rol: e.rol,
          })))
      })
      .catch(() => setError('No se pudieron cargar los empleados'))
      .finally(() => setLoading(false))
  }, [])

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
          <p className="text-sm mt-1">Administrá el personal del centro</p>
        </div>
        <button
          onClick={() => navigate('/admin/empleados/nuevo')}
          className="bg-destructive text-white text-sm font-medium px-4 py-2 rounded-lg hover:opacity-90 transition-opacity"
        >
          + Nuevo Empleado
        </button>
      </div>

      {loading && <p className="text-sm text-muted">Cargando empleados...</p>}
      {error && <p className="text-sm text-destructive">{error}</p>}

      <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
        {empleados.map((empleado) => (
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