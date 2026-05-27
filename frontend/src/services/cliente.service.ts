import api from './api';

export interface FichaMedicaResponse {
    id_ficha: string;
    enfermedades: boolean;
    operaciones_quirurgicas: boolean;
    detalle: string;
}

export interface ClienteResponse {
    dni: number;
    nombre_apellido: string;
    email: string;
    telefono: string;
    fecha_nacimiento: string;
    estado: string;
    rol: string;
    ficha_medica: FichaMedicaResponse;
}

export interface UpdateClienteRequest {
    nombre_apellido: string;
    fecha_nacimiento: string;
}

export const clienteService = {
    async getPerfil(dni: number): Promise<ClienteResponse> {
        const response = await api.get<ClienteResponse>(`/clientes/get-cliente/${dni}`);
        return response.data;
    },

    async updatePerfil(dni: number, data: UpdateClienteRequest): Promise<void> {
        await api.put(`/clientes/update-cliente/${dni}`, data);
    },
};
