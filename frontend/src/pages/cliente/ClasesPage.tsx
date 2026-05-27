import { useEffect, useState } from 'react'
import { useAuth } from '@/context/AuthContext'
import { ClaseCardCliente, type EstadoReserva } from '@/components/clases/ClaseCardCliente'
import { ReservaModal } from '@/components/clases/ReservaModal'
import { Header } from '@/components/layout/Header'
import { clasesService, type ClaseDTO } from '@/services/clases.service'

export function ClasesPage() {
  const { user } = useAuth()
  const [clases, setClases] = useState<ClaseDTO[]>([])
  const [reservadas, setReservadas] = useState<Set<string>>(new Set())
  const [loading, setLoading] = useState(true)
  const [error, setError] = useState<string | null>(null)
  const [claseSeleccionada, setClaseSeleccionada] = useState<ClaseDTO | null>(null)
  const [modalAbierto, setModalAbierto] = useState(false)

  useEffect(() => {
    async function cargar() {
      try {
        const data = await clasesService.getClases()
        setClases(data)
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
    if (clase.lleno) return 'sin-cupo'
    return 'disponible'
  }

  function handleReservar(clase: ClaseDTO) {
    setClaseSeleccionada(clase)
    setModalAbierto(true)
  }

  function handleConfirmar() {
    if (!claseSeleccionada) return
    // TODO: redirigir a MP cuando el back esté listo
    setReservadas((prev) => new Set(prev).add(claseSeleccionada.id_clase))
    setModalAbierto(false)
    setClaseSeleccionada(null)
  }

  function handleCancelarModal() {
    setModalAbierto(false)
    setClaseSeleccionada(null)
  }

  async function handleCancelar(idClase: string) {
    // TODO: llamar al back cuando esté listo
    setReservadas((prev) => {
      const next = new Set(prev)
      next.delete(idClase)
      return next
    })
  }

  async function handleListaEspera(_clase: ClaseDTO) {
    // TODO: implementar cuando el back tenga el endpoint de lista de espera
  }

  if (loading) return <p className="p-8 text-muted text-sm">Cargando clases...</p>
  if (error) return <p className="p-8 text-destructive text-sm">{error}</p>

  return (
    <div className="min-h-screen flex flex-col bg-background">
      <Header />
      <main className="flex-1 p-4 md:p-8">
        <div className="mb-6">
          <h1 className="text-2xl font-bold text-primary">Clases disponibles</h1>
          <p className="text-sm text-gray-600 mt-1">
            Hola {user?.email}, inscribite a una clase o anotate en lista de espera
          </p>
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
                descripcion={clase.descripcion}
                estadoReserva={getEstadoReserva(clase)}
                onReservar={() => handleReservar(clase)}
                onCancelar={() => handleCancelar(clase.id_clase)}
                onListaEspera={() => handleListaEspera(clase)}
              />
            ))}
          </div>
        )}

        <ReservaModal
          clase={claseSeleccionada}
          abierto={modalAbierto}
          onCancelar={handleCancelarModal}
          onConfirmar={handleConfirmar}
        />
      </main>
    </div>
  )
}