// Formulario de registro para nuevos clientes. Solo clientes se auto-registran; los empleados los crea el dueño.
// Llama a authService.register() y redirige al login al completarse.
import { useState } from 'react';
import { Link, useNavigate } from 'react-router-dom';
import { Header } from '@/components/layout/Header';
import { authService } from '@/services/auth.service';

export function RegisterPage() {
    const [form, setForm] = useState({
        nombre_apellido: '',
        email: '',
        dni: '',
        telefono: '',
        fecha_nacimiento: '',
        password: '',
        estado: 'alta',
        enfermedades: false,
        operaciones_quirurgicas: false,
        detalle: '',
    });
    const [error, setError] = useState<string | null>(null);
    const [loading, setLoading] = useState(false);
    const navigate = useNavigate();

    function handleChange(e: React.ChangeEvent<HTMLInputElement | HTMLTextAreaElement>) {
        const { name, value, type } = e.target;
        setForm((prev) => ({
            ...prev,
            [name]: type === 'checkbox' ? (e.target as HTMLInputElement).checked : value,
        }));
    }

    async function handleSubmit(e: React.FormEvent) {
        e.preventDefault();
        setError(null);
        setLoading(true);
        try {
            await authService.register({
                nombre_apellido: form.nombre_apellido,
                email: form.email,
                dni: Number(form.dni),
                telefono: form.telefono,
                fecha_nacimiento: form.fecha_nacimiento,
                password: form.password,
                estado: form.estado,
                ficha_medica: {
                    enfermedades: form.enfermedades,
                    operaciones_quirurgicas: form.operaciones_quirurgicas,
                    detalle: form.detalle,
                },
            });
            navigate('/login');
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
                            <label htmlFor="nombre_apellido" className="text-sm font-medium">Nombre y apellido</label>
                            <input id="nombre_apellido" name="nombre_apellido" placeholder="Juan Pérez" value={form.nombre_apellido} onChange={handleChange} required className="flex h-10 w-full rounded-md border border-input bg-background px-3 py-2 text-sm" />
                        </div>
                        <div className="space-y-1">
                            <label htmlFor="email" className="text-sm font-medium">Email</label>
                            <input id="email" name="email" type="email" placeholder="tu@email.com" value={form.email} onChange={handleChange} required className="flex h-10 w-full rounded-md border border-input bg-background px-3 py-2 text-sm" />
                        </div>
                        <div className="space-y-1">
                            <label htmlFor="password" className="text-sm font-medium">Contraseña</label>
                            <input id="password" name="password" type="password" value={form.password} onChange={handleChange} required className="flex h-10 w-full rounded-md border border-input bg-background px-3 py-2 text-sm" />
                        </div>
                        <div className="space-y-1">
                            <label htmlFor="dni" className="text-sm font-medium">DNI</label>
                            <input id="dni" name="dni" placeholder="12345678" value={form.dni} onChange={handleChange} required className="flex h-10 w-full rounded-md border border-input bg-background px-3 py-2 text-sm" />
                        </div>
                        <div className="space-y-1">
                            <label htmlFor="telefono" className="text-sm font-medium">Teléfono</label>
                            <input id="telefono" name="telefono" placeholder="2615001234" value={form.telefono} onChange={handleChange} required className="flex h-10 w-full rounded-md border border-input bg-background px-3 py-2 text-sm" />
                        </div>
                        <div className="space-y-1">
                            <label htmlFor="fecha_nacimiento" className="text-sm font-medium">Fecha de nacimiento</label>
                            <input id="fecha_nacimiento" name="fecha_nacimiento" type="date" value={form.fecha_nacimiento} onChange={handleChange} required className="flex h-10 w-full rounded-md border border-input bg-background px-3 py-2 text-sm" />
                        </div>

                        {/* Ficha médica */}
                        <fieldset className="space-y-3 rounded-md border border-border p-4">
                            <legend className="text-sm font-medium px-1">Ficha médica</legend>
                            <label className="flex items-center gap-2 text-sm cursor-pointer">
                                <input type="checkbox" name="enfermedades" checked={form.enfermedades} onChange={handleChange} className="h-4 w-4 rounded border-border" />
                                ¿Tenés enfermedades preexistentes?
                            </label>
                            <label className="flex items-center gap-2 text-sm cursor-pointer">
                                <input type="checkbox" name="operaciones_quirurgicas" checked={form.operaciones_quirurgicas} onChange={handleChange} className="h-4 w-4 rounded border-border" />
                                ¿Tuviste operaciones quirúrgicas?
                            </label>
                            <div className="space-y-1">
                                <label htmlFor="detalle" className="text-sm font-medium">Detalle</label>
                                <textarea id="detalle" name="detalle" value={form.detalle} onChange={handleChange} placeholder="Describí antecedentes médicos relevantes" className="flex w-full rounded-md border border-input bg-background px-3 py-2 text-sm min-h-16" />
                            </div>
                        </fieldset>

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