// Página para crear una nueva clase.
// Solo accesible para recepcionista y dueño (protegido desde App.tsx).
import { useState } from 'react';
import { useNavigate } from 'react-router-dom';
import { clasesService } from '@/services/clases.service';

export function NuevaClasePage() {
    // Un solo objeto de estado para todos los campos del formulario
    const [form, setForm] = useState({
        nombre: '', tipo: '', profesor: '', dias: '', horario: '', duracionMin: '', cupoMaximo: ''
    });
    const [error, setError] = useState<string | null>(null);     // null = sin error
    const [success, setSuccess] = useState<string | null>(null); // null = sin mensaje de éxito
    const [loading, setLoading] = useState(false);

    const navigate = useNavigate();

    // Mismo patrón que NuevoEmpleadoPage: un handler para todos los inputs y el select
    function handleChange(e: React.ChangeEvent<HTMLInputElement | HTMLSelectElement>) {
        setForm((prev) => ({ ...prev, [e.target.name]: e.target.value }));
    }

    async function handleSubmit(e: React.FormEvent) {
        e.preventDefault();    // Evita que el formulario recargue la página
        setError(null);
        setSuccess(null);
        setLoading(true);
        try {
            await clasesService.crearClase({
                ...form,
                duracionMin: Number(form.duracionMin),
                cupoMaximo: Number(form.cupoMaximo),
            });
            setSuccess('Clase creada correctamente');
            // Vuelve a la lista de clases después de crear
            setTimeout(() => navigate('/admin/clases'), 1500);
        } catch {
            setError('Error al crear la clase. Revisá los datos.');
        } finally {
            setLoading(false); // Siempre se ejecuta, haya error o no
        }
    }

    // div en lugar de main porque el AdminLayout ya provee el main
    return (
        <div className='flex-1 flex items-center justify-center px-4 py-12'>
            <div className='w-full max-w-sm'>
                <h1 className='text-2xl font-bold mb-6 text-center'>Nueva clase</h1>
                <form onSubmit={handleSubmit} className='space-y-4'>
                    <div className='space-y-1'>
                        <label htmlFor='nombre' className='text-sm font-medium'>Nombre</label>
                        <input id='nombre' name='nombre' placeholder='CrossFit' value={form.nombre} onChange={handleChange} required className='flex h-10 w-full rounded-md border border-input bg-background px-3 py-2 text-sm' />
                    </div>
                    <div className='space-y-1'>
                        <label htmlFor='tipo' className='text-sm font-medium'>Tipo</label>
                        {/* Select en lugar de input: el tipo se elige de una lista fija */}
                        <select id='tipo' name='tipo' value={form.tipo} onChange={handleChange} required className='flex h-10 w-full rounded-md border border-input bg-background px-3 py-2 text-sm'>
                            <option value='' disabled>Seleccioná un tipo</option>
                            <option value='Fuerza'>Fuerza</option>
                            <option value='Cardio'>Cardio</option>
                            <option value='Flexibilidad'>Flexibilidad</option>
                            <option value='Funcional'>Funcional</option>
                        </select>
                    </div>
                    <div className='space-y-1'>
                        <label htmlFor='profesor' className='text-sm font-medium'>Profesor</label>
                        <input id='profesor' name='profesor' placeholder='Juan Pérez' value={form.profesor} onChange={handleChange} required className='flex h-10 w-full rounded-md border border-input bg-background px-3 py-2 text-sm' />
                    </div>
                    <div className='space-y-1'>
                        <label htmlFor='dias' className='text-sm font-medium'>Días</label>
                        <input id='dias' name='dias' placeholder='Lun/Mié/Vie' value={form.dias} onChange={handleChange} required className='flex h-10 w-full rounded-md border border-input bg-background px-3 py-2 text-sm' />
                    </div>
                    <div className='space-y-1'>
                        <label htmlFor='horario' className='text-sm font-medium'>Horario</label>
                        <input id='horario' name='horario' type='time' value={form.horario} onChange={handleChange} required className='flex h-10 w-full rounded-md border border-input bg-background px-3 py-2 text-sm' />
                    </div>
                    <div className='space-y-1'>
                        <label htmlFor='duracionMin' className='text-sm font-medium'>Duración (minutos)</label>
                        <input id='duracionMin' name='duracionMin' type='number' min='1' placeholder='60' value={form.duracionMin} onChange={handleChange} required className='flex h-10 w-full rounded-md border border-input bg-background px-3 py-2 text-sm' />
                    </div>
                    <div className='space-y-1'>
                        <label htmlFor='cupoMaximo' className='text-sm font-medium'>Cupo máximo</label>
                        <input id='cupoMaximo' name='cupoMaximo' type='number' min='1' placeholder='15' value={form.cupoMaximo} onChange={handleChange} required className='flex h-10 w-full rounded-md border border-input bg-background px-3 py-2 text-sm' />
                    </div>
                    {error && <p className='text-sm text-red-600'>{error}</p>}
                    {/* Mensaje de éxito en verde cuando la clase se creó correctamente */}
                    {success && <p className='text-sm text-green-600'>{success}</p>}
                    <button type='submit' disabled={loading} className='w-full bg-brand text-white rounded-md h-10 text-sm font-medium hover:opacity-90 disabled:opacity-50'>
                        {loading ? 'Creando...' : 'Crear clase'}
                    </button>
                </form>
            </div>
        </div>
    );
}