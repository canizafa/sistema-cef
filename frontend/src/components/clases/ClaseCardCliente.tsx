import { Clock, Calendar, Users, Dumbbell, DoorOpen, UserRound } from 'lucide-react'
import { Badge } from '@/components/ui/badge'
import { Button } from '@/components/ui/button'
import { Card, CardContent, CardFooter, CardHeader, CardTitle, CardDescription } from '@/components/ui/card'

export type EstadoReserva = 'disponible' | 'sin-cupo' | 'reservada'

interface ClaseCardClienteProps {
  idClase: string
  dia: string
  diaSemana: string
  horario: string
  descripcion: string
  lleno: boolean
  estadoReserva: EstadoReserva
  idActividad: string
  idSala: string
  dniProfesor: number
  nombreProfesor: string
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
  diaSemana,
  horario,
  descripcion,
  lleno,
  estadoReserva,
  idActividad,
  idSala,
  dniProfesor,
  nombreProfesor,
  onReservar,
  onCancelar,
  onListaEspera,
}: ClaseCardClienteProps) {
  return (
    <Card className="bg-surface border-border">
      <CardHeader className="pb-2">
        <div className="flex items-start justify-between gap-2">
          <div>
            <CardTitle className="text-base font-semibold text-primary">{idActividad}</CardTitle>
            <CardDescription className="text-sm text-gray-500 mt-0.5">{descripcion}</CardDescription>
          </div>
          {getBadge(estadoReserva)}
        </div>
      </CardHeader>

      <CardContent className="space-y-1.5 pb-3">
        <div className="flex items-center gap-2 text-sm">
          <Calendar className="w-4 h-4 text-destructive" />
          <span className="font-medium text-destructive">Fecha inicio:</span>
          <span className="text-gray-700">{dia ? dia.split('-').reverse().join('/') : '-'}</span>
        </div>
        <div className="flex items-center gap-2 text-sm">
          <Calendar className="w-4 h-4 text-destructive" />
          <span className="font-medium text-destructive">Días de la semana:</span>
          <span className="text-gray-700">{diaSemana}</span>
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
        <div className="flex items-center gap-2 text-sm">
          <UserRound className="w-4 h-4 text-destructive" />
          <span className="font-medium text-destructive">Profesor:</span>
          <span className="text-gray-700">{nombreProfesor} — (DNI: {dniProfesor})</span>
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