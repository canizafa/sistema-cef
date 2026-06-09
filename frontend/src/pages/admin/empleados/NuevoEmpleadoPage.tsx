import { useState } from 'react';
import { useNavigate } from 'react-router-dom';
import { empleadoService } from '@/services/empleados.service';

export function NuevoEmpleadoPage() {
    const navigate = useNavigate();
    const [form, setForm] = useState({
        nombre: '', apellido: '', dni: '', mail: '', password: '', rol: 'empleado'
    });
    const [error, setError] = useState<string | null>(null);
    const [success, setSuccess] = useState<string | null>(null);
    const [loading, setLoading] = useState(false);

    function handleChange(e: React.ChangeEvent<HTMLInputElement | HTMLSelectElement>) {
        setForm((prev) => ({ ...prev, [e.target.name]: e.target.value }));
    }

    async function handleSubmit(e: React.FormEvent) {
        e.preventDefault();
        setError(null);
        setSuccess(null);
        setLoading(true);
        try {
            await empleadoService.registrarEmpleado({
                nombre_apellido: `${form.nombre} ${form.apellido}`,
                dni: Number(form.dni),
                mail: form.mail,
                password: form.password,
                genero: 'otro',
                estado: 'alta',
                rol: 'empleado',
            });
            setSuccess('Empleado dado de alta en el sistema.');
            setForm({ nombre: '', apellido: '', dni: '', mail: '', password: '', rol: 'empleado' });
            setTimeout(() => navigate('/admin/empleados'), 2000);
        } catch {
            setError('Error al registrar el empleado. Revisá los datos.');
        } finally {
            setLoading(false);
        }
    }

    return (
        <main className="flex-1 flex items-center justify-center px-4 py-12">
            <div className="w-full max-w-sm">
                <h1 className="text-2xl font-bold mb-6 text-center">Registrar empleado</h1>
                <form onSubmit={handleSubmit} className="space-y-4">
                    <div className="space-y-1">
                        <label htmlFor="nombre" className="text-sm font-medium">Nombre</label>
                        <input id="nombre" name="nombre" placeholder="Ana" value={form.nombre} onChange={handleChange} required className="flex h-10 w-full rounded-md border border-input bg-background px-3 py-2 text-sm" />
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
                        <input id="mail" name="mail" type="email" placeholder="empleado@cef.com" value={form.mail} onChange={handleChange} required className="flex h-10 w-full rounded-md border border-input bg-background px-3 py-2 text-sm" />
                    </div>
                    <div className="space-y-1">
                        <label htmlFor="password" className="text-sm font-medium">Contraseña inicial</label>
                        <input id="password" name="password" type="password" placeholder="••••••••" value={form.password} onChange={handleChange} required className="flex h-10 w-full rounded-md border border-input bg-background px-3 py-2 text-sm" />
                    </div>
                    {error && <p className="text-sm text-red-600">{error}</p>}
                    {success && <p className="text-sm text-green-600">{success}</p>}
                    <button type="submit" disabled={loading} className="w-full bg-brand text-white rounded-md h-10 text-sm font-medium hover:opacity-90 disabled:opacity-50">
                        {loading ? 'Registrando...' : 'Registrar empleado'}
                    </button>
                </form>
            </div>
        </main>
    );
}