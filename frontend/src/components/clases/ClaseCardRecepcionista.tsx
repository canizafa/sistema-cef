import { Clock, Calendar, Users } from 'lucide-react'
import { Badge } from '@/components/ui/badge'
import { Button } from '@/components/ui/button'
import { Card, CardContent, CardFooter, CardHeader } from '@/components/ui/card'
import type { EstadoClase } from '@/services/clases.service'

interface ClaseCardRecepcionistaProps {
  idClase: string
  dia: string
  horario: string
  estado: EstadoClase
  descripcion: string
  lleno: boolean
  onEditar?: () => void
  onVerReservas?: () => void
}

export function ClaseCardRecepcionista({
  dia,
  horario,
  estado,
  descripcion,
  lleno,
  onEditar,
  onVerReservas,
}: ClaseCardRecepcionistaProps) {
  return (
    <Card className="bg-surface border-border">
      <CardHeader className="pb-2">
        <div className="flex items-start justify-between">
          <p className="text-sm font-medium text-primary">{descripcion}</p>
          {estado === 'alta' ? (
            <Badge className="bg-success text-white">Alta</Badge>
          ) : (
            <Badge variant="outline">Baja</Badge>
          )}
        </div>
      </CardHeader>

      <CardContent className="space-y-1.5 pb-3">
        <div className="flex items-center gap-2 text-sm text-primary">
          <Calendar className="w-4 h-4 text-destructive" />
          {dia}
        </div>
        <div className="flex items-center gap-2 text-sm text-primary">
          <Clock className="w-4 h-4 text-destructive" />
          {horario}
        </div>
        <div className="flex items-center gap-2 text-sm text-primary">
          <Users className="w-4 h-4 text-destructive" />
          {lleno ? 'Sin lugares' : 'Con lugares'}
        </div>
      </CardContent>

      <CardFooter className="gap-2 pt-0">
        <Button variant="outline" size="sm" className="flex-1" onClick={onEditar}>
          Editar
        </Button>
        <Button variant="default" size="sm" className="flex-1" onClick={onVerReservas}>
          Ver reservas
        </Button>
      </CardFooter>
    </Card>
  )
}