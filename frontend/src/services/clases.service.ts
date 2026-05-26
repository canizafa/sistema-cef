import api from './api';

export type EstadoClase = 'alta' | 'baja';

export type ClaseDTO = {
    id_clase: string;
    dia: string;
    horario: string;
    estado: EstadoClase;
    lleno: boolean;
    descripcion: string;
    id_actividad: string;
};

export type NuevaClaseData = {
    dia: string;
    horario: string;
    cupo_profe: number;
    cupo_maximo: number;
    estado: EstadoClase;
    id_actividad: string;
    id_sala: string;
    dni_profesor: number;
    descripcion: string;
};

export type NuevaReservaData = {
    fecha: string;
    estado: string;
    dni_cliente: number;
    id_clase: string;
};

export type NuevaListaEsperaData = {
    dni_cliente: number;
    id_clase: string;
    fecha: string;
};

export const clasesService = {
    async getClases(): Promise<ClaseDTO[]> {
        const response = await api.get('/clase/get-all');
        return response.data.clases;
    },

    async crearClase(data: NuevaClaseData) {
        const response = await api.post('/clase/create', data);
        return response.data;
    },

    async getClase(params: unknown) {
        const response = await api.get('/clase/get-clase', { params });
        return response.data;
    },

    async actualizarClase(data: unknown) {
        const response = await api.post('/clase/update-clase', data);
        return response.data;
    },

    async eliminarClase(data: unknown) {
        const response = await api.post('/clase/delete-clase', data);
        return response.data;
    },
};

export const reservasService = {
    async crearReserva(data: NuevaReservaData) {
        const response = await api.post('/reservas/create', data);
        return response.data;
    },

    async cancelarReserva(idReserva: string) {
        const response = await api.delete(`/reservas/delete-reserva/${idReserva}`);
        return response.data;
    },
};

export const listaEsperaService = {
    async anotarse(data: NuevaListaEsperaData) {
        const response = await api.post('/lista-espera', data);
        return response.data;
    },
};
