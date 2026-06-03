import { useEffect, useState } from 'react';
import { QRCodeSVG } from 'qrcode.react';
import { Header } from '@/components/layout/Header';
import { useAuth } from '@/context/AuthContext';
import { reservasService, clasesService, type ReservaResponse, type ClaseDTO } from '@/services/clases.service';

export function ReservasPage() {
    const { user } = useAuth();

    const [reservas, setReservas] = useState<ReservaResponse[]>([]);
    const [clases, setClases] = useState<ClaseDTO[]>([]);
    const [loading, setLoading] = useState(true);
    const [error, setError] = useState<string | null>(null);

    const [mostrarQr, setMostrarQr] = useState(false);

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

    function getClase(idClase: string): ClaseDTO | undefined {
        return clases.find((c) => c.id_clase === idClase);
    }

    const qrDisponible = !!user?.dni;

    return (
        <div className="min-h-screen flex flex-col">
            <Header />
            <main className="flex-1 px-4 py-10 max-w-md mx-auto w-full space-y-6">
                <div className="flex items-center justify-between">
                    <h1 className="text-2xl font-bold">Mis Reservas</h1>
                    <button
                        type="button"
                        onClick={() => setMostrarQr(true)}
                        className="bg-brand text-white rounded-md px-4 py-2 text-sm font-medium hover:opacity-90"
                    >
                        Visualizar QR
                    </button>
                </div>

                {loading && <p className="text-sm text-gray-500 text-center">Cargando...</p>}
                {error && <p className="text-sm text-red-600 text-center">{error}</p>}

                {!loading && !error && reservas.length === 0 && (
                    <p className="text-sm text-gray-500 text-center">No tenés reservas activas.</p>
                )}

                {reservas.map((reserva, idx) => {
                    const clase = getClase(reserva.id_clase);
                    return (
                        <div key={idx} className="border border-gray-200 rounded-lg p-4 space-y-1">
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
                            <span className={`inline-block text-xs font-medium px-2 py-0.5 rounded-full ${
                                reserva.estado === 'alta'
                                    ? 'bg-green-100 text-green-700'
                                    : 'bg-gray-100 text-gray-500'
                            }`}>
                                {reserva.estado === 'alta' ? 'Activa' : reserva.estado}
                            </span>
                        </div>
                    );
                })}
            </main>

            {/* Modal QR */}
            {mostrarQr && (
                <div
                    className="fixed inset-0 z-50 flex items-center justify-center bg-black/50"
                    onClick={() => setMostrarQr(false)}
                >
                    <div
                        className="bg-white rounded-xl p-8 flex flex-col items-center gap-4 shadow-xl"
                        onClick={(e) => e.stopPropagation()}
                    >
                        <h2 className="text-lg font-semibold">Tu QR</h2>

                        {qrDisponible ? (
                            <>
                                <QRCodeSVG value={String(user!.dni)} size={200} />
                                <p className="text-xs text-gray-500">DNI: {user!.dni}</p>
                            </>
                        ) : (
                            <p className="text-sm text-red-600">QR no disponible</p>
                        )}

                        <button
                            type="button"
                            onClick={() => setMostrarQr(false)}
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
