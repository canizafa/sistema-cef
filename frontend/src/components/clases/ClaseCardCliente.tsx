// Card visual de una clase individual.
// Muestra descripcion, dia, horario y cupo. El botón cambia según el estado:
// "Reservar" si hay cupo, "Sin cupo" si está llena, "Ya reservada" si el usuario ya la reservó.
import { Clock, Users, Calendar } from 'lucide-react'
import { Badge } from '@/components/ui/badge'
import { Button } from '@/components/ui/button'
import { Card, CardContent, CardFooter, CardHeader } from '@/components/ui/card'

type EstadoReserva = 'disponible' | 'sin-cupo' | 'reservada'

interface ClaseCardClienteProps {
  idClase: number
  descripcion: string
  dia: string
  horario: string
  cupoMaximo: number
  estado: EstadoReserva
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
  descripcion,
  dia,
  horario,
  cupoMaximo,
  estado,
  onReservar,
  onCancelar,
  onListaEspera,
}: ClaseCardClienteProps) {
  return (
    <Card className="bg-surface border-border relative">
      <CardHeader className="pb-2">
        <div className="flex items-start justify-between">
          <h3 className="text-xl font-semibold text-primary">{descripcion}</h3>
          {getBadge(estado)}
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
          Cupo máximo: {cupoMaximo} personas
        </div>
      </CardContent>

      <CardFooter className="gap-2 pt-0">
        {estado === 'disponible' && (
          <Button variant="default" size="sm" className="flex-1" onClick={onReservar}>
            Inscribirse en la clase
          </Button>
        )}

        {estado === 'sin-cupo' && (
          <>
            <Button variant="outline" size="sm" className="flex-1" disabled>
              Sin cupo
            </Button>
            <Button variant="outline" size="sm" className="flex-1" onClick={onListaEspera}>
              Lista de espera
            </Button>
          </>
        )}

        {estado === 'reservada' && (
          <Button variant="destructive" size="sm" className="flex-1" onClick={onCancelar}>
            Cancelar reserva
          </Button>
        )}
      </CardFooter>
    </Card>
  )
}