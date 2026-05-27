import { useEffect, useState } from 'react'
import { useNavigate } from 'react-router-dom'
import { ClaseCardRecepcionista } from '@/components/clases/ClaseCardRecepcionista'
import { type ClaseDTO } from '@/services/clases.service'

// ===================== MOCK =====================
const MOCK_CLASES = true; // cambiar a false cuando el back esté listo

const clasesMock: ClaseDTO[] = [
    { id_clase: '1', dia: '2026-06-02', horario: '08:00', estado: 'alta', lleno: false, descripcion: 'Yoga para principiantes', id_actividad: 'act-1' },
    { id_clase: '2', dia: '2026-06-02', horario: '10:00', estado: 'alta', lleno: false, descripcion: 'Pilates intermedio', id_actividad: 'act-2' },
    { id_clase: '3', dia: '2026-06-03', horario: '09:00', estado: 'alta', lleno: true,  descripcion: 'Spinning avanzado', id_actividad: 'act-3' },
]
// ================================================

export default function ClasesAdminPage() {
  const navigate = useNavigate()
  const [clases, setClases] = useState<ClaseDTO[]>(MOCK_CLASES ? clasesMock : [])
  const [loading, setLoading] = useState(false)
  const [error, setError] = useState<string | null>(null)

  if (loading) return <p className="p-8 text-muted text-sm">Cargando clases...</p>
  if (error)   return <p className="p-8 text-destructive text-sm">{error}</p>

  return (
    <main className="p-4 md:p-8 bg-background min-h-screen">
      <div className="flex items-center justify-between mb-6">
        <div>
          <h1 className="text-2xl font-bold text-primary">Gestión de Clases</h1>
          <p className="text-sm text-muted mt-1">Administrá las clases y horarios</p>
        </div>
        <button
          onClick={() => navigate('/admin/clases/nueva')}
          className="bg-destructive text-white text-sm font-medium px-4 py-2 rounded-lg hover:opacity-90 transition-opacity"
        >
          + Nueva Clase
        </button>
      </div>

      {clases.length === 0 ? (
        <p className="text-sm text-muted">No hay clases cargadas.</p>
      ) : (
        <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
          {clases.map((clase) => (
            <ClaseCardRecepcionista
              key={clase.id_clase}
              idClase={clase.id_clase}
              dia={clase.dia}
              horario={clase.horario}
              estado={clase.estado}
              onEditar={() => console.log('Editar clase id:', clase.id_clase)}
              onVerReservas={() => console.log('Ver reservas de clase id:', clase.id_clase)}
            />
          ))}
        </div>
      )}
    </main>
  )
}