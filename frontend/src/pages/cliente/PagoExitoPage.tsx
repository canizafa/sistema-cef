import { useEffect, useRef, useState } from 'react';
import { Link } from 'react-router-dom';
import { Header } from '@/components/layout/Header';
import { membresiaService, PRECIO_MEMBRESIA } from '@/services/membresia.service';
import { pagosService } from '@/services/pagos.service';
import { reservasService } from '@/services/clases.service';
import { useAuth } from '@/context/AuthContext';

export function PagoExitoPage() {
    const { user } = useAuth();
    const [esMembresía, setEsMembresia] = useState(false);
    const [clasesLlenas, setClasesLlenas] = useState(0);
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
                        .catch(() => setErrorReserva(true));
                    pagosService
                        .confirmarPago({
                            monto: 5000,
                            tipo: 'abono',
                            fecha: new Date().toLocaleDateString('en-CA'),
                        })
                        .catch(() => {});
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
            const { tipo, dni, idActividad, idMembresia, clases } = JSON.parse(raw) as {
                tipo: string;
                dni: number;
                idActividad: string;
                idMembresia?: string;
                clases?: { id_clase: string; fecha: string; lleno: boolean }[];
            };
            const dniEfectivo = dni ?? user?.dni;
            if (!dniEfectivo) return;
            const promesa = idMembresia
                ? membresiaService.renovarMembresia(idMembresia, tipo, dniEfectivo, idActividad)
                : membresiaService.crearMembresia(tipo, dniEfectivo, idActividad);
            promesa
                .then(() => {
                    if (!clases || clases.length === 0) return;
                    const disponibles = clases.filter((c) => !c.lleno);
                    const llenas = clases.filter((c) => c.lleno);
                    setClasesLlenas(llenas.length);
                    return Promise.allSettled(
                        disponibles.map((c) =>
                            reservasService.crearReserva({
                                fecha: c.fecha,
                                tipo: 'membresia',
                                estado: 'confirmada',
                                dni_cliente: dniEfectivo,
                                id_clase: c.id_clase,
                            })
                        )
                    );
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
                    {clasesLlenas > 0 && (
                        <p className='text-xs text-amber-600'>
                            {clasesLlenas} clase{clasesLlenas > 1 ? 's' : ''} ya estaba
                            {clasesLlenas > 1 ? 'n' : ''} llena{clasesLlenas > 1 ? 's' : ''} y no se reservó (lista de espera próximamente).
                        </p>
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
