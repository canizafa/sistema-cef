import { useState, useEffect } from 'react'
import { useParams, useNavigate } from 'react-router-dom'
import { profesorService } from '@/services/profesor.service'

export function EditarProfesoresPage() {
  const { id } = useParams()
  const navigate = useNavigate()

  const [form, setForm] = useState({
    nombre: '',
    apellido: '',
  })
  const [genero, setGenero] = useState('')
  const [estado, setEstado] = useState('')
  const [error, setError] = useState<string | null>(null)
  const [success, setSuccess] = useState<string | null>(null)
  const [loading, setLoading] = useState(false)
  const [loadingDatos, setLoadingDatos] = useState(true)

  useEffect(() => {
    profesorService.getProfesor(Number(id))
      .then((profesor) => {
        const [nombre, ...resto] = profesor.nombre_completo.split(' ')
        setForm({
          nombre,
          apellido: resto.join(' '),
        })
        setGenero('otro')
        setEstado(profesor.estado)
      })
      .catch(() => setError('No se pudieron cargar los datos del profesor'))
      .finally(() => setLoadingDatos(false))
  }, [id])

  function handleChange(e: React.ChangeEvent<HTMLInputElement>) {
    setForm((prev) => ({ ...prev, [e.target.name]: e.target.value }))
  }

  async function handleSubmit(e: React.FormEvent) {
    e.preventDefault()
    setError(null)
    setSuccess(null)
    setLoading(true)
    try {
      await profesorService.actualizarProfesor(Number(id), {
        nombre_completo: `${form.nombre} ${form.apellido}`,
        genero,
        estado,
      })
      setSuccess('Profesor actualizado correctamente')
      setTimeout(() => navigate('/admin/profesores'), 3000)
    } catch {
      setError('Error al actualizar el profesor. Revisá los datos.')
    } finally {
      setLoading(false)
    }
  }

  if (loadingDatos) {
    return (
      <main className="flex-1 flex items-center justify-center px-4 py-12">
        <p className="text-sm text-muted">Cargando datos del profesor...</p>
      </main>
    )
  }

  return (
    <main className="flex-1 flex items-center justify-center px-4 py-12">
      <div className="w-full max-w-sm">
        <h1 className="text-2xl font-bold mb-6 text-center">Editar profesor</h1>
        <form onSubmit={handleSubmit} className="space-y-4">
          <div className="space-y-1">
            <label htmlFor="nombre" className="text-sm font-medium">Nombre</label>
            <input
              id="nombre"
              name="nombre"
              value={form.nombre}
              onChange={handleChange}
              required
              className="flex h-10 w-full rounded-md border border-input bg-background px-3 py-2 text-sm"
            />
          </div>
          <div className="space-y-1">
            <label htmlFor="apellido" className="text-sm font-medium">Apellido</label>
            <input
              id="apellido"
              name="apellido"
              value={form.apellido}
              onChange={handleChange}
              required
              className="flex h-10 w-full rounded-md border border-input bg-background px-3 py-2 text-sm"
            />
          </div>
          <div className="space-y-1">
            <label htmlFor="dni" className="text-sm font-medium">DNI</label>
            <input
              id="dni"
              name="dni"
              value={id}
              disabled
              className="flex h-10 w-full rounded-md border border-input bg-background px-3 py-2 text-sm opacity-50 cursor-not-allowed"
            />
          </div>

          {error && <p className="text-sm text-red-600">{error}</p>}
          {success && <p className="text-sm text-green-600">{success}</p>}

          <div className="flex gap-2 pt-2">
            <button
              type="button"
              onClick={() => navigate('/admin/profesores')}
              className="flex-1 border border-input bg-background text-sm font-medium rounded-md h-10 hover:bg-surface"
            >
              Cancelar
            </button>
            <button
              type="submit"
              disabled={loading}
              className="flex-1 bg-brand text-white rounded-md h-10 text-sm font-medium hover:opacity-90 disabled:opacity-50"
            >
              {loading ? 'Guardando...' : 'Guardar cambios'}
            </button>
          </div>
        </form>
      </div>
    </main>
  )
}