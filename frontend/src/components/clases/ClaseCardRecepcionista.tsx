import { Clock, Calendar, Users, Dumbbell, DoorOpen } from 'lucide-react'
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
  idActividad: string
  idSala: string
  onEditar?: () => void
  onVerReservas?: () => void
}

export function ClaseCardRecepcionista({
  dia,
  horario,
  estado,
  descripcion,
  lleno,
  idActividad,
  idSala,
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
        <div className="flex items-center gap-2 text-sm">
          <Calendar className="w-4 h-4 text-destructive" />
          <span className="font-medium text-destructive">Fecha:</span>
          <span className="text-gray-700">{dia}</span>
        </div>
        <div className="flex items-center gap-2 text-sm">
          <Clock className="w-4 h-4 text-destructive" />
          <span className="font-medium text-destructive">Hora:</span>
          <span className="text-gray-700">{horario}</span>
        </div>
        <div className="flex items-center gap-2 text-sm">
          <Users className="w-4 h-4 text-destructive" />
          <span className="font-medium text-destructive">Disponibilidad:</span>
          <span className="text-gray-700">{lleno ? 'Sin lugares' : 'Con lugares'}</span>
        </div>
        <div className="flex items-center gap-2 text-sm">
          <Dumbbell className="w-4 h-4 text-destructive" />
          <span className="font-medium text-destructive">Actividad:</span>
          <span className="text-gray-700">{idActividad}</span>
        </div>
        <div className="flex items-center gap-2 text-sm">
          <DoorOpen className="w-4 h-4 text-destructive" />
          <span className="font-medium text-destructive">Sala:</span>
          <span className="text-gray-700">{idSala}</span>
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