import { useEffect, useState } from 'react';
import axios from 'axios';
import { Header } from '@/components/layout/Header';
import { useAuth } from '@/context/AuthContext';
import { clienteService, type ClienteResponse, type UpdateClienteRequest } from '@/services/cliente.service';
import { authService } from '@/services/auth.service';

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

    const [cambiarForm, setCambiarForm] = useState({ old_password: '', new_password: '' });
    const [cambiarLoading, setCambiarLoading] = useState(false);
    const [cambiarError, setCambiarError] = useState<string | null>(null);
    const [cambiarExito, setCambiarExito] = useState(false);

    const [editando, setEditando] = useState(false);
    const [editForm, setEditForm] = useState<UpdateClienteRequest>({
        nombre_apellido: '', fecha_nacimiento: '',
    });
    const [editLoading, setEditLoading] = useState(false);
    const [editError, setEditError] = useState<string | null>(null);
    const [editExito, setEditExito] = useState(false);

    async function handleCambiarPassword(e: React.FormEvent) {
        e.preventDefault();
        if (!user) return;
        setCambiarError(null);
        setCambiarExito(false);
        setCambiarLoading(true);
        try {
            await authService.changePassword({
                dni_cliente: user.dni,
                old_password: cambiarForm.old_password,
                new_password: cambiarForm.new_password,
            });
            setCambiarExito(true);
            setCambiarForm({ old_password: '', new_password: '' });
        } catch (err) {
            if (axios.isAxiosError(err) && err.response?.status === 401) {
                setCambiarError('Clave actual incorrecta, intente nuevamente.');
            } else {
                setCambiarError('Error al cambiar la contraseña. Intentá de nuevo.');
            }
        } finally {
            setCambiarLoading(false);
        }
    }

    async function handleGuardarPerfil() {
        if (!user) return;
        setEditError(null);
        setEditExito(false);

        const hoy = new Date();
        const fechaMinima = new Date(hoy.getFullYear() - 14, hoy.getMonth(), hoy.getDate());
        if (new Date(editForm.fecha_nacimiento) > fechaMinima) {
            setEditError('Debés tener al menos 14 años.');
            return;
        }

        setEditLoading(true);
        try {
            await clienteService.updatePerfil(user.dni, editForm);
            setPerfil((prev) => prev ? { ...prev, nombre_apellido: editForm.nombre_apellido, fecha_nacimiento: editForm.fecha_nacimiento } : prev);
            setEditando(false);
            setEditExito(true);
        } catch {
            setEditError('Error al guardar los cambios. Intentá de nuevo.');
        } finally {
            setEditLoading(false);
        }
    }

    function handleClickEditar() {
        if (editando) {
            handleGuardarPerfil();
        } else {
            if (perfil) {
                setEditForm({
                    nombre_apellido: perfil.nombre_apellido,
                    fecha_nacimiento: perfil.fecha_nacimiento.slice(0, 10),
                });
            }
            setEditExito(false);
            setEditError(null);
            setEditando(true);
        }
    }

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
                                {editando ? (
                                    <form onSubmit={(e) => { e.preventDefault(); handleGuardarPerfil(); }} className='space-y-3'>
                                        <div className='space-y-1'>
                                            <label htmlFor='edit_nombre' className='text-xs text-gray-500 font-medium uppercase tracking-wide'>Nombre y apellido</label>
                                            <input id='edit_nombre' value={editForm.nombre_apellido} onChange={(e) => setEditForm((p) => ({ ...p, nombre_apellido: e.target.value }))} required className='flex h-10 w-full rounded-md border-2 border-[#C8102E] bg-background px-3 py-2 text-sm' />
                                        </div>
                                        <CampoInfo label='DNI' valor={perfil.dni.toString()} />
                                        <CampoInfo label='Email' valor={perfil.email} />
                                        <CampoInfo label='Teléfono' valor={perfil.telefono} />
                                        <div className='space-y-1'>
                                            <label htmlFor='edit_fecha' className='text-xs text-gray-500 font-medium uppercase tracking-wide'>Fecha de nacimiento</label>
                                            <input id='edit_fecha' type='date' value={editForm.fecha_nacimiento} onChange={(e) => setEditForm((p) => ({ ...p, fecha_nacimiento: e.target.value }))} required className='flex h-10 w-full rounded-md border-2 border-[#C8102E] bg-background px-3 py-2 text-sm' />
                                        </div>
                                        <button type='submit' aria-hidden='true' className='hidden' />
                                    </form>
                                ) : (
                                    <>
                                        <CampoInfo label='Nombre y apellido' valor={perfil.nombre_apellido} />
                                        <CampoInfo label='DNI' valor={perfil.dni.toString()} />
                                        <CampoInfo label='Email' valor={perfil.email} />
                                        <CampoInfo label='Teléfono' valor={perfil.telefono} />
                                        <CampoInfo label='Fecha de nacimiento' valor={new Date(perfil.fecha_nacimiento).toLocaleDateString('es-AR')} />
                                    </>
                                )}
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

                            <div className='space-y-2'>
                                <button
                                    type='button'
                                    onClick={handleClickEditar}
                                    disabled={editLoading}
                                    className='w-full border border-[#C8102E] text-[#C8102E] rounded-md h-10 text-sm font-medium hover:bg-red-50 disabled:opacity-50'
                                >
                                    {editLoading ? 'Guardando...' : 'Editar Datos'}
                                </button>
                                {editError && <p className='text-sm text-red-600'>{editError}</p>}
                                {editExito && <p className='text-sm text-green-600'>Cambio exitoso.</p>}
                            </div>

                            <div className='border border-gray-200 rounded-lg p-5 space-y-4'>
                                <h2 className='text-sm font-semibold text-gray-700'>Cambiar contraseña</h2>
                                <form onSubmit={handleCambiarPassword} className='space-y-3'>
                                    <div className='space-y-1'>
                                        <label htmlFor='old_password' className='text-xs text-gray-500 font-medium uppercase tracking-wide'>Contraseña actual</label>
                                        <input
                                            id='old_password'
                                            type='password'
                                            value={cambiarForm.old_password}
                                            onChange={(e) => setCambiarForm((p) => ({ ...p, old_password: e.target.value }))}
                                            required
                                            className='flex h-10 w-full rounded-md border border-input bg-background px-3 py-2 text-sm'
                                        />
                                    </div>
                                    <div className='space-y-1'>
                                        <label htmlFor='new_password' className='text-xs text-gray-500 font-medium uppercase tracking-wide'>Nueva contraseña</label>
                                        <input
                                            id='new_password'
                                            type='password'
                                            value={cambiarForm.new_password}
                                            onChange={(e) => setCambiarForm((p) => ({ ...p, new_password: e.target.value }))}
                                            required
                                            className='flex h-10 w-full rounded-md border border-input bg-background px-3 py-2 text-sm'
                                        />
                                    </div>
                                    {cambiarError && <p className='text-sm text-red-600'>{cambiarError}</p>}
                                    {cambiarExito && <p className='text-sm text-green-600'>Cambio de contraseña exitoso.</p>}
                                    <button
                                        type='submit'
                                        disabled={cambiarLoading}
                                        className='w-full bg-brand text-white rounded-md h-10 text-sm font-medium hover:opacity-90 disabled:opacity-50'
                                    >
                                        {cambiarLoading ? 'Cambiando...' : 'Cambiar contraseña'}
                                    </button>
                                </form>
                            </div>
                        </>
                    )}
                </div>
            </main>
        </div>
    );
}
