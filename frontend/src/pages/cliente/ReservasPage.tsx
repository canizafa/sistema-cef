import { useEffect, useState } from 'react';
import { toast } from 'sonner';
import { QRCodeSVG } from 'qrcode.react';
import { Header } from '@/components/layout/Header';
import { useAuth } from '@/context/AuthContext';
import { useCreditos } from '@/context/CreditosContext';
import { reservasService, clasesService, type ReservaResponse, type ClaseDTO } from '@/services/clases.service';
import { clienteService } from '@/services/cliente.service';

const DURACION_QR_SEGUNDOS = 15 * 60;

export function ReservasPage() {
    const { user } = useAuth();
    const { creditos, refrescarCreditos } = useCreditos();

    const [reservas, setReservas] = useState<ReservaResponse[]>([]);
    const [clases, setClases] = useState<ClaseDTO[]>([]);
    const [loading, setLoading] = useState(true);
    const [error, setError] = useState<string | null>(null);

    const [reservaParaQr, setReservaParaQr] = useState<ReservaResponse | null>(null);
    const [segundosRestantes, setSegundosRestantes] = useState(DURACION_QR_SEGUNDOS);
    const [cancelandoId, setCancelandoId] = useState<string | null>(null);

    useEffect(() => {
        if (!user) return;
        Promise.all([reservasService.getReservas(), clasesService.getClases()])
            .then(([todasReservas, todasClases]) => {
                setReservas(todasReservas.filter((r) => String(r.dni_cliente) === String(user.dni)));
                setClases(todasClases);
            })
            .catch(() => setError('No se pudieron cargar las reservas.'))
            .finally(() => setLoading(false));
    }, [user]);

    useEffect(() => {
        if (!reservaParaQr) return;
        if (segundosRestantes <= 0) return;
        const interval = setInterval(() => {
            setSegundosRestantes((s) => Math.max(0, s - 1));
        }, 1000);
        return () => clearInterval(interval);
    }, [reservaParaQr, segundosRestantes > 0]);

    function getClase(idClase: string): ClaseDTO | undefined {
        return clases.find((c) => c.id_clase === idClase);
    }

    function abrirQr(reserva: ReservaResponse) {
        setReservaParaQr(reserva);
        setSegundosRestantes(DURACION_QR_SEGUNDOS);
    }

    function cerrarQr() {
        setReservaParaQr(null);
    }

    async function handleCancelar(reserva: ReservaResponse) {
        if (!user) return;
        setCancelandoId(reserva.id_reserva);
        try {
            const creditosAntes = creditos;
            await reservasService.cancelarReserva(reserva.id_reserva);
            setReservas((prev) => prev.filter((r) => r.id_reserva !== reserva.id_reserva));

            // Opción A: re-fetcheamos créditos y comparamos para saber qué mensaje mostrar
            if (reserva.tipo === 'abono') {
                const perfilActualizado = await clienteService.getPerfil(user.dni);
                const creditosNuevos = perfilActualizado.cliente.creditos ?? 0;
                await refrescarCreditos();

                if (creditosNuevos === 0 && creditosAntes > 0) {
                    toast.error('Reserva cancelada, créditos disponibles eliminados por penalización de tercera cancelación.');
                } else if (creditosNuevos > creditosAntes) {
                    toast.success('Reserva cancelada, crédito disponible para futuras reservas.');
                } else {
                    toast.success('Reserva cancelada.');
                }
            } else {
                toast.success('Reserva cancelada.');
            }
        } catch {
            toast.error('No se pudo cancelar la reserva.');
        } finally {
            setCancelandoId(null);
        }
    }

    function formatTiempo(segundos: number): string {
        const min = Math.floor(segundos / 60);
        const seg = segundos % 60;
        return `${min}:${seg.toString().padStart(2, '0')}`;
    }

    const claseDelQr = reservaParaQr ? getClase(reservaParaQr.id_clase) : undefined;
    const qrExpirado = segundosRestantes <= 0;

    return (
        <div className="min-h-screen flex flex-col">
            <Header />
            <main className="flex-1 px-4 py-10 max-w-md mx-auto w-full space-y-6">
                <h1 className="text-2xl font-bold">Mis Reservas</h1>

                {loading && <p className="text-sm text-gray-500 text-center">Cargando...</p>}
                {error && <p className="text-sm text-red-600 text-center">{error}</p>}

                {!loading && !error && reservas.length === 0 && (
                    <p className="text-sm text-gray-500 text-center">No tenés reservas activas.</p>
                )}

                {reservas.map((reserva, idx) => {
                    const clase = getClase(reserva.id_clase);
                    return (
                        <div key={idx} className="border border-gray-200 rounded-lg p-4 space-y-2">
                            <p className="text-sm font-semibold text-gray-800">
                                {clase ? clase.descripcion || clase.id_actividad : reserva.id_clase}
                            </p>
                            {clase && (
                                <p className="text-xs text-gray-500">
                                    {clase.dia_semana} — {clase.horario}
                                </p>
                            )}
                            <p className="text-xs text-gray-500">
                                Fecha: {reserva.fecha.slice(0, 10).split('-').reverse().join('/')}
                            </p>
                            <div className="flex items-center justify-between">
                                <span className={`inline-block text-xs font-medium px-2 py-0.5 rounded-full ${
                                    reserva.estado === 'confirmada'
                                        ? 'bg-green-100 text-green-700'
                                        : 'bg-gray-100 text-gray-500'
                                }`}>
                                    {reserva.estado === 'confirmada' ? 'Activa' : reserva.estado}
                                </span>
                                <button
                                    type="button"
                                    onClick={() => abrirQr(reserva)}
                                    className="text-xs font-medium text-brand hover:underline"
                                >
                                    Ver QR
                                </button>
                            </div>
                            {reserva.estado === 'confirmada' && (
                                <button
                                    type="button"
                                    onClick={() => handleCancelar(reserva)}
                                    disabled={cancelandoId === reserva.id_reserva}
                                    className="w-full border border-red-300 text-red-600 rounded-md h-8 text-xs font-medium hover:bg-red-50 disabled:opacity-50"
                                >
                                    {cancelandoId === reserva.id_reserva ? 'Cancelando...' : 'Cancelar reserva'}
                                </button>
                            )}
                        </div>
                    );
                })}
            </main>

            {/* Modal QR por reserva */}
            {reservaParaQr && (
                <div
                    className="fixed inset-0 z-50 flex items-center justify-center bg-black/50"
                    onClick={cerrarQr}
                >
                    <div
                        className="bg-white rounded-xl p-8 flex flex-col items-center gap-4 shadow-xl"
                        onClick={(e) => e.stopPropagation()}
                    >
                        <h2 className="text-lg font-semibold">QR de la clase</h2>
                        <p className="text-xs text-gray-500 text-center">
                            {claseDelQr ? `${claseDelQr.descripcion || claseDelQr.id_actividad} — ${claseDelQr.dia_semana} ${claseDelQr.horario}` : reservaParaQr.id_clase}
                        </p>

                        {qrExpirado ? (
                            <>
                                <p className="text-sm text-red-600">El QR expiró.</p>
                                <button
                                    type="button"
                                    onClick={() => abrirQr(reservaParaQr)}
                                    className="bg-brand text-white rounded-md px-4 py-2 text-sm font-medium hover:opacity-90"
                                >
                                    Generar de nuevo
                                </button>
                            </>
                        ) : (
                            <>
                                <QRCodeSVG value={reservaParaQr.id_reserva} size={200} />
                                <p className="text-xs text-gray-500">
                                    Válido por {formatTiempo(segundosRestantes)}
                                </p>
                            </>
                        )}

                        <button
                            type="button"
                            onClick={cerrarQr}
                            className="mt-2 text-sm text-gray-500 hover:text-gray-700 underline"
                        >
                            Cerrar
                        </button>
                    </div>
                </div>
            )}
        </div>
    );
}
