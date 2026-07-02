import { useEffect, useState } from 'react';
import { Header } from '@/components/layout/Header';
import { useAuth } from '@/context/AuthContext';
import { actividadService, membresiaService, type ActividadResponse, type MembresiaResponse, PRECIO_MEMBRESIA } from '@/services/membresia.service';
import { pagosService } from '@/services/pagos.service';
import { clasesService, reservasService, type ClaseDTO } from '@/services/clases.service';

export function MembresiaPage() {
    const { user } = useAuth();

    const [membresias, setMembresias] = useState<MembresiaResponse[]>([]);
    const [actividades, setActividades] = useState<ActividadResponse[]>([]);
    const [clases, setClases] = useState<ClaseDTO[]>([]);
    const [loading, setLoading] = useState(true);
    const [error, setError] = useState<string | null>(null);
    const [procesandoId, setProcesandoId] = useState<string | null>(null);
    const [seleccion, setSeleccion] = useState<Record<string, string>>({});
    const [clasesYaReservadas, setClasesYaReservadas] = useState<Set<string>>(new Set());

    useEffect(() => {
        if (!user) return;
        Promise.all([
            membresiaService.getMembresiasPorDni(user.dni),
            actividadService.getActividades(),
            clasesService.getClases(),
            reservasService.getReservas(),
        ])
            .then(([mems, acts, cls, reservas]) => {
                setMembresias(mems);
                setActividades(acts);
                setClases(cls);
                const reservasConfirmadas = reservas.filter(
                    (r) => String(r.dni_cliente) === String(user.dni) && r.estado === 'confirmada'
                );
                setClasesYaReservadas(new Set(reservasConfirmadas.map((r) => r.id_clase)));
            })
            .catch(() => setError('No se pudo cargar la información.'))
            .finally(() => setLoading(false));
    }, [user]);

    type GrupoHorario = {
        key: string;
        label: string;
        clases: ClaseDTO[];
    };

    function getGruposHorario(idActividad: string): GrupoHorario[] {
        const clasesActividad = clases
            .filter((c) => c.id_actividad === idActividad)
            .sort((a, b) => a.dia.localeCompare(b.dia));

        const grupos = new Map<string, ClaseDTO[]>();
        for (const c of clasesActividad) {
            const key = `${c.dia_semana}-${c.horario}`;
            grupos.set(key, [...(grupos.get(key) ?? []), c]);
        }

        return Array.from(grupos.entries())
            .filter(([, clasesGrupo]) => !clasesGrupo.some((c) => c.inscripciones >= c.cupo_base))
            .map(([key, clasesGrupo]) => ({
                key,
                label: `${clasesGrupo[0].dia_semana} ${clasesGrupo[0].horario}`,
                clases: clasesGrupo,
            }));
    }

    function iniciarPago(actividad: ActividadResponse, grupo: GrupoHorario, idMembresia?: string) {
        if (!user) return;
        setProcesandoId(idMembresia ?? actividad.id);
        pagosService
            .crearPago({
                titulo: `Membresía - ${actividad.nombre}`,
                monto: PRECIO_MEMBRESIA,
                fecha: new Date().toLocaleDateString('en-CA'),
                hora: '00:00',
                sena: false,
                id_membresia: '',
                dni: user.dni,
                reserva_paga: '',
            })
            .then((pago) => {
                localStorage.setItem('pending_membresia', JSON.stringify({
                    tipo: actividad.nombre,
                    dni: user.dni,
                    idActividad: actividad.id,
                    idMembresia,
                    horario: grupo.clases[0]?.horario ?? '00:00',
                    clases: grupo.clases.map((c) => ({
                        id_clase: c.id_clase,
                        fecha: c.dia,
                        lleno: c.inscripciones >= c.cupo_base,
                        diaSemana: c.dia_semana,
                        nombreActividad: actividad.nombre,
                        yaReservada: clasesYaReservadas.has(c.id_clase),
                    })),
                }));
                window.location.href = pago.sandbox_init_point;
            })
            .catch(() => {
                setError('Error al iniciar el pago. Intentá de nuevo.');
                setProcesandoId(null);
            });
    }

    function puedeRenovar(membresia: MembresiaResponse): boolean {
        if (!membresia.fecha_fin) return true;
        const hoy = new Date().toLocaleDateString('en-CA');
        return hoy >= membresia.fecha_fin.slice(0, 10);
    }

    async function handleDarBaja(membresia: MembresiaResponse, idActividad: string) {
        if (!user) return;
        setProcesandoId(membresia.id_membresia);
        try {
            await membresiaService.darBajaMembresia(membresia, user.dni);

            const idsClaseActividad = new Set(
                clases.filter((c) => c.id_actividad === idActividad).map((c) => c.id_clase)
            );
            const reservas = await reservasService.getReservas();
            const reservasDeEstaMembresia = reservas.filter(
                (r) =>
                    String(r.dni_cliente) === String(user.dni) &&
                    r.tipo === 'membresia' &&
                    idsClaseActividad.has(r.id_clase)
            );
            await Promise.allSettled(
                reservasDeEstaMembresia.map((r) => reservasService.cancelarReserva(r.id_reserva))
            );

            setMembresias((prev) =>
                prev.map((m) => (m.id_membresia === membresia.id_membresia ? { ...m, estado: 'cancelado' } : m))
            );
        } catch {
            setError('No se pudo dar de baja la membresía.');
        } finally {
            setProcesandoId(null);
        }
    }

    return (
        <div className="min-h-screen flex flex-col">
            <Header />
            <main className="flex-1 px-4 py-10 max-w-md mx-auto w-full space-y-6">
                <h1 className="text-2xl font-bold text-center">Membresía</h1>

                {loading && <p className="text-sm text-gray-500 text-center">Cargando...</p>}
                {error && <p className="text-sm text-red-600 text-center">{error}</p>}

                {!loading && !error && (
                    <div className="space-y-3">
                        {actividades.length === 0 && (
                            <p className="text-sm text-gray-400 text-center">No hay actividades disponibles.</p>
                        )}
                        {actividades.map((act) => {
                            const grupos = getGruposHorario(act.id);
                            const membresia = membresias.find((m) => m.tipo === act.nombre);
                            const procesando = procesandoId === act.id || procesandoId === membresia?.id_membresia;
                            const grupoSeleccionado = grupos.find((g) => g.key === seleccion[act.id]);
                            const sinSeleccion = !grupoSeleccionado;
                            const renovacionBloqueada = !!membresia && membresia.estado === 'activo' && !puedeRenovar(membresia);

                            return (
                                <div key={act.id} className="border border-gray-200 rounded-lg p-4 space-y-2">
                                    <p className="text-sm font-semibold text-gray-800">{act.nombre}</p>
                                    {act.descripcion && (
                                        <p className="text-xs text-gray-500">{act.descripcion}</p>
                                    )}
                                    {grupos.length > 0 && (
                                        <div className="flex flex-wrap gap-1.5">
                                            {grupos.map((g) => (
                                                <button
                                                    key={g.key}
                                                    type="button"
                                                    onClick={() =>
                                                        setSeleccion((prev) => ({ ...prev, [act.id]: g.key }))
                                                    }
                                                    className={`text-xs rounded-full px-2 py-0.5 border ${
                                                        seleccion[act.id] === g.key
                                                            ? 'bg-brand text-white border-brand'
                                                            : 'text-gray-600 bg-gray-100 border-gray-100'
                                                    }`}
                                                >
                                                    {g.label} ({g.clases.length} clases)
                                                </button>
                                            ))}
                                        </div>
                                    )}

                                    {membresia?.estado === 'activo' ? (
                                        <>
                                            <p className="text-xs text-green-700 font-medium">
                                                Activa hasta {membresia.fecha_fin?.slice(0, 10).split('-').reverse().join('/')}
                                            </p>
                                            <div className="flex gap-2">
                                                <button
                                                    type="button"
                                                    onClick={() => grupoSeleccionado && iniciarPago(act, grupoSeleccionado, membresia.id_membresia)}
                                                    disabled={procesando || sinSeleccion || renovacionBloqueada}
                                                    className="flex-1 bg-brand text-white rounded-md h-9 text-sm font-medium hover:opacity-90 disabled:opacity-50"
                                                >
                                                    {procesando ? '...' : 'Renovar'}
                                                </button>
                                                <button
                                                    type="button"
                                                    onClick={() => handleDarBaja(membresia, act.id)}
                                                    disabled={procesando}
                                                    className="flex-1 border border-red-300 text-red-600 rounded-md h-9 text-sm font-medium hover:bg-red-50 disabled:opacity-50"
                                                >
                                                    Dar de baja
                                                </button>
                                            </div>
                                        </>
                                    ) : membresia ? (
                                        <button
                                            type="button"
                                            onClick={() => grupoSeleccionado && iniciarPago(act, grupoSeleccionado, membresia.id_membresia)}
                                            disabled={procesando || sinSeleccion}
                                            className="w-full bg-brand text-white rounded-md h-9 text-sm font-medium hover:opacity-90 disabled:opacity-50"
                                        >
                                            {procesando ? 'Redirigiendo...' : 'Pagar Membresía'}
                                        </button>
                                    ) : (
                                        <>
                                            <p className="text-xs text-gray-500 font-medium">
                                                ${PRECIO_MEMBRESIA.toLocaleString('es-AR')} / mes
                                            </p>
                                            <button
                                                type="button"
                                                onClick={() => grupoSeleccionado && iniciarPago(act, grupoSeleccionado)}
                                                disabled={procesando || sinSeleccion}
                                                className="w-full bg-brand text-white rounded-md h-9 text-sm font-medium hover:opacity-90 disabled:opacity-50"
                                            >
                                                {procesando ? 'Redirigiendo...' : 'Pagar Membresía'}
                                            </button>
                                        </>
                                    )}
                                </div>
                            );
                        })}
                    </div>
                )}
            </main>
        </div>
    );
}
