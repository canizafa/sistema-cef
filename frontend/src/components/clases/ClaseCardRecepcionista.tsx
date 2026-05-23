// Card visual de una clase para el recepcionista.
// Muestra descripcion, dia, horario, sala y cupo maximo. Permite editar y ver reservas.
import { Clock, Users, DoorOpen, Calendar } from 'lucide-react'
import { Badge } from '@/components/ui/badge'
import { Button } from '@/components/ui/button'
import { Card, CardContent, CardFooter, CardHeader } from '@/components/ui/card'

type EstadoClase = 'activa' | 'inactiva'

interface ClaseCardRecepcionistaProps {
  idClase: number
  descripcion: string
  dia: string
  horario: string
  idSala: number
  cupoMaximo: number
  estado: EstadoClase
  onEditar?: () => void
  onVerReservas?: () => void
}

export function ClaseCardRecepcionista({
  descripcion,
  dia,
  horario,
  idSala,
  cupoMaximo,
  estado,
  onEditar,
  onVerReservas,
}: ClaseCardRecepcionistaProps) {
  return (
    <Card className="bg-surface border-border relative">
      <CardHeader className="pb-2">
        <div className="flex items-start justify-between">
          <h3 className="text-xl font-semibold text-primary">{descripcion}</h3>
          {estado === 'activa' ? (
            <Badge className="bg-success text-white">Activa</Badge>
          ) : (
            <Badge variant="outline">Inactiva</Badge>
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
          <DoorOpen className="w-4 h-4 text-destructive" />
          Sala: {idSala}
        </div>
        <div className="flex items-center gap-2 text-sm text-primary">
          <Users className="w-4 h-4 text-destructive" />
          Cupo máximo: {cupoMaximo} personas
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