import { Mail, IdCard, User } from 'lucide-react'
import { Badge } from '@/components/ui/badge'
import { Button } from '@/components/ui/button'
import { Card, CardContent, CardFooter, CardHeader } from '@/components/ui/card'
 
type EstadoEmpleado = 'activo' | 'inactivo'
type RolEmpleado = 'recepcionista' | 'dueño'
 
interface EmpleadoCardProps {
  dni: number
  nombreApellido: string
  mail: string
  genero: string
  estado: EstadoEmpleado
  rol: RolEmpleado
  onEditar?: () => void
  onDesactivar?: () => void
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
}: EmpleadoCardProps) {
  return (
    <Card className="bg-surface border-border relative">
      <CardHeader className="pb-2">
        <div className="flex items-start justify-between">
          <div>
            <h3 className="text-xl font-semibold text-primary">{nombreApellido}</h3>
            <p className="text-sm text-muted">DNI: {dni}</p>
          </div>
          {estado === 'activo' ? (
            <Badge className="bg-success text-white">Activo</Badge>
          ) : (
            <Badge variant="outline">Inactivo</Badge>
          )}
        </div>
        {/* El rol se muestra como tag igual que el tipo en ClaseCardRecepcionista */}
        <span className="inline-block mt-2 text-xs font-medium px-2 py-0.5 rounded border border-brand text-brand bg-brand/10">
          {rol}
        </span>
      </CardHeader>
 
      <CardContent className="space-y-1.5 pb-3">
        <div className="flex items-center gap-2 text-sm text-primary">
          <Mail className="w-4 h-4 text-destructive" />
          {mail}
        </div>
        <div className="flex items-center gap-2 text-sm text-primary">
          <User className="w-4 h-4 text-destructive" />
          {genero}
        </div>
        <div className="flex items-center gap-2 text-sm text-primary">
          <IdCard className="w-4 h-4 text-destructive" />
          DNI: {dni}
        </div>
      </CardContent>
 
      <CardFooter className="gap-2 pt-0">
        <Button variant="outline" size="sm" className="flex-1" onClick={onEditar}>
          Editar
        </Button>
        <Button variant="destructive" size="sm" className="flex-1" onClick={onDesactivar}>
          Desactivar
        </Button>
      </CardFooter>
    </Card>
  )
}