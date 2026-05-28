import { useEffect, useState } from 'react'
import { useNavigate } from 'react-router-dom'
import { ProfesorCard } from '@/components/empleados/ProfesorCard'
import { profesorService } from '@/services/profesor.service'

type EstadoProfesor = 'alta' | 'baja'

interface Profesor {
  dni: number
  nombreCompleto: string
  estado: EstadoProfesor
}

export function ProfesoresPage() {
  const navigate = useNavigate()
  const [profesores, setProfesores] = useState<Profesor[]>([])
  const [loading, setLoading] = useState(true)
  const [error, setError] = useState<string | null>(null)
  const [soloActivos, setSoloActivos] = useState(false)

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

  const todosActivos = profesores.length > 0 && profesores.every((p) => p.estado === 'alta')

  const profesoresFiltrados = soloActivos
    ? profesores.filter((p) => p.estado === 'alta')
    : profesores

  function handleToggleFiltro() {
    if (!soloActivos && todosActivos) return
    setSoloActivos((prev) => !prev)
  }

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

      <div className="mb-4">
        <button
          onClick={handleToggleFiltro}
          className="bg-brand text-white text-sm font-medium px-4 py-2 rounded-lg hover:opacity-90 transition-opacity"
        >
          {soloActivos ? 'Listar todos los profesores' : 'Listar profesores activos'}
        </button>
        {!soloActivos && todosActivos && (
          <p className="text-sm  mt-2">Todos los profesores están activos.</p>
        )}
      </div>

      {loading && <p className="text-sm text-muted">Cargando profesores...</p>}
      {error && <p className="text-sm text-destructive">{error}</p>}

      {!loading && !error && profesores.length === 0 && (
        <p className="text-sm ">No hay profesores registrados en el sistema.</p>
      )}

      {!loading && !error && soloActivos && profesoresFiltrados.length === 0 && (
        <p className="text-sm ">No hay profesores activos.</p>
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