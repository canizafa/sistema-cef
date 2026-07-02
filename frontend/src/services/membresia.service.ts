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

function fechaFinA30Dias(): { hoy: string; fin: string } {
    const hoy = new Date().toLocaleDateString('en-CA');
    const fechaFinDate = new Date();
    fechaFinDate.setDate(fechaFinDate.getDate() + 30);
    return { hoy, fin: fechaFinDate.toLocaleDateString('en-CA') };
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

    async crearMembresia(tipo: string, dni: number, idActividad: string, horario: string): Promise<void> {
        const { hoy, fin } = fechaFinA30Dias();
        await api.post('/membresias/create', {
            tipo,
            id_actividad: idActividad,
            horario,
            estado: 'activo',
            fecha_inicio: hoy,
            fecha_fin: fin,
            dni_cliente: dni,
            aceptar_espera: false,
        });
    },

    async renovarMembresia(idMembresia: string, tipo: string, dni: number, idActividad: string, horario: string): Promise<void> {
        const { hoy, fin } = fechaFinA30Dias();
        await api.put(`/membresias/update-membresia/${idMembresia}`, {
            tipo,
            id_actividad: idActividad,
            horario,
            estado: 'activo',
            fecha_inicio: hoy,
            fecha_fin: fin,
            dni_cliente: dni,
            aceptar_espera: false,
        });
    },

    async darBajaMembresia(membresia: MembresiaResponse, dni: number): Promise<void> {
        await api.put(`/membresias/update-membresia/${membresia.id_membresia}`, {
            tipo: membresia.tipo,
            id_actividad: membresia.id_actividad,
            estado: 'cancelado',
            fecha_inicio: membresia.fecha_inicio.slice(0, 10),
            fecha_fin: membresia.fecha_fin,
            dni_cliente: dni,
        });
    },
};
