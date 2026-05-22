import { Clock, Users, Zap } from 'lucide-react'
import { Badge } from '@/components/ui/badge'
import { Button } from '@/components/ui/button'
import { Card, CardContent, CardFooter, CardHeader } from '@/components/ui/card'

type EstadoClase = 'activa' | 'inactiva'

interface ClaseCardRecepcionistaProps {
  nombre: string
  profesor: string
  tipo: string
  horario: string
  duracionMin: number
  cupoMaximo: number
  estado: EstadoClase
  onEditar?: () => void
  onVerReservas?: () => void
}

export function ClaseCardRecepcionista({
  nombre,
  profesor,
  tipo,
  horario,
  duracionMin,
  cupoMaximo,
  estado,
  onEditar,
  onVerReservas,
}: ClaseCardRecepcionistaProps) {
  return (
    <Card className="bg-surface border-border relative">
      <CardHeader className="pb-2">
        <div className="flex items-start justify-between">
          <div>
            <h3 className="text-xl font-semibold text-primary">{nombre}</h3>
            <p className="text-sm text-muted">{profesor}</p>
          </div>
          {estado === 'activa' ? (
            <Badge className="bg-success text-white">Activa</Badge>
          ) : (
            <Badge variant="outline">Inactiva</Badge>
          )}
        </div>
        <span className="inline-block mt-2 text-xs font-medium px-2 py-0.5 rounded border border-brand text-brand bg-brand/10">
          {tipo}
        </span>
      </CardHeader>

      <CardContent className="space-y-1.5 pb-3">
        <div className="flex items-center gap-2 text-sm text-primary">
          <Clock className="w-4 h-4 text-destructive" />
          {horario}
        </div>
        <div className="flex items-center gap-2 text-sm text-primary">
          <Users className="w-4 h-4 text-destructive" />
          Cupo máximo: {cupoMaximo} personas
        </div>
        <div className="flex items-center gap-2 text-sm text-primary">
          <Zap className="w-4 h-4 text-destructive" />
          Duración: {duracionMin} min
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