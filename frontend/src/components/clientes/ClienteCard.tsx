import { Mail, IdCard, X, Check, Trash2, Bell } from 'lucide-react'
import { Badge } from '@/components/ui/badge'
import { Button } from '@/components/ui/button'
import { Card, CardContent, CardFooter, CardHeader, CardTitle } from '@/components/ui/card'
import { toast } from 'sonner'

export type EstadoCuenta = 'alta' | 'baja' | 'eliminado'
export type EstadoMembresia = 'activa' | 'vencida' | 'sin-membresia'

interface ClienteCardProps {
  dni: number
  nombreApellido: string
  email: string
  estadoCuenta: EstadoCuenta
  estadoMembresia?: EstadoMembresia
  motivoEliminacion?: string | null
  onToggleEstado?: () => void
  onEliminar?: () => void
}

function getBadgeCuenta(estado: EstadoCuenta) {
  switch (estado) {
    case 'alta':
      return <Badge className="bg-success text-white">Activo</Badge>
    case 'baja':
      return <Badge className="bg-gray-400 text-white">Inactivo</Badge>
    case 'eliminado':
      return <Badge className="bg-red-700 text-white">Eliminado</Badge>
  }
}

function getBadgeMembresia(estado?: EstadoMembresia) {
  switch (estado) {
    case 'activa':
      return <Badge className="bg-success text-white">Membresía al día</Badge>
    case 'vencida':
      return <Badge className="bg-red-700 text-white">Membresía vencida</Badge>
    case 'sin-membresia':
      return <Badge className="bg-gray-400 text-white">Sin membresía</Badge>
    default:
      return null
  }
}

export function ClienteCard({
  dni,
  nombreApellido,
  email,
  estadoCuenta,
  estadoMembresia,
  motivoEliminacion,
  onToggleEstado,
  onEliminar,
}: ClienteCardProps) {
  const activo = estadoCuenta === 'alta'

  const handleEnviarNotificacion = () => {
    toast.success("Notificación enviada con éxito")
  }

  return (
    <Card className="bg-surface border-border">
      <CardHeader className="pb-1">
        <div className="flex items-center justify-between gap-2">
          <CardTitle className="text-base font-semibold text-primary">{nombreApellido}</CardTitle>
          {getBadgeCuenta(estadoCuenta)}
        </div>
        {estadoMembresia && (
          <div className="pt-1">
            {getBadgeMembresia(estadoMembresia)}
          </div>
        )}
      </CardHeader>

      <CardContent className="space-y-1 pb-3">
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
        {estadoCuenta === 'eliminado' && motivoEliminacion && (
          <div className="flex items-start gap-2 text-sm pt-1">
            <span className="font-medium text-destructive">Motivo:</span>
            <span className="text-gray-700">{motivoEliminacion}</span>
          </div>
        )}
      </CardContent>

      {estadoCuenta !== 'eliminado' && (
        <CardFooter className="flex-col gap-2 pt-0 border-none">
          {activo ? (
            <Button
              variant="outline"
              size="sm"
              className="w-full border-destructive/40 text-destructive bg-destructive/10 hover:bg-destructive/20"
              onClick={onToggleEstado}
            >
              <X className="w-4 h-4 mr-2" />
              Desactivar
            </Button>
          ) : (
            <Button
              variant="outline"
              size="sm"
              className="w-full"
              onClick={onToggleEstado}
            >
              <Check className="w-4 h-4 mr-2" />
              Activar
            </Button>
          )}

          <Button
            variant="default"
            size="sm"
            className="w-full"
            onClick={handleEnviarNotificacion}
          >
            <Bell className="w-4 h-4 mr-2" />
            Enviar notificación
          </Button>

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
      )}
    </Card>
  )
}