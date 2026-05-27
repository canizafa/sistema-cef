// Página para crear una nueva clase.
// Solo accesible para recepcionista y dueño (protegido desde App.tsx).
import { useState } from 'react';
import { useNavigate } from 'react-router-dom';
import { clasesService, type EstadoClase } from '@/services/clases.service';
import { Button } from '@/components/ui/button';
import { Input } from '@/components/ui/input';
import { Label } from '@/components/ui/label';
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from '@/components/ui/select';

export function NuevaClasePage() {
    const [form, setForm] = useState({
        dia: '',
        horario: '',
        cupo_profe: '',
        cupo_maximo: '',
        estado: '' as EstadoClase | '',
        id_actividad: '',
        id_sala: '',
        dni_profesor: '',
        descripcion: '',
    });
    const [error, setError] = useState<string | null>(null);
    const [success, setSuccess] = useState<string | null>(null);
    const [loading, setLoading] = useState(false);

    const navigate = useNavigate();

    function handleChange(e: React.ChangeEvent<HTMLInputElement>) {
        setForm((prev) => ({ ...prev, [e.target.name]: e.target.value }));
    }

    function handleSelect(name: string, value: string) {
        setForm((prev) => ({ ...prev, [name]: value }));
    }

    async function handleSubmit(e: React.FormEvent) {
        e.preventDefault();
        setError(null);
        setSuccess(null);
        setLoading(true);
        try {
            await clasesService.crearClase({
                dia: form.dia,
                horario: form.horario,
                cupo_profe: Number(form.cupo_profe),
                cupo_maximo: Number(form.cupo_maximo),
                estado: form.estado as EstadoClase,
                id_actividad: String(form.id_actividad),
                id_sala: String(form.id_sala),
                dni_profesor: Number(form.dni_profesor),
                descripcion: form.descripcion,
            });
            setSuccess('Clase creada correctamente');
            setTimeout(() => navigate('/admin/clases'), 1500);
        } catch {
            setError('Error al crear la clase. Revisá los datos.');
        } finally {
            setLoading(false);
        }
    }

    return (
        <div className="flex-1 flex items-center justify-center px-4 py-12">
            <div className="w-full max-w-sm">
                <h1 className="text-2xl font-bold mb-6 text-center">Nueva clase</h1>
                <form onSubmit={handleSubmit} className="space-y-4">

                    <div className="space-y-1">
                        <Label htmlFor="dia">Fecha</Label>
                        <Input id="dia" name="dia" type="date" value={form.dia} onChange={handleChange} required />
                    </div>

                    <div className="space-y-1">
                        <Label htmlFor="horario">Horario</Label>
                        <Input id="horario" name="horario" type="time" value={form.horario} onChange={handleChange} required />
                    </div>

                    <div className="space-y-1">
                        <Label htmlFor="cupo_maximo">Cupo máximo</Label>
                        <Input id="cupo_maximo" name="cupo_maximo" type="number" min="1" placeholder="15" value={form.cupo_maximo} onChange={handleChange} required />
                    </div>

                    <div className="space-y-1">
                        <Label htmlFor="cupo_profe">Cupo reservado para profesor</Label>
                        <Input id="cupo_profe" name="cupo_profe" type="number" min="0" placeholder="1" value={form.cupo_profe} onChange={handleChange} required />
                        <p className="text-xs text-muted">Lugares reservados fuera del cupo general</p>
                    </div>

                    <div className="space-y-1">
                        <Label htmlFor="descripcion">Descripción</Label>
                        <Input id="descripcion" name="descripcion" placeholder="Ej: Clase de yoga para principiantes" value={form.descripcion} onChange={handleChange} required />
                    </div>

                    <div className="space-y-1">
                        <Label htmlFor="id_actividad">ID de actividad</Label>
                        <Input id="id_actividad" name="id_actividad" placeholder="act-001" value={form.id_actividad} onChange={handleChange} required />
                    </div>

                    <div className="space-y-1">
                        <Label htmlFor="id_sala">ID de sala</Label>
                        <Input id="id_sala" name="id_sala" placeholder="sala-001" value={form.id_sala} onChange={handleChange} required />
                    </div>

                    <div className="space-y-1">
                        <Label htmlFor="dni_profesor">DNI del profesor</Label>
                        <Input id="dni_profesor" name="dni_profesor" type="number" min="1000000" placeholder="30123456" value={form.dni_profesor} onChange={handleChange} required />
                    </div>

                    <div className="space-y-1">
                        <Label>Estado</Label>
                        <Select value={form.estado} onValueChange={(v) => handleSelect('estado', v)} required>
                            <SelectTrigger>
                                <SelectValue placeholder="Seleccioná un estado" />
                            </SelectTrigger>
                            <SelectContent>
                                <SelectItem value="alta">Alta</SelectItem>
                                <SelectItem value="baja">Baja</SelectItem>
                            </SelectContent>
                        </Select>
                    </div>

                    {error && <p className="text-xs text-destructive">{error}</p>}
                    {success && <p className="text-xs text-success">{success}</p>}

                    <Button type="submit" disabled={loading} className="w-full bg-brand text-white">
                        {loading ? 'Creando...' : 'Crear clase'}
                    </Button>

                </form>
            </div>
        </div>
    );
}