import api from './api';

export type EstadoClase = 'alta' | 'baja';

export type ClaseDTO = {
    id_clase: string;
    dia: string;
    dia_semana: string;
    horario: string;
    estado: EstadoClase;
    lleno: boolean;
    descripcion: string;
    id_actividad: string;
    id_sala: string;
    dni_profesor: number;
};

export type NuevaClaseData = {
    dia: string;
    horario: string;
    cupo_base: number;
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

export type ReservaResponse = {
    fecha: string;
    tipo: string;
    estado: string;
    dni_cliente: string;
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
        return response.data.clases ?? response.data;
    },

    async crearClase(data: NuevaClaseData) {
        const response = await api.post('/clase/create', data);
        return response.data;
    },

    async getClase(id: string) {
        const response = await api.get(`/clase/get-clase/${id}`);
        return response.data;
    },

    async actualizarClase(id: string, data: unknown) {
        const response = await api.put(`/clase/update-clase/${id}`, data);
        return response.data;
    },

    async eliminarClase(id: string) {
        const response = await api.delete(`/clase/delete-clase/${id}`);
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

    async getReservas(): Promise<ReservaResponse[]> {
        const response = await api.get('/reservas/get-all');
        return response.data.reservas ?? response.data;
    },
};

export const listaEsperaService = {
    async anotarse(data: NuevaListaEsperaData) {
        const response = await api.post('/lista-espera', data);
        return response.data;
    },
};