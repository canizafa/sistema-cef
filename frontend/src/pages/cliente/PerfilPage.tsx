import { useEffect, useState } from 'react';
import { Header } from '@/components/layout/Header';
import { useAuth } from '@/context/AuthContext';
import { clienteService, type ClienteResponse } from '@/services/cliente.service';

function CampoInfo({ label, valor }: { label: string; valor: string }) {
    return (
        <div className='space-y-1'>
            <p className='text-xs text-gray-500 font-medium uppercase tracking-wide'>{label}</p>
            <p className='text-sm text-gray-900'>{valor}</p>
        </div>
    );
}

export function PerfilPage() {
    const { user } = useAuth();
    const [perfil, setPerfil] = useState<ClienteResponse | null>(null);
    const [loading, setLoading] = useState(true);
    const [error, setError] = useState<string | null>(null);

    useEffect(() => {
        if (!user) return;
        clienteService
            .getPerfil(user.dni)
            .then(setPerfil)
            .catch(() => setError('No se pudo cargar el perfil.'))
            .finally(() => setLoading(false));
    }, [user]);

    return (
        <div className='min-h-screen flex flex-col'>
            <Header />
            <main className='flex-1 flex items-center justify-center px-4 py-12'>
                <div className='w-full max-w-md space-y-6'>
                    <h1 className='text-2xl font-bold text-center'>Mi perfil</h1>

                    {loading && (
                        <p className='text-sm text-gray-500 text-center'>Cargando...</p>
                    )}

                    {error && (
                        <p className='text-sm text-red-600 text-center'>{error}</p>
                    )}

                    {perfil && (
                        <>
                            <div className='border border-gray-200 rounded-lg p-5 space-y-4'>
                                <h2 className='text-sm font-semibold text-gray-700'>Datos personales</h2>
                                <CampoInfo label='Nombre y apellido' valor={perfil.nombre_apellido} />
                                <CampoInfo label='DNI' valor={perfil.dni.toString()} />
                                <CampoInfo label='Email' valor={perfil.email} />
                                <CampoInfo label='Teléfono' valor={perfil.telefono} />
                                <CampoInfo
                                    label='Fecha de nacimiento'
                                    valor={new Date(perfil.fecha_nacimiento).toLocaleDateString('es-AR')}
                                />
                            </div>

                            <div className='border border-gray-200 rounded-lg p-5 space-y-4'>
                                <h2 className='text-sm font-semibold text-gray-700'>Ficha médica</h2>
                                <CampoInfo
                                    label='Enfermedades'
                                    valor={perfil.ficha_medica.enfermedades ? 'Sí' : 'No'}
                                />
                                <CampoInfo
                                    label='Operaciones quirúrgicas'
                                    valor={perfil.ficha_medica.operaciones_quirurgicas ? 'Sí' : 'No'}
                                />
                                {perfil.ficha_medica.detalle && (
                                    <CampoInfo label='Detalle' valor={perfil.ficha_medica.detalle} />
                                )}
                            </div>
                        </>
                    )}
                </div>
            </main>
        </div>
    );
}
