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

const PRECIO_CLASE = 5000

interface ReservaModalProps {
  clase: ClaseDTO | null
  abierto: boolean
  onCancelar: () => void
  onConfirmar: () => void
  loading?: boolean
  creditos?: number
}

export function ReservaModal({ clase, abierto, onCancelar, onConfirmar, loading, creditos = 0 }: ReservaModalProps) {
  const montoPago = Math.max(0, PRECIO_CLASE - creditos)
  const esGratis = creditos >= PRECIO_CLASE
  const tieneCreditos = creditos > 0

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

            {tieneCreditos && (
              <div className="rounded-md bg-green-50 border border-green-200 px-3 py-2 text-sm text-green-800 space-y-0.5">
                <p>Créditos disponibles: <span className="font-semibold">${creditos}</span></p>
                {esGratis
                  ? <p>La clase quedará <span className="font-semibold">gratuita</span> con tus créditos.</p>
                  : <p>Se descontarán <span className="font-semibold">${creditos}</span>. Pagás <span className="font-semibold">${montoPago}</span> con Mercado Pago.</p>
                }
              </div>
            )}
          </div>
        )}

        <DialogFooter>
          <Button variant="outline" onClick={onCancelar}>
            Cancelar
          </Button>
          <Button variant="default" onClick={onConfirmar} disabled={loading}>
            {loading
              ? 'Procesando...'
              : esGratis
                ? 'Confirmar (sin costo)'
                : montoPago < PRECIO_CLASE
                  ? `Pagar $${montoPago} con Mercado Pago`
                  : 'Pagar con Mercado Pago'
            }
          </Button>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  )
}
