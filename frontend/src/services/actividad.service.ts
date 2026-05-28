import api from './api';

export interface Actividad {
    id: string;
    nombre: string;
    descripcion: string;
}

export const actividadService = {
    async getActividades(): Promise<Actividad[]> {
        const response = await api.get('/actividades/get-actividades');
        return response.data;
    },

    async getActividad(id: string): Promise<Actividad> {
        const response = await api.get(`/actividades/get-actividad/${id}`);
        return response.data;
    },
};