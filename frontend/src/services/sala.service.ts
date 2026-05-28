import api from './api';

export interface Sala {
    id: string;
    numero: number;
    capacidad_maxima: number;
}

export const salaService = {
    async getSalas(): Promise<Sala[]> {
        const response = await api.get('/salas/get-all/');
        return response.data;
    },

    async getSala(id: string): Promise<Sala> {
        const response = await api.get(`/salas/get-sala/${id}`);
        return response.data;
    },
};