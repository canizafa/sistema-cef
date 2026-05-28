import api from './api';

interface CreatePagoRequest {
    titulo: string;
    monto: number;
    fecha: string;
    hora: string;
    sena: boolean;
    id_membresia: string;
    reserva_paga: string;
}

interface PagoResponse {
    init_point: string;
    sandbox_init_point: string;
}

export const pagosService = {
    async crearPago(data: CreatePagoRequest): Promise<PagoResponse> {
        const response = await api.post<PagoResponse>('/pagos/create', data);
        return response.data;
    },
};
