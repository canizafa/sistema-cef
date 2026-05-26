// Modal de confirmación de reserva.
// Muestra el nombre y horario de la clase seleccionada y pide confirmación antes de hacer el request.
// Existe para evitar reservas accidentales y mostrar los errores del backend (membresía vencida, sin cupo).
import {
  Dialog,
  DialogContent,
  DialogHeader,
  DialogTitle,
  DialogDescription,
  DialogFooter,
} from '@/components/ui/dialog'
import { Button } from '@/components/ui/button'
import { Calendar, Clock } from 'lucide-react'
import type { ClaseDTO } from '@/services/clases.service'

interface ReservaModalProps {
  clase: ClaseDTO | null
  abierto: boolean
  onCancelar: () => void
  onConfirmar: () => void
}

export function ReservaModal({ clase, abierto, onCancelar, onConfirmar }: ReservaModalProps) {
  return (
    <Dialog open={abierto} onOpenChange={onCancelar}>
      <DialogContent className="max-w-md">
        <DialogHeader>
          <DialogTitle>Confirmar reserva</DialogTitle>
          <DialogDescription>
            Revisá los datos de la clase antes de continuar al pago.
          </DialogDescription>
        </DialogHeader>

        {clase && (
          <div className="space-y-3 py-2">
            <p className="text-sm font-medium text-primary">{clase.descripcion}</p>
            <div className="flex items-center gap-2 text-sm text-gray-600">
              <Calendar className="w-4 h-4" />
              {clase.dia}
            </div>
            <div className="flex items-center gap-2 text-sm text-gray-600">
              <Clock className="w-4 h-4" />
              {clase.horario}
            </div>
          </div>
        )}

        <DialogFooter>
          <Button variant="outline" onClick={onCancelar}>
            Cancelar
          </Button>
          <Button variant="default" onClick={onConfirmar}>
            Pagar con Mercado Pago
          </Button>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  )
}