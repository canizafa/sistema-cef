import { Mail, IdCard, X, Check, Trash2 } from 'lucide-react'
import { Badge } from '@/components/ui/badge'
import { Button } from '@/components/ui/button'
import { Card, CardContent, CardFooter, CardHeader, CardTitle } from '@/components/ui/card'

export type EstadoCuenta = 'activo' | 'inactivo'
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
      return <Badge className="bg-gray-400 text-white">Inactivo</Badge>
  }
}

function getBadgeMembresia(estado: EstadoMembresia) {
  switch (estado) {
    case 'vigente':
      return <Badge className="bg-success text-white">Membresía vigente</Badge>
    case 'vencida':
      return <Badge className="bg-destructive text-white">Membresía vencida</Badge>
    case 'sin-membresia':
      return <Badge className="bg-gray-500 text-white">Sin membresía</Badge>
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

      <CardFooter className="flex-col gap-2 pt-0 border-none">
        <div className="flex gap-2 w-full">
          <Button variant="outline" size="sm" className="flex-1" onClick={onEditar}>
            Editar cliente
          </Button>
          {activo ? (
            <Button
              variant="outline"
              size="sm"
              className="flex-1 border-destructive/40 text-destructive bg-destructive/10 hover:bg-destructive/20"
              onClick={onToggleEstado}
            >
              <X className="w-4 h-4 mr-2" />
              Desactivar
            </Button>
          ) : (
            <Button
              variant="outline"
              size="sm"
              className="flex-1"
              onClick={onToggleEstado}
            >
              <Check className="w-4 h-4 mr-2" />
              Activar
            </Button>
          )}
        </div>
        <Button
          variant="destructive"
          size="sm"
          className="w-full"
          onClick={onEliminar}
        >
          <Trash2 className="w-4 h-4 mr-2" />
          Eliminar cliente
        </Button>
      </CardFooter>
    </Card>
  )
}