import api from './api';

export type EstadoMembresia = 'alta' | 'baja';

export interface MembresiaResponse {
    id_membresia: string;
    tipo_actividad: string;
    dni_cliente: number;
    estado: EstadoMembresia;
    id_actividad: string;
    fecha_inicio: string;
    fecha_fin: string;
    horario: string;
}

export const membresiaService = {
    async getMembresias(): Promise<MembresiaResponse[]> {
        const response = await api.get('/membresias/get-all');
        return response.data;
    },
};