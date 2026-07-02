import { useEffect, useState } from 'react'
import { toast } from 'sonner'
import { useAuth } from '@/context/AuthContext'
import { useCreditos } from '@/context/CreditosContext'
import { ClaseCardCliente, type EstadoReserva } from '@/components/clases/ClaseCardCliente'
import { ReservaModal } from '@/components/clases/ReservaModal'
import { Header } from '@/components/layout/Header'
import { clasesService, reservasService, listaEsperaService, type ClaseDTO } from '@/services/clases.service'
import { actividadService, type Actividad } from '@/services/actividad.service'
import { profesorService, type Profesor } from '@/services/profesor.service'
import { pagosService } from '@/services/pagos.service'
import { clienteService } from '@/services/cliente.service'

const PRECIO_CLASE = 5000

export function ClasesPage() {
  const { user } = useAuth()
  const { creditos, refrescarCreditos } = useCreditos()
  const [clases, setClases] = useState<ClaseDTO[]>([])
  const [actividades, setActividades] = useState<Actividad[]>([])
  const [profesores, setProfesores] = useState<Profesor[]>([])
  const [reservadas, setReservadas] = useState<Set<string>>(new Set())
  const [enListaEspera, setEnListaEspera] = useState<Set<string>>(new Set())
  const [loading, setLoading] = useState(true)
  const [error, setError] = useState<string | null>(null)
  const [claseSeleccionada, setClaseSeleccionada] = useState<ClaseDTO | null>(null)
  const [modalAbierto, setModalAbierto] = useState(false)
  const [loadingPago, setLoadingPago] = useState(false)

  useEffect(() => {
    async function cargar() {
      try {
        const [dataClases, dataActividades, dataProfesores, dataReservas] = await Promise.all([
          clasesService.getClases(),
          actividadService.getActividades(),
          profesorService.getProfesores(),
          reservasService.getReservas(),
        ])
        setClases(dataClases)
        setActividades(dataActividades)
        setProfesores(dataProfesores)

        if (user) {
          const reservasActivas = dataReservas.filter(
            (r) => String(r.dni_cliente) === String(user.dni) && r.estado !== 'cancelada'
          )
          setReservadas(new Set(reservasActivas.map((r) => r.id_clase)))
        }
      } catch {
        setError('No se pudieron cargar las clases.')
      } finally {
        setLoading(false)
      }
    }
    cargar()
  }, [user])

  function getNombreActividad(idActividad: string): string {
    return actividades.find((a) => String(a.id) === String(idActividad))?.nombre ?? idActividad
  }

  function getNombreProfesor(dniProfesor: number): string {
    return profesores.find((p) => p.dni === dniProfesor)?.nombre_completo ?? 'Sin asignar'
  }

  function getEstadoReserva(clase: ClaseDTO): EstadoReserva {
    if (reservadas.has(clase.id_clase)) return 'reservada'
    if (clase.inscripciones >= clase.cupo_base) return 'sin-cupo'
    return 'disponible'
  }

  function handleReservar(clase: ClaseDTO) {
    setClaseSeleccionada(clase)
    setModalAbierto(true)
  }

  async function handleConfirmar() {
    if (!claseSeleccionada || !user) return
    setLoadingPago(true)
    try {
      if (creditos >= PRECIO_CLASE) {
        // Caso 1: créditos cubren el total — sin MercadoPago
       await reservasService.crearReserva({
  fecha: claseSeleccionada.dia,
  tipo: 'abono',
  estado: 'confirmada',
  dni_cliente: user.dni,
  id_clase: claseSeleccionada.id_clase,
  horario: claseSeleccionada.horario,
})
        setReservadas((prev) => new Set(prev).add(claseSeleccionada.id_clase))
        handleCancelarModal()
        try {
          await clienteService.usarCreditos(user.dni, PRECIO_CLASE)
          await refrescarCreditos()
          toast.success(`Clase reservada. Se usaron $${PRECIO_CLASE} en créditos.`)
        } catch {
          toast.warning('Clase reservada, pero no se pudo descontar el crédito automáticamente.')
        }
      } else {
        // Caso 2 (créditos parciales) y Caso 3 (sin créditos): MercadoPago
        // El backend descuenta los créditos del monto total internamente (crear_pago_handler),
        // por eso se manda el precio completo y no el ya descontado.
        const data = await pagosService.crearPago({
          titulo: `Reserva: ${claseSeleccionada.descripcion}`,
          monto: PRECIO_CLASE,
          fecha: new Date().toISOString().split('T')[0],
          hora: claseSeleccionada.horario,
          sena: false,
          id_membresia: '',
          dni: user.dni,
          reserva_paga: '',
        })
        localStorage.setItem('pending_reserva', JSON.stringify({
          dni: user.dni,
          idClase: claseSeleccionada.id_clase,
          fecha: claseSeleccionada.dia,
        }))
        window.location.href = data.sandbox_init_point
      }
    } catch {
      toast.error('No se pudo procesar el pago. Intentá de nuevo.')
    } finally {
      setLoadingPago(false)
    }
  }

  function handleCancelarModal() {
    setModalAbierto(false)
    setClaseSeleccionada(null)
  }

  async function handleListaEspera(clase: ClaseDTO) {
    if (!user) return
    try {
      const listas = await listaEsperaService.getAll()
      let lista = listas.find((l) => l.id_clase === clase.id_clase)
      if (!lista) {
        lista = await listaEsperaService.crearLista(clase.id_clase, getNombreActividad(clase.id_actividad))
      }
      await listaEsperaService.anotarse(clase.id_clase, lista.id_espera, user.dni)
      setEnListaEspera((prev) => new Set(prev).add(clase.id_clase))
    } catch {
      toast.error('No se pudo anotar en la lista de espera. Intentá de nuevo.')
    }
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
                      cupoBase={clase.cupo_base}
                      inscripciones={clase.inscripciones}
                      estadoReserva={getEstadoReserva(clase)}
                      idActividad={getNombreActividad(clase.id_actividad)}
                      idSala={clase.id_sala}
                      dniProfesor={clase.dni_profesor}
                      nombreProfesor={getNombreProfesor(clase.dni_profesor)}
                      enListaEspera={enListaEspera.has(clase.id_clase)}
                      onReservar={() => handleReservar(clase)}
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
          creditos={creditos}
        />
      </main>
    </div>
  )
}