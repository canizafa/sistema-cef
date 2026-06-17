import api from './api';

export interface FichaMedica {
    enfermedades: boolean;
    operaciones_quirurgicas: boolean;
    detalle: string | null;
}

export interface ClienteResponse {
    dni: number;
    nombre_apellido: string;
    email: string;
    telefono: string;
    fecha_nacimiento: string;
    estado: string;
    rol: string;
    id_ficha: string;
    ficha_medica: FichaMedica;
}

export interface UpdatePerfilData {
    dni: number;
    nombre_apellido: string;
    email: string;
    telefono: string;
    fecha_nacimiento: string;
    estado: string;
    rol: string;
    ficha_medica: FichaMedica;
}

export const clienteService = {
    async getClientes(): Promise<ClienteResponse[]> {
        const response = await api.get<ClienteResponse[]>('/clientes/get-all');
        return response.data;
    },

    async getPerfil(dni: number): Promise<ClienteResponse> {
        const response = await api.get<ClienteResponse>(`/clientes/get-cliente/${dni}`);
        return response.data;
    },

    async updatePerfil(dni: number, data: UpdatePerfilData): Promise<ClienteResponse> {
        const response = await api.put<ClienteResponse>(`/clientes/update-cliente/${dni}`, data);
        return response.data;
    },

    async eliminarCliente(dni: number): Promise<void> {
        await api.delete(`/clientes/delete-cliente/${dni}`);
    },
};