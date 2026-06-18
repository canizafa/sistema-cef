import { useEffect, useState } from 'react';
import { Link } from 'react-router-dom';
import { Header } from '@/components/layout/Header';
import { membresiaService, PRECIO_MEMBRESIA } from '@/services/membresia.service';
import { pagosService } from '@/services/pagos.service';
import { useAuth } from '@/context/AuthContext';

export function PagoExitoPage() {
    const { user } = useAuth();
    const [esMembresía, setEsMembresia] = useState(false);

    useEffect(() => {
        const raw = localStorage.getItem('pending_membresia');
        if (!raw) return;
        try {
            const { tipo, dni } = JSON.parse(raw) as { tipo: string; dni: number };
            const dniEfectivo = dni ?? user?.dni;
            if (!dniEfectivo) return;
            membresiaService.crearMembresia(tipo, dniEfectivo).finally(() => {
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
