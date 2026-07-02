import { useState, useEffect } from 'react';
import { useNavigate } from 'react-router-dom';
import { clasesService, type EstadoClase } from '@/services/clases.service';
import { profesorService, type Profesor } from '@/services/profesor.service';
import { salaService, type Sala } from '@/services/sala.service';
import { actividadService, type Actividad } from '@/services/actividad.service';
import { Button } from '@/components/ui/button';
import { Input } from '@/components/ui/input';
import { Label } from '@/components/ui/label';
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from '@/components/ui/select';

export function NuevaClasePage() {
    const [form, setForm] = useState({
        dia: '',
        horario: '',
        cupo_base: '',
        id_actividad: '',
        id_sala: '',
        dni_profesor: '',
        descripcion: '',
    });
    const [profesores, setProfesores] = useState<Profesor[]>([]);
    const [salas, setSalas] = useState<Sala[]>([]);
    const [actividades, setActividades] = useState<Actividad[]>([]);
    const [error, setError] = useState<string | null>(null);
    const [success, setSuccess] = useState<string | null>(null);
    const [loading, setLoading] = useState(false);

    const navigate = useNavigate();

    useEffect(() => {
        profesorService.getProfesores()
            .then((data) => setProfesores(data.filter((p) => p.estado === 'alta')))
            .catch(() => setError('No se pudieron cargar los profesores'));

        salaService.getSalas()
            .then(setSalas)
            .catch(() => setError('No se pudieron cargar las salas'));

        actividadService.getActividades()
            .then(setActividades)
            .catch(() => setError('No se pudieron cargar las actividades'));
    }, []);

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

        if (!form.dni_profesor) {
            setError('Seleccioná un profesor');
            return;
        }
        if (!form.id_sala) {
            setError('Seleccioná una sala');
            return;
        }
        if (!form.id_actividad) {
            setError('Seleccioná una actividad');
            return;
        }

        const fechaSeleccionada = new Date(`${form.dia}T${form.horario}`);
        const ahora = new Date();
        if (fechaSeleccionada <= ahora) {
            setError('No se puede crear una clase en una fecha del pasado');
            return;
        }

        const esDomingo = fechaSeleccionada.getDay() === 0; // 0 = domingo en JS

        const [horas, minutos] = form.horario.split(':').map(Number);
        const minutosDesdeMedianoche = horas * 60 + minutos;
        const apertura = 7 * 60;   // 07:00
        const cierre = 21 * 60;    // 21:00 (la clase puede arrancar hasta esa hora; el gym cierra a las 22)
        const fueraDeHorario = minutosDesdeMedianoche < apertura || minutosDesdeMedianoche > cierre;

        if (esDomingo || fueraDeHorario) {
            setError('Clase fuera de horario o día permitido.');
            return;
        }

        const salaSeleccionada = salas.find((s) => String(s.id) === form.id_sala);
        if (salaSeleccionada) {
            if (Number(form.cupo_base) > salaSeleccionada.capacidad_maxima) {
                setError('La clase no pudo darse de alta porque el cupo base ingresado supera al máximo de la sala');
                return;
            }
        }

        try {
            const clases = await clasesService.getClases();

            const salaOcupada = clases.some(
                (c) => c.id_sala === form.id_sala && c.dia === form.dia && c.horario === form.horario
            );
            if (salaOcupada) {
                setError('Sala no disponible para ese día y horario');
                return;
            }

            const profesorOcupado = clases.some(
                (c) => c.dni_profesor === Number(form.dni_profesor) && c.dia === form.dia && c.horario === form.horario
            );
            if (profesorOcupado) {
                setError('El profesor ya cuenta con una clase en el día y horario seleccionados');
                return;
            }
        } catch {
            return;
        }

        setLoading(true);
        try {
            await clasesService.crearClase({
                dia: form.dia,
                horario: form.horario,
                cupo_base: Number(form.cupo_base),
                cupo_maximo: Number(form.cupo_base),
                estado: 'alta' as EstadoClase,
                id_actividad: String(form.id_actividad),
                id_sala: String(form.id_sala),
                dni_profesor: Number(form.dni_profesor),
                descripcion: form.descripcion,
            });
            setSuccess('Clase creada correctamente');
            setTimeout(() => navigate('/admin/clases'), 1500);
        } catch {
            // sin mensaje
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
                        <Label htmlFor="cupo_base">Cupo base</Label>
                        <Input id="cupo_base" name="cupo_base" type="number" min="1" placeholder="15" value={form.cupo_base} onChange={handleChange} required />
                    </div>

                    <div className="space-y-1">
                        <Label htmlFor="descripcion">Descripción</Label>
                        <Input id="descripcion" name="descripcion" placeholder="Ej: Clase de yoga para principiantes" value={form.descripcion} onChange={handleChange} required />
                    </div>

                    <div className="space-y-1">
                        <Label>Actividad</Label>
                        <Select
                            value={form.id_actividad}
                            onValueChange={(v) => handleSelect('id_actividad', v)}
                            disabled={actividades.length === 0}
                        >
                            <SelectTrigger>
                                <SelectValue placeholder={actividades.length === 0 ? 'No hay actividades cargadas' : 'Seleccioná una actividad'} />
                            </SelectTrigger>
                            <SelectContent>
                                {actividades.map((a) => (
                                    <SelectItem key={a.id} value={String(a.id)}>
                                        {a.nombre}
                                    </SelectItem>
                                ))}
                            </SelectContent>
                        </Select>
                    </div>

                    <div className="space-y-1">
                        <Label>Sala</Label>
                        <Select
                            value={form.id_sala}
                            onValueChange={(v) => handleSelect('id_sala', v)}
                            disabled={salas.length === 0}
                        >
                            <SelectTrigger>
                                <SelectValue placeholder={salas.length === 0 ? 'No hay salas cargadas' : 'Seleccioná una sala'} />
                            </SelectTrigger>
                            <SelectContent>
                                {salas.map((s) => (
                                    <SelectItem key={s.id} value={String(s.id)}>
                                        Sala {s.numero} (cap. {s.capacidad_maxima})
                                    </SelectItem>
                                ))}
                            </SelectContent>
                        </Select>
                    </div>

                    <div className="space-y-1">
                        <Label>Profesor</Label>
                        <Select
                            value={form.dni_profesor}
                            onValueChange={(v) => handleSelect('dni_profesor', v)}
                            disabled={profesores.length === 0}
                        >
                            <SelectTrigger>
                                <SelectValue placeholder={profesores.length === 0 ? 'No hay profesores activos' : 'Seleccioná un profesor'} />
                            </SelectTrigger>
                            <SelectContent>
                                {profesores.map((p) => (
                                    <SelectItem key={p.dni} value={String(p.dni)}>
                                        {p.nombre_completo}
                                    </SelectItem>
                                ))}
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