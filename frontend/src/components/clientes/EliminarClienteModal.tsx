import { useState } from "react";
import {
  Dialog,
  DialogContent,
  DialogHeader,
  DialogTitle,
  DialogDescription,
  DialogFooter,
} from "@/components/ui/dialog";
import { toast } from "sonner";
import { Mail, IdCard } from "lucide-react";
import { clienteService } from "@/services/cliente.service";

interface EliminarClienteModalProps {
  open: boolean;
  onOpenChange: (open: boolean) => void;
  cliente: {
    dni: number;
    nombre_apellido: string;
    email: string;
  };
  onEliminado?: () => void;
}

export function EliminarClienteModal({
  open,
  onOpenChange,
  cliente,
  onEliminado,
}: EliminarClienteModalProps) {
  const [loading, setLoading] = useState(false);
  const [motivo, setMotivo] = useState('');

  async function handleEliminar() {
    setLoading(true);
    try {
      await clienteService.eliminarCliente(cliente.dni);
      onOpenChange(false);
      onEliminado?.();
      toast.success("Cliente eliminado con éxito");
    } catch {
      toast.error("No se pudo eliminar el cliente");
    } finally {
      setLoading(false);
    }
  }

  return (
    <Dialog open={open} onOpenChange={onOpenChange}>
      <DialogContent className="max-w-md">
        <DialogHeader>
          <DialogTitle>Confirmar eliminación</DialogTitle>
          <DialogDescription>
            Revisá los datos del cliente antes de continuar.
          </DialogDescription>
        </DialogHeader>

        <div className="space-y-3 py-1">
          <p className="font-medium text-base" style={{ color: "#D01F25" }}>
            {cliente.nombre_apellido}
          </p>

          <div className="flex items-center gap-2 text-sm">
            <IdCard className="w-4 h-4" style={{ color: "#4B5563" }} />
            <span style={{ color: "#4B5563" }}>{cliente.dni}</span>
          </div>

          <div className="flex items-center gap-2 text-sm">
            <Mail className="w-4 h-4" style={{ color: "#4B5563" }} />
            <span style={{ color: "#4B5563" }}>{cliente.email}</span>
          </div>

          <div className="space-y-1 pt-1">
            <label className="text-sm font-medium" style={{ color: '#1A1A1A' }}>
              Motivo de eliminación
            </label>
            <textarea
              value={motivo}
              onChange={(e) => setMotivo(e.target.value)}
              placeholder="Ingresá el motivo de la eliminación"
              rows={3}
              className="w-full border border-gray-300 rounded-lg px-3 py-2 text-sm resize-none focus:outline-none focus:border-brand"
              style={{ color: '#1A1A1A' }}
            />
          </div>
        </div>

        <DialogFooter className="gap-2 pt-2">
          <button
            onClick={() => onOpenChange(false)}
            disabled={loading}
            style={{
              padding: "9px 20px",
              borderRadius: "9999px",
              border: "1px solid #D1D5DB",
              background: "transparent",
              cursor: "pointer",
              fontSize: "14px",
              fontWeight: 500,
            }}
          >
            Cancelar
          </button>
          <button
            onClick={handleEliminar}
            disabled={loading || motivo.trim() === ''}
            style={{
              padding: "9px 24px",
              borderRadius: "9999px",
              border: "none",
              background: "#D01F25",
              color: "#fff",
              cursor: (loading || motivo.trim() === '') ? "not-allowed" : "pointer",
              fontSize: "14px",
              fontWeight: 500,
              opacity: (loading || motivo.trim() === '') ? 0.7 : 1,
            }}
          >
            {loading ? "Eliminando..." : "Confirmar eliminación"}
          </button>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  );
}