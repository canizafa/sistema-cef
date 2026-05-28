import api from './api';

export interface ReservaPagaRequest {
    fecha: string;
    tipo: string;
    estado: string;
    dni_cliente: string;
    id_clase: string;
}

export interface CreatePagoRequest {
    titulo: string;
    monto: number;
    fecha: string;
    hora: string;
    sena: boolean;
    id_membresia: string;
    reserva_paga: ReservaPagaRequest;
}

export interface PagoResponse {
    init_point: string;
    sandbox_init_point: string;
}

export const pagoService = {
    async crearPago(data: CreatePagoRequest): Promise<PagoResponse> {
        const response = await api.post('/pagos/create', data);
        return response.data;
    },
};
