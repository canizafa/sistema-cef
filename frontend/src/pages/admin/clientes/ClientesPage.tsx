import { ClienteCard } from '@/components/clientes/ClienteCard'

// PENDIENTE BACK: datos de ejemplo. El estado de membresía todavía
// no lo devuelve el back (no hay tabla de membresías/pagos).
const CLIENTES_EJEMPLO = [
  { dni: 40112233, nombreApellido: 'Lola López',       email: 'lolalopez@gmail.com',  estadoCuenta: 'activo',   estadoMembresia: 'vigente' },
  { dni: 38554120, nombreApellido: 'Sebastián Juárez', email: 'sebajuarez@gmail.com', estadoCuenta: 'activo',   estadoMembresia: 'vencida' },
  { dni: 35221890, nombreApellido: 'Carlos Díaz',      email: 'cdiaz@hotmail.com',    estadoCuenta: 'activo',   estadoMembresia: 'sin-membresia' },
  { dni: 41330775, nombreApellido: 'Martina Ruiz',     email: 'mruiz@gmail.com',      estadoCuenta: 'inactivo', estadoMembresia: 'sin-membresia' },
] as const

export function ClientesPage() {
  return (
    <div className="p-8">
      <h1 className="text-2xl font-bold text-primary mb-6">Gestión de Clientes</h1>

      <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
        {CLIENTES_EJEMPLO.map((c) => (
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
    </div>
  )
}