import { Clock, Calendar } from 'lucide-react'
import { Badge } from '@/components/ui/badge'
import { Button } from '@/components/ui/button'
import { Card, CardContent, CardFooter, CardHeader } from '@/components/ui/card'

export type EstadoReserva = 'disponible' | 'sin-cupo' | 'reservada'

interface ClaseCardClienteProps {
  idClase: string
  dia: string
  horario: string
  descripcion: string
  estadoReserva: EstadoReserva
  onReservar?: () => void
  onCancelar?: () => void
  onListaEspera?: () => void
}

function getBadge(estado: EstadoReserva) {
  switch (estado) {
    case 'disponible':
      return <Badge className="bg-success text-white">Disponible</Badge>
    case 'sin-cupo':
      return <Badge className="bg-destructive text-white">Sin cupo</Badge>
    case 'reservada':
      return <Badge className="bg-brand text-white">Ya reservada</Badge>
  }
}

export function ClaseCardCliente({
  dia,
  horario,
  descripcion,
  estadoReserva,
  onReservar,
  onCancelar,
  onListaEspera,
}: ClaseCardClienteProps) {
  return (
    <Card className="bg-surface border-border">
      <CardHeader className="pb-2">
        <div className="flex items-start justify-between">
          <p className="text-sm font-medium text-primary">{descripcion}</p>
          {getBadge(estadoReserva)}
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
      </CardContent>

      <CardFooter className="gap-2 pt-0">
        {estadoReserva === 'disponible' && (
          <Button variant="default" size="sm" className="flex-1" onClick={onReservar}>
            Reservar clase
          </Button>
        )}
        {estadoReserva === 'sin-cupo' && (
          <>
            <Button variant="outline" size="sm" className="flex-1" disabled>
              Sin cupo
            </Button>
            <Button variant="outline" size="sm" className="flex-1" onClick={onListaEspera}>
              Lista de espera
            </Button>
          </>
        )}
        {estadoReserva === 'reservada' && (
          <Button variant="destructive" size="sm" className="flex-1" onClick={onCancelar}>
            Cancelar reserva
          </Button>
        )}
      </CardFooter>
    </Card>
  )
}