import { Mail, IdCard } from 'lucide-react'
import { Badge } from '@/components/ui/badge'
import { Button } from '@/components/ui/button'
import { Card, CardContent, CardFooter, CardHeader, CardTitle } from '@/components/ui/card'

export type EstadoCuenta = 'activo' | 'inactivo'
// PENDIENTE BACK: el back todavía no devuelve estado de membresía
// (no hay tabla de membresías/pagos). Por ahora se pasa por props desde mock.
export type EstadoMembresia = 'vigente' | 'vencida' | 'sin-membresia'

interface ClienteCardProps {
  dni: number
  nombreApellido: string
  email: string
  estadoCuenta: EstadoCuenta
  estadoMembresia: EstadoMembresia
  onEditar?: () => void
  onToggleEstado?: () => void
  onEliminar?: () => void
}

function getBadgeCuenta(estado: EstadoCuenta) {
  switch (estado) {
    case 'activo':
      return <Badge className="bg-success text-white">Activo</Badge>
    case 'inactivo':
      return <Badge variant="outline" className="text-gray-500">Inactivo</Badge>
  }
}

function getBadgeMembresia(estado: EstadoMembresia) {
  switch (estado) {
    case 'vigente':
      return <Badge className="bg-success text-white">Membresía vigente</Badge>
    case 'vencida':
      return <Badge className="bg-destructive text-white">Membresía vencida</Badge>
    case 'sin-membresia':
      return <Badge variant="outline" className="text-gray-500">Sin membresía</Badge>
  }
}

export function ClienteCard({
  dni,
  nombreApellido,
  email,
  estadoCuenta,
  estadoMembresia,
  onEditar,
  onToggleEstado,
  onEliminar,
}: ClienteCardProps) {
  const activo = estadoCuenta === 'activo'

  return (
    <Card className="bg-surface border-border">
      <CardHeader className="pb-1">
        <div className="flex items-center justify-between gap-2">
          <CardTitle className="text-base font-semibold text-primary">{nombreApellido}</CardTitle>
          {getBadgeCuenta(estadoCuenta)}
        </div>
      </CardHeader>

      <CardContent className="space-y-1 pb-3">
        <div className="mb-1.5">{getBadgeMembresia(estadoMembresia)}</div>
        <div className="flex items-center gap-2 text-sm">
          <Mail className="w-4 h-4 text-destructive" />
          <span className="font-medium text-destructive">Mail:</span>
          <span className="text-gray-700 truncate">{email}</span>
        </div>
        <div className="flex items-center gap-2 text-sm">
          <IdCard className="w-4 h-4 text-destructive" />
          <span className="font-medium text-destructive">DNI:</span>
          <span className="text-gray-700">{dni.toLocaleString('es-AR')}</span>
        </div>
      </CardContent>

      <CardFooter className="flex-col gap-2 pt-0">
        <div className="flex gap-2 w-full">
          <Button variant="outline" size="sm" className="flex-1" onClick={onEditar}>
            Editar cliente
          </Button>
          {activo ? (
            <Button variant="outline" size="sm" className="flex-1" onClick={onToggleEstado}>
              Desactivar
            </Button>
          ) : (
            <Button variant="default" size="sm" className="flex-1" onClick={onToggleEstado}>
              Activar
            </Button>
          )}
        </div>
        <Button variant="destructive" size="sm" className="w-full" onClick={onEliminar}>
          Eliminar cliente
        </Button>
      </CardFooter>
    </Card>
  )
}