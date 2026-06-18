import { Mail, IdCard, X, Check, Trash2 } from 'lucide-react'
import { Badge } from '@/components/ui/badge'
import { Button } from '@/components/ui/button'
import { Card, CardContent, CardFooter, CardHeader, CardTitle } from '@/components/ui/card'

export type EstadoCuenta = 'alta' | 'baja' | 'eliminado'

interface ClienteCardProps {
  dni: number
  nombreApellido: string
  email: string
  estadoCuenta: EstadoCuenta
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

export function ClienteCard({
  dni,
  nombreApellido,
  email,
  estadoCuenta,
  motivoEliminacion,
  onToggleEstado,
  onEliminar,
}: ClienteCardProps) {
  const activo = estadoCuenta === 'alta'

  return (
    <Card className="bg-surface border-border">
      <CardHeader className="pb-1">
        <div className="flex items-center justify-between gap-2">
          <CardTitle className="text-base font-semibold text-primary">{nombreApellido}</CardTitle>
          {getBadgeCuenta(estadoCuenta)}
        </div>
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