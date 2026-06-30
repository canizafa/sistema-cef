import { useEffect, useState } from 'react'
import { Search, CalendarDays } from 'lucide-react' // <--- Traemos un icono lindo para programar
import { ClienteCard } from '@/components/clientes/ClienteCard'
import { EliminarClienteModal } from '@/components/clientes/EliminarClienteModal'
import { clienteService, type ClienteResponse } from '@/services/cliente.service'
import { toast } from 'sonner'

type EstadoCuenta = 'alta' | 'baja' | 'eliminado'
type FiltroCliente = 'todos' | 'alta' | 'baja' | 'eliminado'

interface Cliente {
  dni: number
  nombreApellido: string
  email: string
  telefono: string
  fechaNacimiento: string
  idFicha: string
  estadoCuenta: EstadoCuenta
  motivoEliminacion: string | null
}

const normalizar = (texto: string) =>
  texto.toLowerCase().normalize('NFD').replace(/[\u0300-\u036f]/g, '')

export function ClientesPage() {
  const [clientes, setClientes] = useState<Cliente[]>([])
  const [loading, setLoading] = useState(true)
  const [error, setError] = useState<string | null>(null)
  const [filtro, setFiltro] = useState<FiltroCliente>('todos')
  const [busquedaNombre, setBusquedaNombre] = useState('')
  const [modalEliminarAbierto, setModalEliminarAbierto] = useState(false)
  const [clienteAEliminar, setClienteAEliminar] = useState<Cliente | null>(null)
  const [, setLoadingToggle] = useState<number | null>(null)

  useEffect(() => {
    clienteService.getClientes()
      .then((data) => {
        setClientes(data.map((c) => ({
          dni: c.dni,
          nombreApellido: c.nombre_apellido,
          email: c.email,
          telefono: c.telefono,
          fechaNacimiento: c.fecha_nacimiento,
          idFicha: c.id_ficha,
          estadoCuenta: c.motivo_eliminacion ? 'eliminado' : c.estado as EstadoCuenta,
          motivoEliminacion: c.motivo_eliminacion,
        })))
      })
      .catch(() => setError('No se pudieron cargar los clientes'))
      .finally(() => setLoading(false))
  }, [])

  const handleToggleEstado = async (dni: number) => {
    const cliente = clientes.find((c) => c.dni === dni)
    if (!cliente) return
    setLoadingToggle(dni)
    try {
      const clienteRaw: ClienteResponse = {
        dni: cliente.dni,
        nombre_apellido: cliente.nombreApellido,
        email: cliente.email,
        telefono: cliente.telefono,
        fecha_nacimiento: cliente.fechaNacimiento,
        estado: cliente.estadoCuenta,
        rol: 'cliente',
        id_ficha: cliente.idFicha,
        motivo_eliminacion: cliente.motivoEliminacion,
        creditos: 0
      }
      await clienteService.toggleEstado(clienteRaw)
      setClientes((prev) =>
        prev.map((c) =>
          c.dni === dni
            ? { ...c, estadoCuenta: c.estadoCuenta === 'alta' ? 'baja' : 'alta' }
            : c
        )
      )
      toast.success(
        cliente.estadoCuenta === 'alta'
          ? 'Cliente desactivado correctamente'
          : 'Cliente activado correctamente'
      )
    } catch {
      toast.error('No se pudo actualizar el estado del cliente')
    } finally {
      setLoadingToggle(null)
    }
  }

  const handleEliminar = (dni: number) => {
    const cliente = clientes.find((c) => c.dni === dni)
    if (!cliente) return
    setClienteAEliminar(cliente)
    setModalEliminarAbierto(true)
  }

  const handleEliminarConfirmado = () => {
    if (!clienteAEliminar) return
    setClientes((prev) =>
      prev.map((c) =>
        c.dni === clienteAEliminar.dni
          ? { ...c, estadoCuenta: 'eliminado' as EstadoCuenta }
          : c
      )
    )
    setClienteAEliminar(null)
  }

  // Manejador para el nuevo botón global
  const handleProgramarNotificaciones = () => {
    toast.info("Apertura de programación de notificaciones")
  }

  const clientesFiltrados = clientes.filter((c) => {
    if (busquedaNombre.trim() !== '') {
      return normalizar(c.nombreApellido)
        .startsWith(normalizar(busquedaNombre.trim()))
    }
    if (filtro === 'alta') return c.estadoCuenta === 'alta'
    if (filtro === 'baja') return c.estadoCuenta === 'baja'
    if (filtro === 'eliminado') return c.estadoCuenta === 'eliminado'
    return c.estadoCuenta !== 'eliminado'
  })

  const mensajeVacio = () => {
    if (busquedaNombre.trim() !== '') return 'No existe un cliente con ese nombre y apellido.'
    if (filtro === 'todos') return 'No hay clientes registrados en el sistema.'
    return 'No existen clientes con el filtro solicitado.'
  }

  const tabs: { label: string; value: FiltroCliente }[] = [
    { label: 'Todos', value: 'todos' },
    { label: 'Activos', value: 'alta' },
    { label: 'Inactivos', value: 'baja' },
    { label: 'Eliminados', value: 'eliminado' },
  ]

  return (
    <main className="p-4 md:p-8 bg-background min-h-screen">
      {/* SECCIÓN SUPERIOR: Título y Botón alineado a la derecha */}
      <div className="flex flex-col sm:flex-row sm:items-center sm:justify-between gap-4 mb-6">
        <div>
          <h1 className="text-2xl font-bold text-primary">Gestión de Clientes</h1>
          <p className="text-sm mt-1">Administrá los clientes del centro</p>
        </div>
        
        <button
          onClick={handleProgramarNotificaciones}
          className="flex items-center justify-center gap-2 bg-brand text-white text-sm font-medium h-10 px-4 rounded-lg hover:bg-brand/90 transition-colors self-start sm:self-auto"
        >
          <CalendarDays className="w-4 h-4" />
          Programar envío de notificaciones
        </button>
      </div>

      <div className="relative mb-4">
        <Search className="absolute left-3 top-2.5 w-4 h-4 text-gray-500" />
        <input
          type="text"
          placeholder="Buscar cliente por nombre y apellido"
          value={busquedaNombre}
          onChange={(e) => setBusquedaNombre(e.target.value)}
          className="w-full border-2 border-brand rounded-lg pl-9 pr-4 h-10 text-sm bg-background text-primary placeholder:text-gray-400"
        />
      </div>

      <div className="flex gap-2 mb-6">
        {tabs.map((tab) => (
          <button
            key={tab.value}
            onClick={() => setFiltro(tab.value)}
            disabled={busquedaNombre.trim() !== ''}
            className={`px-4 py-1.5 rounded-full text-sm font-medium transition-colors
              ${filtro === tab.value && busquedaNombre.trim() === ''
                ? 'bg-brand text-white'
                : 'bg-border text-gray-500'
              }
              ${busquedaNombre.trim() !== ''
                ? 'opacity-40 cursor-not-allowed'
                : 'hover:bg-muted hover:text-white'
              }`}
          >
            {tab.label}
          </button>
        ))}
      </div>

      {loading && <p className="text-sm text-muted">Cargando clientes...</p>}
      {error && <p className="text-sm text-destructive">{error}</p>}

      {!loading && !error && clientesFiltrados.length === 0 && (
        <p className="text-sm" style={{ color: '#4B5563' }}>{mensajeVacio()}</p>
      )}

      <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
        {clientesFiltrados.map((c) => (
          <ClienteCard
            key={c.dni}
            dni={c.dni}
            nombreApellido={c.nombreApellido}
            email={c.email}
            estadoCuenta={c.estadoCuenta}
            motivoEliminacion={c.motivoEliminacion}
            onToggleEstado={() => handleToggleEstado(c.dni)}
            onEliminar={() => handleEliminar(c.dni)}
          />
        ))}
      </div>

      {clienteAEliminar && (
        <EliminarClienteModal
          open={modalEliminarAbierto}
          onOpenChange={setModalEliminarAbierto}
          cliente={{
            dni: clienteAEliminar.dni,
            nombre_apellido: clienteAEliminar.nombreApellido,
            email: clienteAEliminar.email,
          }}
          onEliminado={handleEliminarConfirmado}
        />
      )}
    </main>
  )
}