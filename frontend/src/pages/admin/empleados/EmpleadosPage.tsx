import { useEffect, useState } from 'react'
import { useNavigate } from 'react-router-dom'
import { Search } from 'lucide-react'
import { toast } from 'sonner'
import { EmpleadoCard } from '@/components/empleados/EmpleadoCard'
import { EliminarEmpleadoModal } from '@/components/empleados/EliminarEmpleadoModal'
import { empleadoService } from '@/services/empleados.service'

type EstadoEmpleado = 'alta' | 'baja' | 'eliminado'
type RolEmpleado = 'duenio' | 'empleado' | 'profesor'
type FiltroEmpleado = 'todos' | 'alta' | 'baja' | 'eliminado'

interface Empleado {
  dni: number
  nombreApellido: string
  mail: string
  genero: string
  estado: EstadoEmpleado
  rol: RolEmpleado
}

const normalizar = (texto: string) =>
  texto.toLowerCase().normalize('NFD').replace(/[\u0300-\u036f]/g, '')

export function EmpleadosPage() {
  const navigate = useNavigate()
  const [empleados, setEmpleados] = useState<Empleado[]>([])
  const [loading, setLoading] = useState(true)
  const [error, setError] = useState<string | null>(null)
  const [filtro, setFiltro] = useState<FiltroEmpleado>('todos')
  const [busquedaNombre, setBusquedaNombre] = useState('')

  const [modalEliminarAbierto, setModalEliminarAbierto] = useState(false)
  const [empleadoAEliminar, setEmpleadoAEliminar] = useState<Empleado | null>(null)

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
    navigate(`/admin/empleados/${dni}/editar`)
  }

  const handleDesactivar = async (dni: number) => {
    const empleado = empleados.find((e) => e.dni === dni)
    if (!empleado) return
    try {
      await empleadoService.desactivarEmpleado({
        dni: empleado.dni,
        nombre_apellido: empleado.nombreApellido,
        mail: empleado.mail,
        genero: empleado.genero,
        estado: empleado.estado,
        rol: empleado.rol,
      })
      setEmpleados((prev) =>
        prev.map((e) => e.dni === dni ? { ...e, estado: 'baja' as EstadoEmpleado } : e)
      )
      toast.success('Empleado desactivado correctamente')
    } catch {
      toast.error('No se pudo desactivar el empleado')
    }
  }

  const handleActivar = async (dni: number) => {
    const empleado = empleados.find((e) => e.dni === dni)
    if (!empleado) return
    try {
      await empleadoService.activarEmpleado({
        dni: empleado.dni,
        nombre_apellido: empleado.nombreApellido,
        mail: empleado.mail,
        genero: empleado.genero,
        estado: empleado.estado,
        rol: empleado.rol,
      })
      setEmpleados((prev) =>
        prev.map((e) => e.dni === dni ? { ...e, estado: 'alta' as EstadoEmpleado } : e)
      )
      toast.success('Empleado activado correctamente')
    } catch {
      toast.error('No se pudo activar el empleado')
    }
  }

  const handleEliminar = (dni: number) => {
    const empleado = empleados.find((e) => e.dni === dni)
    if (!empleado) return
    setEmpleadoAEliminar(empleado)
    setModalEliminarAbierto(true)
  }

  const handleEliminarConfirmado = () => {
    if (!empleadoAEliminar) return
    setEmpleados((prev) =>
      prev.map((e) =>
        e.dni === empleadoAEliminar.dni
          ? { ...e, estado: 'eliminado' as EstadoEmpleado }
          : e
      )
    )
    setEmpleadoAEliminar(null)
  }

  const empleadosFiltrados = empleados.filter((e) => {
    if (busquedaNombre.trim() !== '') {
      return normalizar(e.nombreApellido)
        .startsWith(normalizar(busquedaNombre.trim()))
    }

    if (filtro === 'alta') return e.estado === 'alta'
    if (filtro === 'baja') return e.estado === 'baja'
    if (filtro === 'eliminado') return e.estado === 'eliminado'
    return e.estado !== 'eliminado'
  })

  const mensajeVacio = () => {
    if (busquedaNombre.trim() !== '') {
      return 'No existe un empleado con ese nombre y apellido.'
    }
    if (filtro === 'todos') return 'No hay empleados registrados en el sistema.'
    return 'No existen empleados con el filtro solicitado.'
  }

  const tabs: { label: string; value: FiltroEmpleado }[] = [
    { label: 'Todos', value: 'todos' },
    { label: 'Activos', value: 'alta' },
    { label: 'Inactivos', value: 'baja' },
    { label: 'Eliminados', value: 'eliminado' },
  ]

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

      <div className="relative mb-4">
        <Search className="absolute left-3 top-2.5 w-4 h-4 text-gray-500" />
        <input
          type="text"
          placeholder="Buscar empleado por nombre y apellido"
          value={busquedaNombre}
          onChange={(e) => setBusquedaNombre(e.target.value)}
          className="w-full border-2 border-brand rounded-lg pl-9 pr-4 h-10 text-sm bg-background text-primary placeholder:text-gray-400"
        />
      </div>

      <div className="flex gap-2 mb-6">
        {tabs.map((tab) => (
          <button
            key={tab.value}
            onClick={() => setFiltro(tab.value)}
            disabled={busquedaNombre.trim() !== ''}
            className={`px-4 py-1.5 rounded-full text-sm font-medium transition-colors
              ${filtro === tab.value && busquedaNombre.trim() === ''
                ? 'bg-brand text-white'
                : 'bg-border text-gray-500'
              }
              ${busquedaNombre.trim() !== ''
                ? 'opacity-40 cursor-not-allowed'
                : 'hover:bg-muted hover:text-white'
              }`}
          >
            {tab.label}
          </button>
        ))}
      </div>

      {loading && <p className="text-sm text-muted">Cargando empleados...</p>}
      {error && <p className="text-sm text-destructive">{error}</p>}

      {!loading && !error && empleadosFiltrados.length === 0 && (
        <p className="text-sm" style={{ color: '#4B5563' }}>{mensajeVacio()}</p>
      )}

      <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
        {empleadosFiltrados.map((empleado) => (
          <EmpleadoCard
            key={empleado.dni}
            dni={empleado.dni}
            nombreApellido={empleado.nombreApellido}
            mail={empleado.mail}
            estado={empleado.estado}
            rol={empleado.rol}
            onEditar={() => handleEditar(empleado.dni)}
            onDesactivar={() => handleDesactivar(empleado.dni)}
            onActivar={() => handleActivar(empleado.dni)}
            onEliminar={() => handleEliminar(empleado.dni)}
          />
        ))}
      </div>

      {empleadoAEliminar && (
        <EliminarEmpleadoModal
          open={modalEliminarAbierto}
          onOpenChange={setModalEliminarAbierto}
          empleado={{
            dni: empleadoAEliminar.dni,
            nombre_apellido: empleadoAEliminar.nombreApellido,
            mail: empleadoAEliminar.mail,
            genero: empleadoAEliminar.genero,
            estado: empleadoAEliminar.estado,
            rol: empleadoAEliminar.rol,
          }}
          onEliminado={handleEliminarConfirmado}
        />
      )}
    </main>
  )
}