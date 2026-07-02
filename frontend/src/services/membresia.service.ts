import api from './api';

export type ActividadResponse = {
    id: string;
    nombre: string;
    descripcion: string;
};

export type MembresiaResponse = {
    id_membresia: string;
    id_actividad: string;
    tipo: string;
    estado: string;
    fecha_inicio: string;
    fecha_fin: string | null;
};

export const PRECIO_MEMBRESIA = 5000;

export const actividadService = {
    async getActividades(): Promise<ActividadResponse[]> {
        const response = await api.get('/actividades/get-actividades');
        return response.data.actividades ?? response.data;
    },
};

function calcularFechas(diaInicio: string): { inicio: string; fin: string } {
    const fechaInicio = new Date(diaInicio + 'T00:00:00');
    const fechaFin = new Date(fechaInicio);
    fechaFin.setDate(fechaFin.getDate() + 30);
    return {
        inicio: diaInicio,
        fin: fechaFin.toLocaleDateString('en-CA'),
    };
}

export const membresiaService = {
    async getMembresiasPorDni(dni: number): Promise<MembresiaResponse[]> {
        try {
            const response = await api.get(`/membresias/get-membresia-dni/${dni}`);
            return response.data ?? [];
        } catch {
            return [];
        }
    },

    async crearMembresia(tipo: string, dni: number, idActividad: string, horario: string, diaInicio: string): Promise<void> {
        const { inicio, fin } = calcularFechas(diaInicio);
        await api.post('/membresias/create', {
            tipo,
            id_actividad: idActividad,
            horario,
            estado: 'activo',
            fecha_inicio: inicio,
            fecha_fin: fin,
            dni_cliente: dni,
            aceptar_espera: false,
        });
    },

    async renovarMembresia(idMembresia: string, tipo: string, dni: number, idActividad: string, horario: string, diaInicio: string): Promise<void> {
        const { inicio, fin } = calcularFechas(diaInicio);
        await api.put(`/membresias/update-membresia/${idMembresia}`, {
            tipo,
            id_actividad: idActividad,
            horario,
            estado: 'activo',
            fecha_inicio: inicio,
            fecha_fin: fin,
            dni_cliente: dni,
            aceptar_espera: false,
        });
    },

    async darBajaMembresia(membresia: MembresiaResponse): Promise<void> {
        await api.delete(`/membresias/delete-membresia/${membresia.id_membresia}`);
    },
};