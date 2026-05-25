import { Link } from 'react-router-dom';
import { Header } from '@/components/layout/Header';

export function PagoExitoPage() {
    return (
        <div className='min-h-screen flex flex-col'>
            <Header />
            <main className='flex-1 flex items-center justify-center px-4 py-12'>
                <div className='w-full max-w-sm text-center space-y-4'>
                    <h1 className='text-2xl font-bold'>Pago exitoso</h1>
                    <p className='text-sm text-gray-500'>
                        Tu pago fue procesado correctamente. Tu clase fue reservada.
                    </p>
                    <Link to='/clases' className='text-brand hover:underline text-sm block'>
                        Volver a clases
                    </Link>
                </div>
            </main>
        </div>
    );
}
