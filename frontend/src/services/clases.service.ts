import api from './api';

export type EstadoClase = 'alta' | 'baja';

export type ClaseDTO = {
    id_clase: string;
    dia: string;
    dia_semana: string;
    horario: string;
    estado: EstadoClase;
    lleno: boolean;
    cupo_base: number;
    inscripciones: number;
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
    tipo: string;
    estado: string;
    dni_cliente: number;
    id_clase: string;
};

export type ReservaResponse = {
    id_reserva: string;
    fecha: string;
    tipo: string;
    estado: string;
    dni_cliente: string;
    id_clase: string;
};

export type ListaEsperaResponse = {
    id_espera: string;
    tipo: string;
    id_clase: string;
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
    async getAll(): Promise<ListaEsperaResponse[]> {
        const response = await api.get('/lista-espera/get-all');
        return response.data;
    },

    async crearLista(idClase: string, tipo: string): Promise<ListaEsperaResponse> {
        const response = await api.post('/lista-espera/create', { id_clase: idClase, tipo });
        return response.data;
    },

    async anotarse(idClase: string, idEspera: string, dniCliente: number): Promise<void> {
        await api.post(`/lista-espera/insert-user/${idClase}`, {
            id_espera: idEspera,
            dni_cliente: dniCliente,
            fecha_ingreso: new Date().toLocaleDateString('en-CA'),
        });
    },
};