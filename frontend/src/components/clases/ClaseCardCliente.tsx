// Card visual de una clase individual.
// Muestra nombre, tipo, horario, cupo disponible y profesor. El botón cambia según el estado:
// "Reservar" si hay cupo, "Sin cupo" si está llena, "Ya reservada" si el usuario ya la reservó.
import { Clock, Users, Zap } from 'lucide-react'
import { Badge } from '@/components/ui/badge'
import { Button } from '@/components/ui/button'
import { Card, CardContent, CardFooter, CardHeader } from '@/components/ui/card'
 
type EstadoReserva = 'disponible' | 'sin-cupo' | 'reservada'
 
interface ClaseCardClienteProps {
  nombre: string
  profesor: string
  tipo: string
  horario: string
  duracionMin: number
  cupoActual: number
  cupoMaximo: number
  estado: EstadoReserva
  onReservar?: () => void
  onCancelar?: () => void
  onListaEspera?: () => void
  onVerDetalles?: () => void
}
 
function getBadge(estado: EstadoReserva) {
  switch (estado) {
    case 'disponible':
      return <Badge className="bg-success text-white">Activa</Badge>
    case 'sin-cupo':
      return <Badge className="bg-destructive text-white">Sin cupo</Badge>
    case 'reservada':
      return <Badge className="bg-brand text-white">Ya reservada</Badge>
  }
}
 
function getCupoColor(porcentaje: number) {
  if (porcentaje >= 100) return 'bg-destructive'
  if (porcentaje >= 70) return 'bg-warning'
  return 'bg-brand'
}
 
export function ClaseCardCliente({
  nombre,
  profesor,
  tipo,
  horario,
  duracionMin,
  cupoActual,
  cupoMaximo,
  estado,
  onReservar,
  onCancelar,
  onListaEspera,
  onVerDetalles,
}: ClaseCardClienteProps) {
  const porcentaje = Math.round((cupoActual / cupoMaximo) * 100)
  const lugares = cupoMaximo - cupoActual
 
  return (
    <Card className="bg-surface border-border relative">
      <CardHeader className="pb-2">
        <div className="flex items-start justify-between">
          <div>
            <h3 className="text-xl font-semibold text-primary">{nombre}</h3>
            <p className="text-sm text-muted">{profesor}</p>
          </div>
          {getBadge(estado)}
        </div>
        <span className="inline-block mt-2 text-xs font-medium px-2 py-0.5 rounded border border-brand text-brand bg-brand/10">
          {tipo}
        </span>
      </CardHeader>
 
      <CardContent className="space-y-3 pb-3">
        <div className="space-y-1.5">
          <div className="flex items-center gap-2 text-sm text-primary">
            <Clock className="w-4 h-4 text-destructive" />
            {horario}
          </div>
          <div className="flex items-center gap-2 text-sm text-primary">
            <Users className="w-4 h-4 text-destructive" />
            {cupoActual} / {cupoMaximo} personas
          </div>
          <div className="flex items-center gap-2 text-sm text-primary">
            <Zap className="w-4 h-4 text-destructive" />
            Duración: {duracionMin} min
          </div>
        </div>
 
        <div className="space-y-1">
          <div className="flex justify-between text-xs text-muted">
            <span>{estado === 'sin-cupo' ? 'Cupo lleno' : 'Cupo disponible'}</span>
            <span>{lugares} lugares</span>
          </div>
          <div className="h-1 w-full rounded-full bg-border overflow-hidden">
            <div
              className={`h-full rounded-full transition-all ${getCupoColor(porcentaje)}`}
              style={{ width: `${Math.min(porcentaje, 100)}%` }}
            />
          </div>
        </div>
      </CardContent>
 
      <CardFooter className="gap-2 pt-0">
        {estado === 'disponible' && (
          <>
            <Button variant="default" size="sm" className="flex-1" onClick={onReservar}>
              Reservar
            </Button>
            <Button variant="outline" size="sm" className="flex-1" onClick={onVerDetalles}>
              Ver detalles
            </Button>
          </>
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
          <>
            <Button variant="destructive" size="sm" className="flex-1" onClick={onCancelar}>
              Cancelar reserva
            </Button>
            <Button variant="outline" size="sm" className="flex-1" onClick={onVerDetalles}>
              Ver detalles
            </Button>
          </>
        )}
      </CardFooter>
    </Card>
  )
}