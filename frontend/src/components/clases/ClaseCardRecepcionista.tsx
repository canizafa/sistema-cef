import { Clock, Calendar, Users, Dumbbell, DoorOpen, UserRound } from 'lucide-react'
import { Button } from '@/components/ui/button'
import { Card, CardContent, CardFooter, CardHeader, CardTitle, CardDescription } from '@/components/ui/card'

interface ClaseCardRecepcionistaProps {
  idClase: string
  dia: string
  diaSemana: string
  horario: string
  descripcion: string
  cupoBase: number
  inscripciones: number
  idActividad: string
  idSala: string
  dniProfesor: number
  nombreProfesor: string
  onEditar?: () => void
  onEliminar?: () => void
  onVerReservas?: () => void
}

export function ClaseCardRecepcionista({
  dia,
  diaSemana,
  horario,
  descripcion,
  cupoBase,
  inscripciones,
  idActividad,
  idSala,
  dniProfesor,
  nombreProfesor,
  onEditar,
  onEliminar,
}: ClaseCardRecepcionistaProps) {
  return (
    <Card className="bg-surface border-border">
      <CardHeader className="pb-2">
        <div className="flex items-start justify-between gap-2">
          <div>
            <CardTitle className="text-base font-semibold text-primary">{idActividad}</CardTitle>
            <CardDescription className="text-sm text-gray-500 mt-0.5">{descripcion}</CardDescription>
          </div>
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
          <span className="font-medium text-destructive">Lugares disponibles:</span>
          <span className="text-gray-700">{inscripciones}/{cupoBase}</span>
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

      <CardFooter className="flex flex-col gap-2 pt-0">
        <Button variant="outline" size="sm" className="w-full" onClick={onEditar}>
          Editar clase
        </Button>
        <Button variant="destructive" size="sm" className="w-full" onClick={onEliminar}>
          Eliminar clase
        </Button>
      </CardFooter>
    </Card>
  )
}