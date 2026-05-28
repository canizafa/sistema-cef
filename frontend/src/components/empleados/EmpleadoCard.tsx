import { Mail, IdCard, User } from 'lucide-react'
import { Badge } from '@/components/ui/badge'
import { Button } from '@/components/ui/button'
import { Card, CardContent, CardFooter, CardHeader } from '@/components/ui/card'

type EstadoEmpleado = 'alta' | 'baja'
type RolEmpleado = 'duenio' | 'empleado' | 'profesor'

interface EmpleadoCardProps {
  dni: number
  nombreApellido: string
  mail: string
  genero: string
  estado: EstadoEmpleado
  rol: RolEmpleado
  onEditar?: () => void
  onDesactivar?: () => void
  onActivar?: () => void
}

export function EmpleadoCard({
  dni,
  nombreApellido,
  mail,
  genero,
  estado,
  rol,
  onEditar,
  onDesactivar,
  onActivar,
}: EmpleadoCardProps) {
  return (
    <Card className="bg-surface border-border relative">
      <CardHeader className="pb-2">
        <div className="flex items-start justify-between">
          <div>
            <h3 className="text-xl font-semibold text-primary">{nombreApellido}</h3>
            <p className="text-sm text-muted">DNI: {dni}</p>
          </div>
          {estado === 'alta' ? (
            <Badge className="bg-success text-white">Activo</Badge>
          ) : (
            <Badge variant="outline">Inactivo</Badge>
          )}
        </div>
        <span className="inline-block mt-2 text-xs font-medium px-2 py-0.5 rounded border border-brand text-brand bg-brand/10">
          {rol}
        </span>
      </CardHeader>

      <CardContent className="space-y-1.5 pb-3">
        <div className="flex items-center gap-2 text-sm">
          <Mail className="w-4 h-4 text-destructive" />
          <span className="font-medium text-destructive">Mail:</span>
          <span className="text-gray-700">{mail}</span>
        </div>
        <div className="flex items-center gap-2 text-sm">
          <User className="w-4 h-4 text-destructive" />
          <span className="font-medium text-destructive">Género:</span>
          <span className="text-gray-700">{genero}</span>
        </div>
        <div className="flex items-center gap-2 text-sm">
          <IdCard className="w-4 h-4 text-destructive" />
          <span className="font-medium text-destructive">DNI:</span>
          <span className="text-gray-700">{dni}</span>
        </div>
      </CardContent>

      <CardFooter className="gap-2 pt-0">
        <Button variant="outline" size="sm" className="flex-1" onClick={onEditar}>
          Editar
        </Button>
        {estado === 'alta' ? (
          <Button variant="destructive" size="sm" className="flex-1" onClick={onDesactivar}>
            Desactivar
          </Button>
        ) : (
          <Button variant="default" size="sm" className="flex-1" onClick={onActivar}>
            Activar
          </Button>
        )}
      </CardFooter>
    </Card>
  )
}