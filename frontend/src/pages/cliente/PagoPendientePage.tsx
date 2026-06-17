import { useEffect } from 'react';
import { Link } from 'react-router-dom';
import { Header } from '@/components/layout/Header';

export function PagoPendientePage() {
    useEffect(() => {
        localStorage.removeItem('pending_membresia');
    }, []);

    return (
        <div className='min-h-screen flex flex-col'>
            <Header />
            <main className='flex-1 flex items-center justify-center px-4 py-12'>
                <div className='w-full max-w-sm text-center space-y-4'>
                    <h1 className='text-2xl font-bold'>Pago en proceso</h1>
                    <p className='text-sm text-gray-500'>
                        Tu pago está siendo procesado. Te notificaremos cuando se confirme.
                    </p>
                    <Link to='/clases' className='text-brand hover:underline text-sm block'>
                        Volver a clases
                    </Link>
                </div>
            </main>
        </div>
    );
}
