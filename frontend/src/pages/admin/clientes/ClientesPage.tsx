import { useState } from 'react'
import { Search } from 'lucide-react'
import { ClienteCard } from '@/components/clientes/ClienteCard'

type EstadoCuenta = 'activo' | 'inactivo'
type EstadoMembresia = 'vigente' | 'vencida' | 'sin-membresia'
type FiltroCliente = 'todos' | 'activo' | 'inactivo'

interface Cliente {
  dni: number
  nombreApellido: string
  email: string
  estadoCuenta: EstadoCuenta
  estadoMembresia: EstadoMembresia
}

const CLIENTES_EJEMPLO: Cliente[] = [
  { dni: 40112233, nombreApellido: 'Lola López',       email: 'lolalopez@gmail.com',  estadoCuenta: 'activo',   estadoMembresia: 'vigente' },
  { dni: 38554120, nombreApellido: 'Sebastián Juárez', email: 'sebajuarez@gmail.com', estadoCuenta: 'activo',   estadoMembresia: 'vencida' },
  { dni: 35221890, nombreApellido: 'Carlos Díaz',      email: 'cdiaz@hotmail.com',    estadoCuenta: 'activo',   estadoMembresia: 'sin-membresia' },
  { dni: 41330775, nombreApellido: 'Martina Ruiz',     email: 'mruiz@gmail.com',      estadoCuenta: 'inactivo', estadoMembresia: 'sin-membresia' },
]

const normalizar = (texto: string) =>
  texto.toLowerCase().normalize('NFD').replace(/[\u0300-\u036f]/g, '')

export function ClientesPage() {
  const [filtro, setFiltro] = useState<FiltroCliente>('todos')
  const [busquedaNombre, setBusquedaNombre] = useState('')

  const clientesFiltrados = CLIENTES_EJEMPLO.filter((c) => {
    const matchEstado =
      filtro === 'activo' ? c.estadoCuenta === 'activo' :
      filtro === 'inactivo' ? c.estadoCuenta === 'inactivo' :
      true

    if (busquedaNombre.trim() === '') {
      return matchEstado
    }

    const matchNombre = normalizar(c.nombreApellido)
      .startsWith(normalizar(busquedaNombre.trim()))

    return matchEstado && matchNombre
  })

  const mensajeVacio = () => {
    if (busquedaNombre.trim() !== '') {
      if (filtro === 'activo') return 'No existe un cliente activo con ese nombre y apellido.'
      if (filtro === 'inactivo') return 'No existe un cliente inactivo con ese nombre y apellido.'
      return 'No existe un cliente con ese nombre y apellido.'
    }
    if (filtro === 'activo') return 'No existen clientes activos en el sistema.'
    if (filtro === 'inactivo') return 'No existen clientes inactivos en el sistema.'
    return 'No hay clientes registrados en el sistema.'
  }

  const tabs: { label: string; value: FiltroCliente }[] = [
    { label: 'Todos', value: 'todos' },
    { label: 'Activos', value: 'activo' },
    { label: 'Inactivos', value: 'inactivo' },
  ]

  return (
    <main className="p-4 md:p-8 bg-background min-h-screen">
      <div className="mb-6">
        <h1 className="text-2xl font-bold text-primary">Gestión de Clientes</h1>
        <p className="text-sm mt-1">Administrá los clientes del centro</p>
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
            className={`px-4 py-1.5 rounded-full text-sm font-medium transition-colors
              ${filtro === tab.value
                ? 'bg-brand text-white'
                : 'bg-border text-gray-500 hover:bg-muted hover:text-white'
              }`}
          >
            {tab.label}
          </button>
        ))}
      </div>

      {clientesFiltrados.length === 0 && (
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
            estadoMembresia={c.estadoMembresia}
            onEditar={() => console.log('editar', c.dni)}
            onToggleEstado={() => console.log('toggle estado', c.dni)}
            onEliminar={() => console.log('eliminar', c.dni)}
          />
        ))}
      </div>
    </main>
  )
}