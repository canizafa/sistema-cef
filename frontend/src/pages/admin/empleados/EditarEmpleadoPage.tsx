import { useState, useEffect } from 'react'
import { useParams, useNavigate } from 'react-router-dom'
import { empleadoService } from '@/services/empleados.service'

export function EditarEmpleadoPage() {
  const { id } = useParams()
  const navigate = useNavigate()

  const [form, setForm] = useState({
    nombre: '',
    apellido: '',
    dni: '',
    mail: '',
  })
  const [genero, setGenero] = useState('')
  const [estado, setEstado] = useState('')
  const [rol, setRol] = useState('')
  const [error, setError] = useState<string | null>(null)
  const [success, setSuccess] = useState<string | null>(null)
  const [loading, setLoading] = useState(false)
  const [loadingDatos, setLoadingDatos] = useState(true)

  useEffect(() => {
    empleadoService.getEmpleados()
      .then((data: any[]) => {
        const empleado = data.find((e) => String(e.dni) === String(id))
        if (!empleado) {
          setError('No se encontró el empleado')
          return
        }
        const [nombre, ...resto] = empleado.nombre_apellido.split(' ')
        setForm({
          nombre,
          apellido: resto.join(' '),
          dni: String(empleado.dni),
          mail: empleado.mail,
        })
        setGenero(empleado.genero)
        setEstado(empleado.estado)
        setRol(empleado.rol)
      })
      .catch(() => setError('No se pudieron cargar los datos del empleado'))
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
      await empleadoService.actualizarEmpleado(Number(id), {
        dni: Number(form.dni),
        nombre_apellido: `${form.nombre} ${form.apellido}`,
        mail: form.mail,
        genero,
        estado,
        rol,
      })
      setSuccess('Empleado actualizado correctamente')
      setTimeout(() => navigate('/admin/empleados', { state: { editadoDni: Number(id) } }), 3000)
    } catch {
      setError('Error al actualizar el empleado. Revisá los datos.')
    } finally {
      setLoading(false)
    }
  }

  if (loadingDatos) {
    return (
      <main className="flex-1 flex items-center justify-center px-4 py-12">
        <p className="text-sm text-muted">Cargando datos del empleado...</p>
      </main>
    )
  }

  return (
    <main className="flex-1 flex items-center justify-center px-4 py-12">
      <div className="w-full max-w-sm">
        <h1 className="text-2xl font-bold mb-6 text-center">Editar empleado</h1>
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
              value={form.dni}
              disabled
              className="flex h-10 w-full rounded-md border border-input bg-background px-3 py-2 text-sm opacity-50 cursor-not-allowed"
            />
          </div>
          <div className="space-y-1">
            <label htmlFor="mail" className="text-sm font-medium">Email</label>
            <input
              id="mail"
              name="mail"
              type="email"
              placeholder="empleado@cef.com"
              value={form.mail}
              onChange={handleChange}
              required
              className="flex h-10 w-full rounded-md border border-input bg-background px-3 py-2 text-sm"
            />
          </div>

          {error && <p className="text-sm text-red-600">{error}</p>}
          {success && <p className="text-sm text-green-600">{success}</p>}

          <div className="flex gap-2 pt-2">
            <button
              type="button"
              onClick={() => navigate('/admin/empleados')}
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