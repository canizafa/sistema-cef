import { useState } from 'react'
import { isAxiosError } from 'axios'
import { estadisticasService, type ClaseMasConcurridaResponse, type ClaseMasCanceladaResponse } from '@/services/estadisticas.service'

function primerDiaDelMes(mes: string): string {
  return `${mes}-01`
}

function ultimoDiaDelMes(mes: string): string {
  const [anio, mesNumero] = mes.split('-').map(Number)
  const ultimoDia = new Date(anio, mesNumero, 0).getDate()
  return `${mes}-${String(ultimoDia).padStart(2, '0')}`
}

function mensajeDeError(err: unknown): string {
  if (isAxiosError(err) && err.response?.status === 422) {
    return 'La fecha de inicio no puede ser posterior a la fecha de fin.'
  }
  return 'No se pudo obtener la estadística. Intentá de nuevo.'
}

export function ReportesPage() {
  const [mesInicio, setMesInicio] = useState('')
  const [mesFin, setMesFin] = useState('')
  const [total, setTotal] = useState<number | null>(null)
  const [sinDatos, setSinDatos] = useState(false)
  const [loading, setLoading] = useState(false)
  const [error, setError] = useState<string | null>(null)

  async function handleVerEstadisticas() {
    if (!mesInicio || !mesFin) return
    setLoading(true)
    setError(null)
    setTotal(null)
    setSinDatos(false)
    try {
      const { total } = await estadisticasService.getRecaudacion(
        primerDiaDelMes(mesInicio),
        ultimoDiaDelMes(mesFin)
      )
      if (total === 0) {
        setSinDatos(true)
      } else {
        setTotal(total)
      }
    } catch (err) {
      setError(mensajeDeError(err))
    } finally {
      setLoading(false)
    }
  }

  const [mesInicioConcurrida, setMesInicioConcurrida] = useState('')
  const [mesFinConcurrida, setMesFinConcurrida] = useState('')
  const [claseMasConcurrida, setClaseMasConcurrida] = useState<ClaseMasConcurridaResponse | null>(null)
  const [sinDatosConcurrida, setSinDatosConcurrida] = useState(false)
  const [loadingConcurrida, setLoadingConcurrida] = useState(false)
  const [errorConcurrida, setErrorConcurrida] = useState<string | null>(null)

  async function handleVerEstadisticasConcurrida() {
    if (!mesInicioConcurrida || !mesFinConcurrida) return
    setLoadingConcurrida(true)
    setErrorConcurrida(null)
    setClaseMasConcurrida(null)
    setSinDatosConcurrida(false)
    try {
      const data = await estadisticasService.getClaseMasConcurrida(
        primerDiaDelMes(mesInicioConcurrida),
        ultimoDiaDelMes(mesFinConcurrida)
      )
      if (data.cantidad === 0) {
        setSinDatosConcurrida(true)
      } else {
        setClaseMasConcurrida(data)
      }
    } catch (err) {
      setErrorConcurrida(mensajeDeError(err))
    } finally {
      setLoadingConcurrida(false)
    }
  }

  const [mesInicioCancelada, setMesInicioCancelada] = useState('')
  const [mesFinCancelada, setMesFinCancelada] = useState('')
  const [claseMasCancelada, setClaseMasCancelada] = useState<ClaseMasCanceladaResponse | null>(null)
  const [sinDatosCancelada, setSinDatosCancelada] = useState(false)
  const [loadingCancelada, setLoadingCancelada] = useState(false)
  const [errorCancelada, setErrorCancelada] = useState<string | null>(null)

  async function handleVerEstadisticasCancelada() {
    if (!mesInicioCancelada || !mesFinCancelada) return
    setLoadingCancelada(true)
    setErrorCancelada(null)
    setClaseMasCancelada(null)
    setSinDatosCancelada(false)
    try {
      const data = await estadisticasService.getClaseMasCancelada(
        primerDiaDelMes(mesInicioCancelada),
        ultimoDiaDelMes(mesFinCancelada)
      )
      if (data.cantidad === 0) {
        setSinDatosCancelada(true)
      } else {
        setClaseMasCancelada(data)
      }
    } catch (err) {
      setErrorCancelada(mensajeDeError(err))
    } finally {
      setLoadingCancelada(false)
    }
  }

  return (
    <main className="p-4 md:p-8 bg-background min-h-screen">
      <h1 className="text-2xl font-bold text-primary mb-6">Reportes</h1>

      <div className="border border-border rounded-lg p-4 max-w-md space-y-4">
        <h2 className="text-lg font-semibold text-primary">Estadística de recaudación</h2>

        <div className="flex gap-3">
          <div className="flex-1">
            <label className="block text-xs text-gray-500 mb-1">Inicio</label>
            <input
              type="month"
              value={mesInicio}
              onChange={(e) => setMesInicio(e.target.value)}
              className="w-full border border-border rounded-md h-9 px-2 text-sm"
            />
          </div>
          <div className="flex-1">
            <label className="block text-xs text-gray-500 mb-1">Hasta</label>
            <input
              type="month"
              value={mesFin}
              onChange={(e) => setMesFin(e.target.value)}
              className="w-full border border-border rounded-md h-9 px-2 text-sm"
            />
          </div>
        </div>

        <button
          type="button"
          onClick={handleVerEstadisticas}
          disabled={!mesInicio || !mesFin || loading}
          className="w-full bg-brand text-white rounded-md h-9 text-sm font-medium hover:opacity-90 disabled:opacity-50"
        >
          {loading ? 'Cargando...' : 'Ver estadísticas'}
        </button>

        {error && <p className="text-sm text-destructive">{error}</p>}
        {sinDatos && (
          <p className="text-sm text-gray-500">
            No hay información para mostrar en ese período.
          </p>
        )}
        {total !== null && (
          <p className="text-sm font-semibold text-primary">
            Total recaudado: ${total.toLocaleString('es-AR')}
          </p>
        )}
      </div>

      <div className="border border-border rounded-lg p-4 max-w-md space-y-4 mt-6">
        <h2 className="text-lg font-semibold text-primary">Generar estadística de Clase más concurrida</h2>

        <div className="flex gap-3">
          <div className="flex-1">
            <label className="block text-xs text-gray-500 mb-1">Inicio</label>
            <input
              type="month"
              value={mesInicioConcurrida}
              onChange={(e) => setMesInicioConcurrida(e.target.value)}
              className="w-full border border-border rounded-md h-9 px-2 text-sm"
            />
          </div>
          <div className="flex-1">
            <label className="block text-xs text-gray-500 mb-1">Hasta</label>
            <input
              type="month"
              value={mesFinConcurrida}
              onChange={(e) => setMesFinConcurrida(e.target.value)}
              className="w-full border border-border rounded-md h-9 px-2 text-sm"
            />
          </div>
        </div>

        <button
          type="button"
          onClick={handleVerEstadisticasConcurrida}
          disabled={!mesInicioConcurrida || !mesFinConcurrida || loadingConcurrida}
          className="w-full bg-brand text-white rounded-md h-9 text-sm font-medium hover:opacity-90 disabled:opacity-50"
        >
          {loadingConcurrida ? 'Cargando...' : 'Ver estadísticas'}
        </button>

        {errorConcurrida && <p className="text-sm text-destructive">{errorConcurrida}</p>}
        {sinDatosConcurrida && (
          <p className="text-sm text-gray-500">
            No hay información para mostrar en ese periodo.
          </p>
        )}
        {claseMasConcurrida && (
          <p className="text-sm font-semibold text-primary">
            Clase más concurrida: {claseMasConcurrida.descripcion} ({claseMasConcurrida.cantidad} reservas)
          </p>
        )}
      </div>

      <div className="border border-border rounded-lg p-4 max-w-md space-y-4 mt-6">
        <h2 className="text-lg font-semibold text-primary">Generar estadística de Clase más cancelada</h2>

        <div className="flex gap-3">
          <div className="flex-1">
            <label className="block text-xs text-gray-500 mb-1">Inicio</label>
            <input
              type="month"
              value={mesInicioCancelada}
              onChange={(e) => setMesInicioCancelada(e.target.value)}
              className="w-full border border-border rounded-md h-9 px-2 text-sm"
            />
          </div>
          <div className="flex-1">
            <label className="block text-xs text-gray-500 mb-1">Hasta</label>
            <input
              type="month"
              value={mesFinCancelada}
              onChange={(e) => setMesFinCancelada(e.target.value)}
              className="w-full border border-border rounded-md h-9 px-2 text-sm"
            />
          </div>
        </div>

        <button
          type="button"
          onClick={handleVerEstadisticasCancelada}
          disabled={!mesInicioCancelada || !mesFinCancelada || loadingCancelada}
          className="w-full bg-brand text-white rounded-md h-9 text-sm font-medium hover:opacity-90 disabled:opacity-50"
        >
          {loadingCancelada ? 'Cargando...' : 'Ver estadísticas'}
        </button>

        {errorCancelada && <p className="text-sm text-destructive">{errorCancelada}</p>}
        {sinDatosCancelada && (
          <p className="text-sm text-gray-500">
            No hay información para mostrar en ese periodo.
          </p>
        )}
        {claseMasCancelada && (
          <p className="text-sm font-semibold text-primary">
            Clase más cancelada: {claseMasCancelada.descripcion} ({claseMasCancelada.cantidad} cancelaciones)
          </p>
        )}
      </div>
    </main>
  )
}
