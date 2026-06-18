import { useEffect, useState } from 'react';
import { Link } from 'react-router-dom';
import { Header } from '@/components/layout/Header';

export function PagoFalloPage() {
    const [reintentoPath, setReintentoPath] = useState('/clases');

    useEffect(() => {
        const eraMembresia = !!localStorage.getItem('pending_membresia');
        if (eraMembresia) {
            setReintentoPath('/membresia');
            localStorage.removeItem('pending_membresia');
        }
    }, []);

    return (
        <div className='min-h-screen flex flex-col'>
            <Header />
            <main className='flex-1 flex items-center justify-center px-4 py-12'>
                <div className='w-full max-w-sm text-center space-y-4'>
                    <h1 className='text-2xl font-bold'>No se pudo realizar el pago</h1>
                    <p className='text-sm text-gray-500'>
                        No se pudo realizar el pago por saldo insuficiente.
                    </p>
                    <Link
                        to={reintentoPath}
                        className='w-full bg-brand text-white rounded-md h-10 text-sm font-medium hover:opacity-90 flex items-center justify-center'
                    >
                        Intentar de nuevo
                    </Link>
                    <Link to='/clases' className='text-brand hover:underline text-sm block'>
                        Volver a clases
                    </Link>
                </div>
            </main>
        </div>
    );
}
