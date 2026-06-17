import { useEffect, useState } from 'react';
import { Header } from '@/components/layout/Header';
import { useAuth } from '@/context/AuthContext';
import { actividadService, membresiaService, type ActividadResponse, type MembresiaResponse, PRECIO_MEMBRESIA } from '@/services/membresia.service';
import { pagosService } from '@/services/pagos.service';

export function MembresiaPage() {
    const { user } = useAuth();

    const [membresia, setMembresia] = useState<MembresiaResponse | null>(null);
    const [actividades, setActividades] = useState<ActividadResponse[]>([]);
    const [loading, setLoading] = useState(true);
    const [error, setError] = useState<string | null>(null);
    const [pagandoId, setPagandoId] = useState<string | null>(null);

    useEffect(() => {
        if (!user) return;
        Promise.all([
            membresiaService.getMembresiaPorDni(user.dni),
            actividadService.getActividades(),
        ])
            .then(([mem, acts]) => {
                setMembresia(mem?.estado === 'alta' ? mem : null);
                setActividades(acts);
            })
            .catch(() => setError('No se pudo cargar la información.'))
            .finally(() => setLoading(false));
    }, [user]);

    async function handlePagar(actividad: ActividadResponse) {
        if (!user) return;
        setPagandoId(actividad.id);
        try {
            const hoy = new Date().toLocaleDateString('en-CA');
            const pago = await pagosService.crearPago({
                titulo: `Membresía - ${actividad.nombre}`,
                monto: PRECIO_MEMBRESIA,
                fecha: hoy,
                hora: '00:00',
                sena: false,
                id_membresia: '',
                reserva_paga: '',
            });
            localStorage.setItem('pending_membresia', JSON.stringify({
                tipo: actividad.nombre,
                dni: user.dni,
            }));
            window.location.href = pago.sandbox_init_point;
        } catch {
            setError('Error al iniciar el pago. Intentá de nuevo.');
            setPagandoId(null);
        }
    }

    return (
        <div className="min-h-screen flex flex-col">
            <Header />
            <main className="flex-1 px-4 py-10 max-w-md mx-auto w-full space-y-6">
                <h1 className="text-2xl font-bold text-center">Membresía</h1>

                {loading && <p className="text-sm text-gray-500 text-center">Cargando...</p>}
                {error && <p className="text-sm text-red-600 text-center">{error}</p>}

                {!loading && !error && membresia && (
                    <div className="border border-green-200 bg-green-50 rounded-lg p-5 space-y-2 text-center">
                        <p className="text-sm font-semibold text-green-700">Tenés una membresía activa</p>
                        <p className="text-xs text-gray-600">Tipo: {membresia.tipo}</p>
                        <p className="text-xs text-gray-600">
                            Desde: {membresia.fecha_inicio.slice(0, 10).split('-').reverse().join('/')}
                        </p>
                    </div>
                )}

                {!loading && !error && !membresia && (
                    <>
                        <p className="text-sm text-gray-500 text-center">
                            No tenés una membresía activa. Elegí una actividad para comenzar.
                        </p>
                        <div className="space-y-3">
                            {actividades.length === 0 && (
                                <p className="text-sm text-gray-400 text-center">No hay actividades disponibles.</p>
                            )}
                            {actividades.map((act) => (
                                <div key={act.id} className="border border-gray-200 rounded-lg p-4 space-y-2">
                                    <p className="text-sm font-semibold text-gray-800">{act.nombre}</p>
                                    {act.descripcion && (
                                        <p className="text-xs text-gray-500">{act.descripcion}</p>
                                    )}
                                    <p className="text-xs text-gray-500 font-medium">
                                        ${PRECIO_MEMBRESIA.toLocaleString('es-AR')} / mes
                                    </p>
                                    <button
                                        type="button"
                                        onClick={() => handlePagar(act)}
                                        disabled={pagandoId === act.id}
                                        className="w-full bg-brand text-white rounded-md h-9 text-sm font-medium hover:opacity-90 disabled:opacity-50"
                                    >
                                        {pagandoId === act.id ? 'Redirigiendo...' : 'Pagar Membresía'}
                                    </button>
                                </div>
                            ))}
                        </div>
                    </>
                )}
            </main>
        </div>
    );
}
