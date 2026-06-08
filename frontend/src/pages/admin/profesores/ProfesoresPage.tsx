import { useEffect, useState } from 'react'
import { useNavigate } from 'react-router-dom'
import { Search } from 'lucide-react'
import { ProfesorCard } from '@/components/empleados/ProfesorCard'
import { profesorService } from '@/services/profesor.service'

type EstadoProfesor = 'alta' | 'baja'
type FiltroProfesor = 'todos' | 'alta' | 'baja'

interface Profesor {
  dni: number
  nombreCompleto: string
  estado: EstadoProfesor
}

const normalizar = (texto: string) =>
  texto.toLowerCase().normalize('NFD').replace(/[\u0300-\u036f]/g, '')

export function ProfesoresPage() {
  const navigate = useNavigate()
  const [profesores, setProfesores] = useState<Profesor[]>([])
  const [loading, setLoading] = useState(true)
  const [error, setError] = useState<string | null>(null)
  const [filtro, setFiltro] = useState<FiltroProfesor>('todos')
  const [busquedaNombre, setBusquedaNombre] = useState('')

  useEffect(() => {
    profesorService.getProfesores()
      .then((data) => {
        setProfesores(data.map((p: any) => ({
          dni: p.dni,
          nombreCompleto: p.nombre_completo,
          estado: p.estado,
        })))
      })
      .catch(() => setError('No se pudieron cargar los profesores'))
      .finally(() => setLoading(false))
  }, [])

  const handleEditar = (dni: number) => {
    console.log('Editar profesor dni:', dni)
  }

  const handleDesactivar = async (dni: number) => {
    const profesor = profesores.find((p) => p.dni === dni)
    if (!profesor) return
    try {
      await profesorService.desactivarProfesor({
        dni: profesor.dni,
        nombre_completo: profesor.nombreCompleto,
        estado: profesor.estado,
      })
      setProfesores((prev) =>
        prev.map((p) => p.dni === dni ? { ...p, estado: 'baja' as EstadoProfesor } : p)
      )
    } catch {
      setError('No se pudo desactivar el profesor')
    }
  }

  const handleActivar = async (dni: number) => {
    const profesor = profesores.find((p) => p.dni === dni)
    if (!profesor) return
    try {
      await profesorService.activarProfesor({
        dni: profesor.dni,
        nombre_completo: profesor.nombreCompleto,
        estado: profesor.estado,
      })
      setProfesores((prev) =>
        prev.map((p) => p.dni === dni ? { ...p, estado: 'alta' as EstadoProfesor } : p)
      )
    } catch {
      setError('No se pudo activar el profesor')
    }
  }

  const profesoresFiltrados = profesores.filter((p) => {
    if (busquedaNombre.trim() !== '') {
      return normalizar(p.nombreCompleto)
        .startsWith(normalizar(busquedaNombre.trim()))
    }

    return filtro === 'alta' ? p.estado === 'alta' :
           filtro === 'baja' ? p.estado === 'baja' :
           true
  })

  const mensajeVacio = () => {
    if (busquedaNombre.trim() !== '') {
      return 'No existe un profesor con ese nombre y apellido.'
    }
    if (filtro === 'alta') return 'No existen profesores activos en el sistema.'
    if (filtro === 'baja') return 'No existen profesores inactivos en el sistema.'
    return 'No hay profesores registrados en el sistema.'
  }

  const tabs: { label: string; value: FiltroProfesor }[] = [
    { label: 'Todos', value: 'todos' },
    { label: 'Activos', value: 'alta' },
    { label: 'Inactivos', value: 'baja' },
  ]

  return (
    <main className="p-4 md:p-8 bg-background min-h-screen">
      <div className="flex items-center justify-between mb-6">
        <div>
          <h1 className="text-2xl font-bold text-primary">Gestión de Profesores</h1>
          <p className="text-sm mt-1">Administrá el personal docente del centro</p>
        </div>
        <button
          onClick={() => navigate('/admin/profesores/nuevo')}
          className="bg-destructive text-white text-sm font-medium px-4 py-2 rounded-lg hover:opacity-90 transition-opacity"
        >
          + Nuevo Profesor
        </button>
      </div>

      <div className="relative mb-4">
        <Search className="absolute left-3 top-2.5 w-4 h-4 text-gray-500" />
        <input
          type="text"
          placeholder="Buscar profesor por nombre y apellido"
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

      {loading && <p className="text-sm text-muted">Cargando profesores...</p>}
      {error && <p className="text-sm text-destructive">{error}</p>}

      {!loading && !error && profesoresFiltrados.length === 0 && (
        <p className="text-sm" style={{ color: '#4B5563' }}>{mensajeVacio()}</p>
      )}

      <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
        {profesoresFiltrados.map((profesor) => (
          <ProfesorCard
            key={profesor.dni}
            dni={profesor.dni}
            nombreCompleto={profesor.nombreCompleto}
            estado={profesor.estado}
            onEditar={() => handleEditar(profesor.dni)}
            onDesactivar={() => handleDesactivar(profesor.dni)}
            onActivar={() => handleActivar(profesor.dni)}
          />
        ))}
      </div>
    </main>
  )
}