import { useEffect, useState } from 'react'
import { useNavigate, useParams } from 'react-router-dom'
import { clasesService, type ClaseDTO } from '@/services/clases.service'
import { profesorService, type Profesor } from '@/services/profesor.service'
import { toast } from 'sonner'

export function EditarClasePage() {
  const { id } = useParams<{ id: string }>()
  const navigate = useNavigate()

  const [clase, setClase] = useState<ClaseDTO | null>(null)
  const [profesores, setProfesores] = useState<Profesor[]>([])
  const [dnisOcupados, setDnisOcupados] = useState<Set<number>>(new Set())
  const [dniProfesorSeleccionado, setDniProfesorSeleccionado] = useState<number | ''>('')
  const [loading, setLoading] = useState(true)
  const [guardando, setGuardando] = useState(false)
  const [error, setError] = useState<string | null>(null)

  useEffect(() => {
    if (!id) return
    async function cargar() {
      try {
        const [dataClase, dataProfesores, dataClases] = await Promise.all([
          clasesService.getClase(id!),
          profesorService.getProfesores(),
          clasesService.getClases(),
        ])
        setClase(dataClase)
        setDniProfesorSeleccionado(dataClase.dni_profesor)

        // Solo profesores activos pueden ser asignados a una clase.
        const activos = dataProfesores.filter((p) => p.estado === 'alta')
        setProfesores(activos)

        // Detectamos qué profesores ya tienen otra clase en el mismo día y horario
        // (excluyendo la propia clase que estamos editando)
        const ocupados = new Set(
          dataClases
            .filter((c) =>
              c.id_clase !== dataClase.id_clase &&
              c.dia === dataClase.dia &&
              c.horario === dataClase.horario
            )
            .map((c) => c.dni_profesor)
        )
        setDnisOcupados(ocupados)
      } catch {
        setError('No se pudo cargar la clase.')
      } finally {
        setLoading(false)
      }
    }
    cargar()
  }, [id])

  function handleSeleccionarProfesor(e: React.ChangeEvent<HTMLSelectElement>) {
    const dni = Number(e.target.value)
    setDniProfesorSeleccionado(dni)

    if (dnisOcupados.has(dni)) {
      const profesor = profesores.find((p) => p.dni === dni)
      toast.warning(
        `${profesor?.nombre_completo ?? 'El profesor'} ya tiene una clase asignada en ese mismo día y horario.`
      )
    }
  }

  async function handleGuardar(e: React.FormEvent) {
    e.preventDefault()
    if (!id || !clase || dniProfesorSeleccionado === '') return

    if (dnisOcupados.has(Number(dniProfesorSeleccionado))) {
      toast.error('No se puede guardar: el profesor ya tiene una clase asignada en ese día y horario.')
      return
    }

    setGuardando(true)
    try {
      await clasesService.actualizarClase(id, {
        id_clase: id,
        dni_profesor: Number(dniProfesorSeleccionado),
        estado: clase.estado,
      })
      toast.success('Clase actualizada con éxito')
      navigate('/admin/clases')
    } catch (err: any) {
      const mensaje = err?.response?.data?.message ?? 'El profesor no se encuentra disponible en ese día y horario.'
      toast.error(mensaje)
    } finally {
      setGuardando(false)
    }
  }

  if (loading) return <p className="p-8 text-muted text-sm">Cargando clase...</p>
  if (error || !clase) return <p className="p-8 text-destructive text-sm">{error ?? 'Clase no encontrada.'}</p>

  return (
    <main className="p-4 md:p-8 bg-background min-h-screen flex justify-center">
      <div className="w-full max-w-md space-y-4">
        <h1 className="text-xl font-bold text-primary">Editar Clase</h1>

        <div className="border border-gray-200 rounded-lg p-4 space-y-2">
          <p className="text-sm text-gray-500">Descripción</p>
          <p className="text-sm font-medium">{clase.descripcion}</p>

          <p className="text-sm text-gray-500 mt-2">Día y horario</p>
          <p className="text-sm font-medium">{clase.dia_semana} — {clase.horario}</p>
        </div>

        <form onSubmit={handleGuardar} className="border border-gray-200 rounded-lg p-4 space-y-3">
          <div>
            <label htmlFor="profesor" className="text-xs text-gray-500 font-medium uppercase tracking-wide">
              Profesor asignado
            </label>
            <select
              id="profesor"
              value={dniProfesorSeleccionado}
              onChange={handleSeleccionarProfesor}
              required
              className="flex h-9 w-full rounded-md border-2 border-[#C8102E] bg-background px-3 py-2 text-sm"
            >
              <option value="" disabled>Seleccioná un profesor</option>
              {profesores.map((p) => (
                <option key={p.dni} value={p.dni}>
                  {p.nombre_completo}
                </option>
              ))}
            </select>
            {dniProfesorSeleccionado !== '' && dnisOcupados.has(Number(dniProfesorSeleccionado)) && (
              <p className="text-xs text-red-500 mt-1">
                Este profesor ya tiene una clase asignada en ese día y horario.
              </p>
            )}
          </div>

          <div className="flex gap-2 pt-2">
            <button
              type="button"
              onClick={() => navigate('/admin/clases')}
              disabled={guardando}
              className="flex-1 border border-gray-300 text-gray-700 rounded-md h-9 text-sm font-medium hover:bg-gray-50 disabled:opacity-50"
            >
              Cancelar
            </button>
            <button
              type="submit"
              disabled={guardando || profesores.length === 0}
              className="flex-1 bg-brand text-white rounded-md h-9 text-sm font-medium hover:opacity-90 disabled:opacity-50"
            >
              {guardando ? 'Guardando...' : 'Guardar cambios'}
            </button>
          </div>
        </form>
      </div>
    </main>
  )
}