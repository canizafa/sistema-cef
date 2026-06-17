import api from './api';

export type ActividadResponse = {
    id: string;
    nombre: string;
    descripcion: string;
};

export type MembresiaResponse = {
    id_membresia: string;
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

export const membresiaService = {
    async getMembresiaPorDni(dni: number): Promise<MembresiaResponse | null> {
        try {
            const response = await api.get(`/membresias/get-membresia-dni/${dni}`);
            return response.data ?? null;
        } catch {
            return null;
        }
    },

    async crearMembresia(tipo: string, dni: number): Promise<void> {
        const hoy = new Date().toLocaleDateString('en-CA');
        const fechaFinDate = new Date();
        fechaFinDate.setDate(fechaFinDate.getDate() + 30);
        const fin = fechaFinDate.toLocaleDateString('en-CA');
        await api.post('/membresias/create', {
            tipo,
            estado: 'alta',
            fecha_inicio: hoy,
            fecha_fin: fin,
            dni_cliente: String(dni),
        });
    },
};
