import { useEffect, useState } from 'react'
import { useNavigate } from 'react-router-dom'
import { ClaseCardRecepcionista } from '@/components/clases/ClaseCardRecepcionista'
import { clasesService, type ClaseDTO } from '@/services/clases.service'
import { actividadService, type Actividad } from '@/services/actividad.service'
import { profesorService, type Profesor } from '@/services/profesor.service'
import { toast } from 'sonner'
import {
  Dialog,
  DialogContent,
  DialogHeader,
  DialogTitle,
  DialogDescription,
  DialogFooter,
} from "@/components/ui/dialog"
import { Calendar, Clock, Users } from "lucide-react"

export default function ClasesPage() {
  const navigate = useNavigate()
  const [clases, setClases] = useState<ClaseDTO[]>([])
  const [actividades, setActividades] = useState<Actividad[]>([])
  const [profesores, setProfesores] = useState<Profesor[]>([])
  const [loading, setLoading] = useState(true)
  const [error, setError] = useState<string | null>(null)

  // Estados para el Modal de Eliminación interna
  const [modalOpen, setModalOpen] = useState(false)
  const [claseAEliminar, setClaseAEliminar] = useState<ClaseDTO | null>(null)
  const [eliminando, setEliminando] = useState(false)

  async function cargarDatos() {
    try {
      const [dataClases, dataActividades, dataProfesores] = await Promise.all([
        clasesService.getClases(),
        actividadService.getActividades(),
        profesorService.getProfesores(),
      ])
      setClases(dataClases)
      setActividades(dataActividades)
      setProfesores(dataProfesores)
    } catch {
      setError('No se pudieron cargar las clases.')
    } finally {
      setLoading(false)
    }
  }

  useEffect(() => {
    cargarDatos()
  }, [])

  // Al hacer clic en la tarjeta, preparamos la clase y abrimos el modal
  function prepararEliminacion(clase: ClaseDTO) {
    setClaseAEliminar(clase)
    setModalOpen(true)
  }

  // Ejecuta la eliminación real desde el modal
  async function handleConfirmarEliminar() {
    if (!claseAEliminar) return

    // Doble validación por seguridad
    if (claseAEliminar.inscripciones > 0) {
      toast.error('No se puede eliminar una clase con alumnos inscriptos.')
      return
    }

    setEliminando(true)
    try {
      await clasesService.eliminarClase(claseAEliminar.id_clase)
      toast.success('Clase eliminada con éxito')
      setModalOpen(false)
      setClaseAEliminar(null)
      cargarDatos() // Refresca las tarjetas
    } catch (err) {
      toast.error('No se pudo eliminar la clase.')
      console.error(err)
    } finally {
      setEliminando(false)
    }
  }

  function getNombreActividad(idActividad: string): string {
    return actividades.find((a) => String(a.id) === String(idActividad))?.nombre ?? idActividad
  }

  function getNombreProfesor(dniProfesor: number): string {
    return profesores.find((p) => p.dni === dniProfesor)?.nombre_completo ?? 'Sin asignar'
  }

  if (loading && clases.length === 0) return <p className="p-8 text-muted text-sm">Cargando clases...</p>
  if (error)   return <p className="p-8 text-destructive text-sm">{error}</p>

  return (
    <main className="p-4 md:p-8 bg-background min-h-screen">
      <div className="flex items-center justify-between mb-6">
        <div>
          <h1 className="text-2xl font-bold text-primary">Gestión de Clases</h1>
          <p className="text-sm mt-1">Administrá las clases y horarios</p>
        </div>
        <button
          onClick={() => navigate('/admin/clases/nueva')}
          className="bg-destructive text-white text-sm font-medium px-4 py-2 rounded-lg hover:opacity-90 transition-opacity"
        >
          + Nueva Clase
        </button>
      </div>

      {clases.length === 0 ? (
        <p className="text-sm">No hay clases cargadas.</p>
      ) : (
        <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
          {clases.map((clase) => (
            <ClaseCardRecepcionista
              key={clase.id_clase}
              idClase={clase.id_clase}
              dia={clase.dia}
              diaSemana={clase.dia_semana}
              horario={clase.horario}
              descripcion={clase.descripcion}
              cupoBase={clase.cupo_base}
              inscripciones={clase.inscripciones}
              idActividad={getNombreActividad(clase.id_actividad)}
              idSala={clase.id_sala}
              dniProfesor={clase.dni_profesor}
              nombreProfesor={getNombreProfesor(clase.dni_profesor)}
              onEditar={() => navigate(`/admin/clases/${clase.id_clase}/editar`)}
              onEliminar={() => prepararEliminacion(clase)}
            />
          ))}
        </div>
      )}

      {/* ================= MODAL INTERNO DE CONFIRMACIÓN ================= */}
      <Dialog open={modalOpen} onOpenChange={setModalOpen}>
        <DialogContent className="max-w-md">
          <DialogHeader>
            <DialogTitle>Confirmar eliminación de clase</DialogTitle>
            <DialogDescription>
              Revisá los datos de la clase antes de continuar. Esta acción no se puede deshacer.
            </DialogDescription>
          </DialogHeader>

          {claseAEliminar && (
            <div className="space-y-3 py-2">
              <p className="font-bold text-base" style={{ color: "#D01F25" }}>
                {getNombreActividad(claseAEliminar.id_actividad)}
              </p>
              
              {claseAEliminar.descripcion && (
                <p className="text-sm text-gray-500 italic">"{claseAEliminar.descripcion}"</p>
              )}

              <div className="flex items-center gap-2 text-sm text-gray-600">
                <Calendar className="w-4 h-4 text-gray-400" />
                <span>{claseAEliminar.dia_semana} — {claseAEliminar.dia ? claseAEliminar.dia.split('-').reverse().join('/') : '-'}</span>
              </div>

              <div className="flex items-center gap-2 text-sm text-gray-600">
                <Clock className="w-4 h-4 text-gray-400" />
                <span>Horario: {claseAEliminar.horario} hs</span>
              </div>

              <div className="flex items-center gap-2 text-sm text-gray-600">
                <Users className="w-4 h-4 text-gray-400" />
                <span>Inscriptos: <strong>{claseAEliminar.inscripciones} / {claseAEliminar.cupo_base}</strong></span>
              </div>

              {/* Alerta si tiene alumnos */}
              {claseAEliminar.inscripciones > 0 && (
                <div className="bg-red-50 border border-red-200 text-red-700 p-2.5 rounded-lg text-xs font-medium mt-2">
                  ⚠️ No se puede eliminar: Esta clase cuenta con alumnos registrados.
                </div>
              )}
            </div>
          )}

          <DialogFooter className="gap-2 pt-2">
            <button
              onClick={() => setModalOpen(false)}
              disabled={eliminando}
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
              onClick={handleConfirmarEliminar}
              // El botón se bloquea si está cargando O si tiene alumnos inscriptos
              disabled={eliminando || (claseAEliminar ? claseAEliminar.inscripciones > 0 : false)}
              style={{
                padding: "9px 24px",
                borderRadius: "9999px",
                border: "none",
                background: "#D01F25",
                color: "#fff",
                cursor: (eliminando || (claseAEliminar ? claseAEliminar.inscripciones > 0 : false)) ? "not-allowed" : "pointer",
                fontSize: "14px",
                fontWeight: 500,
                opacity: (eliminando || (claseAEliminar ? claseAEliminar.inscripciones > 0 : false)) ? 0.5 : 1,
              }}
            >
              {eliminando ? "Eliminando..." : "Confirmar eliminación"}
            </button>
          </DialogFooter>
        </DialogContent>
      </Dialog>
    </main>
  )
}