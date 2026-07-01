import { useEffect, useState } from 'react'
import { useNavigate } from 'react-router-dom'
import { ClaseCardRecepcionista } from '@/components/clases/ClaseCardRecepcionista'
import { clasesService, type ClaseDTO } from '@/services/clases.service'
import { actividadService, type Actividad } from '@/services/actividad.service'
import { profesorService, type Profesor } from '@/services/profesor.service'

export default function ClasesPage() {
  const navigate = useNavigate()
  const [clases, setClases] = useState<ClaseDTO[]>([])
  const [actividades, setActividades] = useState<Actividad[]>([])
  const [profesores, setProfesores] = useState<Profesor[]>([])
  const [loading, setLoading] = useState(true)
  const [error, setError] = useState<string | null>(null)

  useEffect(() => {
    async function cargar() {
      try {
        const [dataClases, dataActividades, dataProfesores] = await Promise.all([
          clasesService.getClases(),
          actividadService.getActividades(),
          profesorService.getProfesores(),
        ])
        setClases(dataClases)
        setActividades(dataActividades)
        setProfesores(dataProfesores)
      } catch {
        setError('No se pudieron cargar las clases.')
      } finally {
        setLoading(false)
      }
    }
    cargar()
  }, [])

  function getNombreActividad(idActividad: string): string {
    return actividades.find((a) => String(a.id) === String(idActividad))?.nombre ?? idActividad
  }

  function getNombreProfesor(dniProfesor: number): string {
    return profesores.find((p) => p.dni === dniProfesor)?.nombre_completo ?? 'Sin asignar'
  }

  if (loading) return <p className="p-8 text-muted text-sm">Cargando clases...</p>
  if (error)   return <p className="p-8 text-destructive text-sm">{error}</p>

  return (
    <main className="p-4 md:p-8 bg-background min-h-screen">
      <div className="flex items-center justify-between mb-6">
        <div>
          <h1 className="text-2xl font-bold text-primary">Gestión de Clases</h1>
          <p className="text-sm mt-1">Administrá las clases y horarios</p>
        </div>
        <button
          onClick={() => navigate('/admin/clases/nueva')}
          className="bg-destructive text-white text-sm font-medium px-4 py-2 rounded-lg hover:opacity-90 transition-opacity"
        >
          + Nueva Clase
        </button>
      </div>

      {clases.length === 0 ? (
        <p className="text-sm">No hay clases cargadas.</p>
      ) : (
        <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
          {clases.map((clase) => (
            <ClaseCardRecepcionista
              key={clase.id_clase}
              idClase={clase.id_clase}
              dia={clase.dia}
              diaSemana={clase.dia_semana}
              horario={clase.horario}
              descripcion={clase.descripcion}
              cupoBase={clase.cupo_base}
              inscripciones={clase.inscripciones}
              idActividad={getNombreActividad(clase.id_actividad)}
              idSala={clase.id_sala}
              dniProfesor={clase.dni_profesor}
              nombreProfesor={getNombreProfesor(clase.dni_profesor)}
              onEditar={() => navigate(`/admin/clases/editar/${clase.id_clase}`)}
              onEliminar={() => console.log('Eliminar clase id:', clase.id_clase)}
            />
          ))}
        </div>
      )}
    </main>
  )
}