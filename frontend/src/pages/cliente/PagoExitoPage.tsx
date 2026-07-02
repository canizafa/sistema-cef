import { useEffect, useRef, useState } from 'react';
import { Link } from 'react-router-dom';
import { Header } from '@/components/layout/Header';
import { membresiaService, PRECIO_MEMBRESIA } from '@/services/membresia.service';
import { pagosService } from '@/services/pagos.service';
import { reservasService } from '@/services/clases.service';
import { useAuth } from '@/context/AuthContext';
import { useCreditos } from '@/context/CreditosContext';

export function PagoExitoPage() {
    const { user } = useAuth();
    const { refrescarCreditos } = useCreditos();
    const [esMembresía, setEsMembresia] = useState(false);
    const [avisosClases, setAvisosClases] = useState<string[]>([]);
    const [errorMembresia, setErrorMembresia] = useState(false);
    const [errorReserva, setErrorReserva] = useState(false);
    const yaProcesado = useRef(false);

    useEffect(() => {
        if (yaProcesado.current) return;

        const rawReserva = localStorage.getItem('pending_reserva');
        if (rawReserva) {
            yaProcesado.current = true;
            try {
                const { dni, idClase, fecha } = JSON.parse(rawReserva) as {
                    dni: number;
                    idClase: string;
                    fecha: string;
                };
                const dniEfectivo = dni ?? user?.dni;
                if (dniEfectivo) {
                    reservasService
                        .crearReserva({
                            fecha,
                            tipo: 'abono',
                            estado: 'confirmada',
                            dni_cliente: dniEfectivo,
                            id_clase: idClase,
                        })
                        .then(() => refrescarCreditos())
                        .catch(() => setErrorReserva(true));
                }
            } finally {
                localStorage.removeItem('pending_reserva');
            }
            return;
        }

        const raw = localStorage.getItem('pending_membresia');
        if (!raw) return;
        yaProcesado.current = true;
        try {
            const { tipo, dni, idActividad, idMembresia, horario, clases } = JSON.parse(raw) as {
                tipo: string;
                dni: number;
                idActividad: string;
                idMembresia?: string;
                horario?: string;
                clases?: unknown[];
            };
            const dniEfectivo = dni ?? user?.dni;
            if (!dniEfectivo) return;
            const horarioEfectivo = horario ?? '00:00';
            const promesa = idMembresia
                ? membresiaService.renovarMembresia(idMembresia, tipo, dniEfectivo, idActividad, horarioEfectivo)
                : membresiaService.crearMembresia(tipo, dniEfectivo, idActividad, horarioEfectivo);
            promesa
                .then(() => {
                    if (clases && clases.length > 0) {
                        setAvisosClases([
                            `Se reservaron ${clases.length} clases automáticamente para el próximo mes.`,
                        ]);
                    }
                })
                .catch(() => {
                    setErrorMembresia(true);
                })
                .finally(() => {
                    localStorage.removeItem('pending_membresia');
                });
            pagosService
                .confirmarPago({
                    monto: PRECIO_MEMBRESIA,
                    tipo: 'membresia',
                    fecha: new Date().toLocaleDateString('en-CA'),
                })
                .catch(() => {});
            setEsMembresia(true);
        } catch {
            localStorage.removeItem('pending_membresia');
        }
    }, [user]);

    return (
        <div className='min-h-screen flex flex-col'>
            <Header />
            <main className='flex-1 flex items-center justify-center px-4 py-12'>
                <div className='w-full max-w-sm text-center space-y-4'>
                    <h1 className='text-2xl font-bold'>Pago exitoso</h1>
                    <p className='text-sm text-gray-500'>
                        {esMembresía
                            ? 'Tu membresía fue activada correctamente.'
                            : 'Tu pago fue procesado correctamente. Tu clase fue reservada.'}
                    </p>
                    {errorMembresia && (
                        <p className='text-xs text-red-600'>
                            Hubo un problema al activar la membresía o reservar las clases. Contactá a soporte.
                        </p>
                    )}
                    {errorReserva && (
                        <p className='text-xs text-red-600'>
                            Hubo un problema al registrar tu reserva. Contactá a soporte.
                        </p>
                    )}
                    {avisosClases.length > 0 && (
                        <ul className='text-xs text-amber-600 text-left list-disc list-inside space-y-1'>
                            {avisosClases.map((aviso) => (
                                <li key={aviso}>{aviso}</li>
                            ))}
                        </ul>
                    )}
                    <Link
                        to={esMembresía ? '/membresia' : '/clases'}
                        className='text-brand hover:underline text-sm block'
                    >
                        {esMembresía ? 'Ver mi membresía' : 'Volver a clases'}
                    </Link>
                </div>
            </main>
        </div>
    );
}
