import { useEffect, useState } from 'react'
import { useAuth } from '@/context/AuthContext'
import { ClaseCardCliente, type EstadoReserva } from '@/components/clases/ClaseCardCliente'
import { ReservaModal } from '@/components/clases/ReservaModal'
import { Header } from '@/components/layout/Header'
import { clasesService, type ClaseDTO } from '@/services/clases.service'
import { actividadService, type Actividad } from '@/services/actividad.service'
import { profesorService, type Profesor } from '@/services/profesor.service'
import { pagosService } from '@/services/pagos.service'

export function ClasesPage() {
  const { user } = useAuth()
  const [clases, setClases] = useState<ClaseDTO[]>([])
  const [actividades, setActividades] = useState<Actividad[]>([])
  const [profesores, setProfesores] = useState<Profesor[]>([])
  const [reservadas, setReservadas] = useState<Set<string>>(new Set())
  const [loading, setLoading] = useState(true)
  const [error, setError] = useState<string | null>(null)
  const [claseSeleccionada, setClaseSeleccionada] = useState<ClaseDTO | null>(null)
  const [modalAbierto, setModalAbierto] = useState(false)
  const [loadingPago, setLoadingPago] = useState(false)

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

  function getEstadoReserva(clase: ClaseDTO): EstadoReserva {
    if (reservadas.has(clase.id_clase)) return 'reservada'
    if (clase.lleno) return 'sin-cupo'
    return 'disponible'
  }

  function handleReservar(clase: ClaseDTO) {
    setClaseSeleccionada(clase)
    setModalAbierto(true)
  }

  async function handleConfirmar() {
    if (!claseSeleccionada) return
    setLoadingPago(true)
    try {
      const data = await pagosService.crearPago({
        titulo: `Reserva: ${claseSeleccionada.descripcion}`,
        monto: 5000,
        fecha: new Date().toISOString().split('T')[0],
        hora: claseSeleccionada.horario,
        sena: false,
        id_membresia: '',
        reserva_paga: '',
      })
      window.location.href = data.sandbox_init_point
    } catch {
      // TODO: mostrar error en el modal
    } finally {
      setLoadingPago(false)
    }
  }

  function handleCancelarModal() {
    setModalAbierto(false)
    setClaseSeleccionada(null)
  }

  async function handleCancelar(idClase: string) {
    setReservadas((prev) => {
      const next = new Set(prev)
      next.delete(idClase)
      return next
    })
  }

  async function handleListaEspera(_clase: ClaseDTO) {
    // TODO: implementar cuando el back tenga el endpoint de lista de espera
  }

  // Agrupar clases por actividad
  const clasesPorActividad = clases.reduce((acc, clase) => {
    const nombre = getNombreActividad(clase.id_actividad)
    if (!acc[nombre]) acc[nombre] = []
    acc[nombre].push(clase)
    return acc
  }, {} as Record<string, ClaseDTO[]>)

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
          <div className="space-y-8">
            {Object.entries(clasesPorActividad).map(([nombreActividad, clasesDelGrupo]) => (
              <div key={nombreActividad}>
                <h2 className="text-lg font-semibold text-primary capitalize mb-3">{nombreActividad}</h2>
                <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
                  {clasesDelGrupo.map((clase) => (
                    <ClaseCardCliente
                      key={clase.id_clase}
                      idClase={clase.id_clase}
                      dia={clase.dia}
                      diaSemana={clase.dia_semana}
                      horario={clase.horario}
                      descripcion={clase.descripcion}
                      lleno={clase.lleno}
                      estadoReserva={getEstadoReserva(clase)}
                      idActividad={getNombreActividad(clase.id_actividad)}
                      idSala={clase.id_sala}
                      dniProfesor={clase.dni_profesor}
                      nombreProfesor={getNombreProfesor(clase.dni_profesor)}
                      onReservar={() => handleReservar(clase)}
                      onCancelar={() => handleCancelar(clase.id_clase)}
                      onListaEspera={() => handleListaEspera(clase)}
                    />
                  ))}
                </div>
              </div>
            ))}
          </div>
        )}

        <ReservaModal
          clase={claseSeleccionada}
          abierto={modalAbierto}
          onCancelar={handleCancelarModal}
          onConfirmar={handleConfirmar}
          loading={loadingPago}
        />
      </main>
    </div>
  )
}