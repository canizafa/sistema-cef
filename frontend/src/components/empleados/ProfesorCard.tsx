import { IdCard, X, Check, Trash2 } from 'lucide-react'
import { Badge } from '@/components/ui/badge'
import { Button } from '@/components/ui/button'
import { Card, CardContent, CardFooter, CardHeader, CardTitle, CardDescription } from '@/components/ui/card'

type EstadoProfesor = 'alta' | 'baja' | 'eliminado'

interface ProfesorCardProps {
  dni: number
  nombreCompleto: string
  estado: EstadoProfesor
  onEditar?: () => void
  onDesactivar?: () => void
  onActivar?: () => void
  onEliminar?: () => void
}

export function ProfesorCard({
  dni,
  nombreCompleto,
  estado,
  onEditar,
  onDesactivar,
  onActivar,
  onEliminar,
}: ProfesorCardProps) {
  return (
    <Card className="bg-surface border-border">
      <CardHeader className="pb-2">
        <div className="flex items-start justify-between gap-2">
          <div>
            <CardTitle className="text-base font-semibold text-primary">{nombreCompleto}</CardTitle>
            <CardDescription className="text-sm text-gray-500 mt-0.5">Profesor</CardDescription>
          </div>
          {estado === 'alta' ? (
            <Badge className="bg-success text-white">Activo</Badge>
          ) : estado === 'baja' ? (
            <Badge className="bg-gray-400 text-white">Inactivo</Badge>
          ) : (
            <Badge className="bg-red-700 text-white">Eliminado</Badge>
          )}
        </div>
      </CardHeader>

      <CardContent className="space-y-1.5 pb-3">
        <div className="flex items-center gap-2 text-sm">
          <IdCard className="w-4 h-4 text-destructive" />
          <span className="font-medium text-destructive">DNI:</span>
          <span className="text-gray-700">{dni}</span>
        </div>
      </CardContent>

      <CardFooter className="flex flex-col gap-2 pt-0 border-none">
        {estado !== 'eliminado' && (
          <div className="flex gap-2 w-full">
            <Button variant="outline" size="sm" className="flex-1" onClick={onEditar}>
              Editar
            </Button>
            {estado === 'alta' ? (
              <Button
                variant="outline"
                size="sm"
                className="flex-1 border-destructive/40 text-destructive bg-destructive/10 hover:bg-destructive/20"
                onClick={onDesactivar}
              >
                <X className="w-4 h-4 mr-2" />
                Desactivar
              </Button>
            ) : (
              <Button
                variant="outline"
                size="sm"
                className="flex-1"
                onClick={onActivar}
              >
                <Check className="w-4 h-4 mr-2" />
                Activar
              </Button>
            )}
          </div>
        )}
        {estado !== 'eliminado' && (
          <Button
            variant="destructive"
            size="sm"
            className="w-full"
            onClick={onEliminar}
          >
            <Trash2 className="w-4 h-4 mr-2" />
            Eliminar profesor
          </Button>
        )}
      </CardFooter>
    </Card>
  )
}