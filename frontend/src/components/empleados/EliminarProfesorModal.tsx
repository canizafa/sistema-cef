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
import { IdCard } from "lucide-react";
import { profesorService } from "@/services/profesor.service";

interface EliminarProfesorModalProps {
  open: boolean;
  onOpenChange: (open: boolean) => void;
  profesor: {
    dni: number;
    nombre_completo: string;
    estado: string;
  };
  onEliminado?: () => void;
}

export function EliminarProfesorModal({
  open,
  onOpenChange,
  profesor,
  onEliminado,
}: EliminarProfesorModalProps) {
  const [loading, setLoading] = useState(false);

  async function handleEliminar() {
    setLoading(true);
    try {
      await profesorService.eliminarProfesor(profesor.dni);
      onOpenChange(false);
      onEliminado?.();
      toast.success("Profesor eliminado con éxito");
    } catch (error: any) {
      if (error?.response?.status === 409) {
        toast.error("No se puede eliminar a un profesor con clases asociadas");
      } else {
        toast.error("No se pudo eliminar el profesor");
      }
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
            Revisá los datos del profesor antes de continuar.
          </DialogDescription>
        </DialogHeader>

        <div className="space-y-3 py-1">
          <p className="font-medium text-base" style={{ color: "#D01F25" }}>
            {profesor.nombre_completo}
          </p>
          <div className="flex items-center gap-2 text-sm">
            <IdCard className="w-4 h-4" style={{ color: "#4B5563" }} />
            <span style={{ color: "#4B5563" }}>{profesor.dni}</span>
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