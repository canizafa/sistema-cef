// Formulario de registro para nuevos clientes. Solo clientes se auto-registran; los empleados los crea el dueño.
// Llama a authService.register() y redirige al login al completarse.
import { useState } from 'react';
import { Link, useNavigate } from 'react-router-dom';
import { Header } from '@/components/layout/Header';
import { authService } from '@/services/auth.service';

export function RegisterPage() {
    const [form, setForm] = useState({
        nombre: '', apellido: '', dni: '', mail: '', password: '',
        fecha_nacimiento: '', telefono: '',
        estado: 'activo', // hardcodeado, el usuario siempre empieza activo
        ficha_medica: {
            enfermedades: false,
            operaciones_quirurgicas: false,
            detalles: '',
        }
    });
    const [error, setError] = useState<string | null>(null);
    const [loading, setLoading] = useState(false);
    const navigate = useNavigate();

    // Handler para inputs de texto normales
    function handleChange(e: React.ChangeEvent<HTMLInputElement | HTMLTextAreaElement>) {
        setForm((prev) => ({ ...prev, [e.target.name]: e.target.value }));
    }

    // Handler para checkboxes de ficha médica
    function handleFichaMedica(e: React.ChangeEvent<HTMLInputElement | HTMLTextAreaElement>) {
    const { name, type } = e.target;
    const value = type === 'checkbox' ? (e.target as HTMLInputElement).checked : e.target.value;
        setForm((prev) => ({
            ...prev,
            ficha_medica: {
                ...prev.ficha_medica,
                [name]: value,
            }
        }));
    }

    async function handleSubmit(e: React.FormEvent) {
        e.preventDefault();
        setError(null);
        setLoading(true);
        try {
            await authService.register(form);
            navigate('/login'); // redirige al login para que inicie sesión
        } catch {
            setError('Error al crear la cuenta. Revisá los datos.');
        } finally {
            setLoading(false);
        }
    }

    return (
        <div className="min-h-screen flex flex-col">
            <Header />
            <main className="flex-1 flex items-center justify-center px-4 py-12">
                <div className="w-full max-w-sm">
                    <h1 className="text-2xl font-bold mb-6 text-center">Crear cuenta</h1>
                    <form onSubmit={handleSubmit} className="space-y-4">
                        <div className="space-y-1">
                            <label htmlFor="nombre" className="text-sm font-medium">Nombre</label>
                            <input id="nombre" name="nombre" placeholder="Juan" value={form.nombre} onChange={handleChange} required className="flex h-10 w-full rounded-md border border-input bg-background px-3 py-2 text-sm" />
                        </div>
                        <div className="space-y-1">
                            <label htmlFor="apellido" className="text-sm font-medium">Apellido</label>
                            <input id="apellido" name="apellido" placeholder="García" value={form.apellido} onChange={handleChange} required className="flex h-10 w-full rounded-md border border-input bg-background px-3 py-2 text-sm" />
                        </div>
                        <div className="space-y-1">
                            <label htmlFor="dni" className="text-sm font-medium">DNI</label>
                            <input id="dni" name="dni" placeholder="12345678" value={form.dni} onChange={handleChange} required className="flex h-10 w-full rounded-md border border-input bg-background px-3 py-2 text-sm" />
                        </div>
                        <div className="space-y-1">
                            <label htmlFor="mail" className="text-sm font-medium">Email</label>
                            <input id="mail" name="mail" type="email" placeholder="tu@email.com" value={form.mail} onChange={handleChange} required className="flex h-10 w-full rounded-md border border-input bg-background px-3 py-2 text-sm" />
                        </div>
                        <div className="space-y-1">
                            <label htmlFor="password" className="text-sm font-medium">Contraseña</label>
                            <input id="password" name="password" type="password" placeholder="••••••••" value={form.password} onChange={handleChange} required className="flex h-10 w-full rounded-md border border-input bg-background px-3 py-2 text-sm" />
                        </div>
                        <div className="space-y-1">
                            <label htmlFor="fecha_nacimiento" className="text-sm font-medium">Fecha de nacimiento</label>
                            <input id="fecha_nacimiento" name="fecha_nacimiento" type="date" value={form.fecha_nacimiento} onChange={handleChange} required className="flex h-10 w-full rounded-md border border-input bg-background px-3 py-2 text-sm" />
                        </div>
                        <div className="space-y-1">
                            <label htmlFor="telefono" className="text-sm font-medium">Teléfono</label>
                            <input id="telefono" name="telefono" placeholder="2615001234" value={form.telefono} onChange={handleChange} required className="flex h-10 w-full rounded-md border border-input bg-background px-3 py-2 text-sm" />
                        </div>

                        {/* Ficha médica */}
                        <div className="space-y-2 border rounded-md p-3">
                            <p className="text-sm font-medium">Ficha médica</p>
                            <label className="flex items-center gap-2 text-sm">
                                <input type="checkbox" name="enfermedades" checked={form.ficha_medica.enfermedades} onChange={handleFichaMedica} />
                                ¿Tiene enfermedades?
                            </label>
                            <label className="flex items-center gap-2 text-sm">
                                <input type="checkbox" name="operaciones_quirurgicas" checked={form.ficha_medica.operaciones_quirurgicas} onChange={handleFichaMedica} />
                                ¿Tuvo operaciones quirúrgicas?
                            </label>
                            <div className="space-y-1">
                                <label htmlFor="detalles" className="text-sm">Detalles</label>
                                <textarea id="detalles" name="detalles" value={form.ficha_medica.detalles} onChange={handleFichaMedica} placeholder="Ninguno" className="flex w-full rounded-md border border-input bg-background px-3 py-2 text-sm min-h-16" />
                            </div>
                        </div>

                        {error && <p className="text-sm text-red-600">{error}</p>}
                        <button type="submit" disabled={loading} className="w-full bg-brand text-white rounded-md h-10 text-sm font-medium hover:opacity-90 disabled:opacity-50">
                            {loading ? 'Creando cuenta...' : 'Crear cuenta'}
                        </button>
                    </form>
                    <p className="text-sm text-center text-gray-500 mt-4">
                        ¿Ya tenés cuenta?{' '}
                        <Link to="/login" className="text-brand hover:underline">Iniciar sesión</Link>
                    </p>
                </div>
            </main>
        </div>
    );
}
