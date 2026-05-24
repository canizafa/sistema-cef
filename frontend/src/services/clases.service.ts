import api from './api';

export type EstadoClase = 'activa' | 'inactiva';

export type ClaseDTO = {
    id_clase: number;
    dia: string;
    horario: string;
    estado: EstadoClase;
};

export type NuevaClaseData = {
    dia: string;
    horario: string;
    cupo_profe: number;
    cupo_maximo: number;
    estado: EstadoClase;
    id_actividad: number;
    id_sala: number;
    dni_profesor: number;
};

export type NuevaReservaData = {
    fecha: string;
    estado: string;
    dni_cliente: number;
    id_clase: number;
};

export type NuevaListaEsperaData = {
    dni_cliente: number;
    id_clase: number;
    fecha: string;
};

export const clasesService = {
    async getClases(): Promise<ClaseDTO[]> {
        const response = await api.get('/clases');
        return response.data;
    },

    async crearClase(data: NuevaClaseData) {
        const response = await api.post('/clases', data);
        return response.data;
    },
};

export const reservasService = {
    async crearReserva(data: NuevaReservaData) {
        const response = await api.post('/reservas', data);
        return response.data;
    },

    async cancelarReserva(idReserva: number) {
        const response = await api.delete(`/reservas/${idReserva}`);
        return response.data;
    },
};

export const listaEsperaService = {
    async anotarse(data: NuevaListaEsperaData) {
        const response = await api.post('/lista-espera', data);
        return response.data;
    },
};