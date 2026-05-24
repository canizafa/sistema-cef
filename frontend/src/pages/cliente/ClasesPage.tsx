import { useEffect, useState } from 'react'
import { useAuth } from '@/context/AuthContext'
import { ClaseCardCliente, type EstadoReserva } from '@/components/clases/ClaseCardCliente'
import {
  clasesService,
  reservasService,
  listaEsperaService,
  type ClaseDTO,
} from '@/services/clases.service'

export function ClasesPage() {
  const { user } = useAuth()
  const [clases, setClases] = useState<ClaseDTO[]>([])
  const [reservadas, setReservadas] = useState<Set<number>>(new Set())
  const [loading, setLoading] = useState(true)
  const [error, setError] = useState<string | null>(null)

  useEffect(() => {
    async function cargar() {
      try {
        const data = await clasesService.getClases()
        setClases(data)
        // TODO: cuando exista GET /reservas?dni_cliente=X, poblar `reservadas` acá
      } catch {
        setError('No se pudieron cargar las clases.')
      } finally {
        setLoading(false)
      }
    }
    cargar()
  }, [])

  function getEstadoReserva(clase: ClaseDTO): EstadoReserva {
    if (reservadas.has(clase.id_clase)) return 'reservada'
    if (clase.estado === 'inactiva') return 'sin-cupo'
    return 'disponible'
  }

  async function handleReservar(clase: ClaseDTO) {
    if (!user?.dni) return
    try {
      await reservasService.crearReserva({
        fecha: clase.dia,
        estado: 'activa',
        dni_cliente: user.dni,
        id_clase: clase.id_clase,
      })
      setReservadas((prev) => new Set(prev).add(clase.id_clase))
    } catch {
      alert('No se pudo realizar la reserva.')
    }
  }

  async function handleCancelar(idClase: number) {
    // TODO: usar id_reserva cuando el backend lo devuelva en la respuesta
    setReservadas((prev) => {
      const next = new Set(prev)
      next.delete(idClase)
      return next
    })
  }

  async function handleListaEspera(clase: ClaseDTO) {
    if (!user?.dni) return
    try {
      await listaEsperaService.anotarse({
        dni_cliente: user.dni,
        id_clase: clase.id_clase,
        fecha: clase.dia,
      })
      alert('Te anotaste en la lista de espera.')
    } catch {
      alert('No se pudo anotar en la lista de espera.')
    }
  }

  if (loading) return <p className="p-8 text-muted text-sm">Cargando clases...</p>
  if (error)   return <p className="p-8 text-destructive text-sm">{error}</p>

  return (
    <main className="p-4 md:p-8 bg-background min-h-screen">
      <div className="mb-6">
        <h1 className="text-2xl font-bold text-primary">Clases disponibles</h1>
        <p className="text-sm text-muted mt-1">Inscribite a una clase o anotate en lista de espera</p>
      </div>

      {clases.length === 0 ? (
        <p className="text-sm text-muted">No hay clases disponibles por el momento.</p>
      ) : (
        <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
          {clases.map((clase) => (
            <ClaseCardCliente
              key={clase.id_clase}
              idClase={clase.id_clase}
              dia={clase.dia}
              horario={clase.horario}
              estadoReserva={getEstadoReserva(clase)}
              onReservar={() => handleReservar(clase)}
              onCancelar={() => handleCancelar(clase.id_clase)}
              onListaEspera={() => handleListaEspera(clase)}
            />
          ))}
        </div>
      )}
    </main>
  )
}