import { useState } from 'react'
import { Search } from 'lucide-react'
import { toast } from 'sonner'
import { reservasService, clasesService } from '@/services/clases.service'
import { asistenciaService } from '@/services/asistencia.service'

interface ReservaConClase {
  id_reserva: string
  fecha: string
  tipo: string
  estado: string
  id_clase: string
  nombreClase: string
  horario: string
  dia: string
}

export function AsistenciasPage() {
  const [dni, setDni] = useState('')
  const [reservas, setReservas] = useState<ReservaConClase[]>([])
  const [loading, setLoading] = useState(false)
  const [loadingAsistencia, setLoadingAsistencia] = useState<string | null>(null)
  const [asistenciasRegistradas, setAsistenciasRegistradas] = useState<Set<string>>(new Set())
  const [error, setError] = useState<string | null>(null)
  const [buscado, setBuscado] = useState(false)

  async function handleBuscar(e: React.FormEvent) {
    e.preventDefault()
    if (!dni.trim()) return
    setError(null)
    setReservas([])
    setBuscado(false)
    setAsistenciasRegistradas(new Set())
    setLoading(true)
    try {
      const todasReservas = await reservasService.getReservas()
      const reservasCliente = todasReservas.filter(
        (r) => String(r.dni_cliente) === String(dni.trim())
      )

      if (reservasCliente.length === 0) {
        setBuscado(true)
        setLoading(false)
        return
      }

      const clases = await clasesService.getClases()

      const reservasConClase: ReservaConClase[] = reservasCliente.map((r) => {
        const clase = clases.find((c) => c.id_clase === r.id_clase)
        return {
          id_reserva: r.id_reserva,
          fecha: r.fecha,
          tipo: r.tipo,
          estado: r.estado,
          id_clase: r.id_clase,
          nombreClase: clase?.descripcion ?? 'Clase sin nombre',
          horario: clase?.horario ?? '-',
          dia: clase?.dia ?? '-',
        }
      })

      const asistenciasCheck = await Promise.all(
        reservasConClase.map((r) =>
          asistenciaService.getAsistenciaPorReserva(r.id_reserva)
            .then((a) => a ? r.id_reserva : null)
        )
      )
      const registradas = new Set(asistenciasCheck.filter(Boolean) as string[])
      setAsistenciasRegistradas(registradas)

      setReservas(reservasConClase)
      setBuscado(true)
    } catch {
      setError('No se pudieron cargar las reservas. Revisá el DNI ingresado.')
    } finally {
      setLoading(false)
    }
  }

  async function handlePasarAsistencia(idReserva: string) {
    setLoadingAsistencia(idReserva)
    try {
      await asistenciaService.crearAsistencia({
        fecha: new Date().toISOString().split('T')[0],
        metodo: 'presencial',
        id_reserva: idReserva,
      })
      setAsistenciasRegistradas((prev) => new Set(prev).add(idReserva))
      toast.success('Asistencia registrada correctamente')
    } catch (error: any) {
      if (error.response?.status === 409) {
        toast.error('La asistencia ya fue registrada para esta reserva.')
      } else {
        toast.error('No se pudo registrar la asistencia.')
      }
    } finally {
      setLoadingAsistencia(null)
    }
  }

  return (
    <main className="p-4 md:p-8 bg-background min-h-screen">
      <div className="mb-6">
        <h1 className="text-2xl font-bold text-primary">Pasar Asistencia</h1>
        <p className="text-sm mt-1">Ingresá el DNI del cliente para ver sus clases y registrar asistencia</p>
      </div>

      <form onSubmit={handleBuscar} className="flex gap-2 mb-6 max-w-sm">
        <div className="relative flex-1">
          <Search className="absolute left-3 top-2.5 w-4 h-4 text-gray-500" />
          <input
            type="text"
            placeholder="Ingresá el DNI"
            value={dni}
            onChange={(e) => setDni(e.target.value)}
            className="w-full border-2 border-brand rounded-lg pl-9 pr-4 h-10 text-sm bg-background text-primary placeholder:text-gray-400"
          />
        </div>
        <button
          type="submit"
          disabled={loading || !dni.trim()}
          className="bg-brand text-white text-sm font-medium px-4 h-10 rounded-lg hover:opacity-90 disabled:opacity-50"
        >
          {loading ? 'Buscando...' : 'Buscar'}
        </button>
      </form>

      {error && <p className="text-sm text-destructive mb-4">{error}</p>}

      {buscado && reservas.length === 0 && (
        <p className="text-sm" style={{ color: '#4B5563' }}>
          No se encontraron reservas para el DNI ingresado.
        </p>
      )}

      {reservas.length > 0 && (
        <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
          {reservas.map((reserva) => {
            const registrada = asistenciasRegistradas.has(reserva.id_reserva)
            return (
              <div
                key={reserva.id_reserva}
                className="border border-border rounded-xl p-4 bg-background space-y-2"
              >
                <h2 className="font-semibold text-base text-primary">{reserva.nombreClase}</h2>
                <p className="text-sm text-gray-500">{reserva.dia} — {reserva.horario}</p>
                <p className="text-sm text-gray-500">Reserva: {reserva.fecha}</p>
                <p className="text-sm text-gray-500">Tipo: {reserva.tipo}</p>
                <button
                  onClick={() => handlePasarAsistencia(reserva.id_reserva)}
                  disabled={loadingAsistencia === reserva.id_reserva || registrada}
                  className={`w-full text-sm font-medium h-9 rounded-lg mt-2 transition-colors
                    ${registrada
                      ? 'bg-green-600 text-white cursor-not-allowed opacity-80'
                      : 'bg-brand text-white hover:opacity-90 disabled:opacity-50'
                    }`}
                >
                  {loadingAsistencia === reserva.id_reserva
                    ? 'Registrando...'
                    : registrada
                    ? '✓ Asistencia ya registrada'
                    : 'Pasar asistencia'}
                </button>
              </div>
            )
          })}
        </div>
      )}
    </main>
  )
}