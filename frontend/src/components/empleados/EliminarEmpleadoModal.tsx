import { useState } from "react";
import { useNavigate } from "react-router-dom";
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
import { empleadoService } from "@/services/empleados.service";
import type { UpdateEmpleado } from "@/services/empleados.service";

interface EliminarEmpleadoModalProps {
  open: boolean;
  onOpenChange: (open: boolean) => void;
  empleado: UpdateEmpleado & { nombre_apellido: string };
  onEliminado?: () => void;
}

export function EliminarEmpleadoModal({
  open,
  onOpenChange,
  empleado,
  onEliminado,
}: EliminarEmpleadoModalProps) {
  const navigate = useNavigate();
  const [loading, setLoading] = useState(false);

  async function handleEliminar() {
    setLoading(true);
    try {
      await empleadoService.eliminarEmpleado(empleado.dni);
      onOpenChange(false);
      onEliminado?.();
      toast.success("Empleado dado de baja");
      navigate("/admin/empleados");
    } catch {
      toast.error("No se pudo eliminar el empleado");
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
            Revisá los datos del empleado antes de continuar.
          </DialogDescription>
        </DialogHeader>

        <div className="space-y-3 py-1">
          <p className="font-medium text-base" style={{ color: "#D01F25" }}>
            {empleado.nombre_apellido}
          </p>

          <div className="flex items-center gap-2 text-sm">
  <IdCard className="w-4 h-4" style={{ color: "#4B5563" }} />
  <span style={{ color: "#4B5563" }}>{empleado.dni}</span>
</div>

<div className="flex items-center gap-2 text-sm">
  <Mail className="w-4 h-4" style={{ color: "#4B5563" }} />
  <span style={{ color: "#4B5563" }}>{empleado.mail}</span>
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
            disabled={loading}
            style={{
              padding: "9px 24px",
              borderRadius: "9999px",
              border: "none",
              background: "#D01F25",
              color: "#fff",
              cursor: loading ? "not-allowed" : "pointer",
              fontSize: "14px",
              fontWeight: 500,
              opacity: loading ? 0.7 : 1,
            }}
          >
            {loading ? "Eliminando..." : "Confirmar eliminación"}
          </button>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  );
}