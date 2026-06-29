import api from './api';

export interface RecaudacionResponse {
    total: number;
}

export interface ClaseMasConcurridaResponse {
    id_clase: string;
    descripcion: string;
    cantidad: number;
}

export interface ClaseMasCanceladaResponse {
    id_clase: string;
    descripcion: string;
    cantidad: number;
}

export const estadisticasService = {
    async getRecaudacion(fechaDesde: string, fechaHasta: string): Promise<RecaudacionResponse> {
        const response = await api.get('/estadisticas/get-recaudacion', {
            params: { fecha_desde: fechaDesde, fecha_hasta: fechaHasta },
        });
        return response.data;
    },

    async getClaseMasConcurrida(fechaDesde: string, fechaHasta: string): Promise<ClaseMasConcurridaResponse> {
        const response = await api.get('/estadisticas/get-clase-mas-concurrida', {
            params: { fecha_desde: fechaDesde, fecha_hasta: fechaHasta },
        });
        return response.data;
    },

    async getClaseMasCancelada(fechaDesde: string, fechaHasta: string): Promise<ClaseMasCanceladaResponse> {
        const response = await api.get('/estadisticas/get-clase-mas-cancelada', {
            params: { fecha_desde: fechaDesde, fecha_hasta: fechaHasta },
        });
        return response.data;
    },
};
