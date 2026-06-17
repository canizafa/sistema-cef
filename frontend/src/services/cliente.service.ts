import api from './api';

export interface ClienteResponse {
    dni: number;
    nombre_apellido: string;
    email: string;
    telefono: string;
    fecha_nacimiento: string;
    estado: string;
    rol: string;
    id_ficha: string;
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

    async eliminarCliente(dni: number): Promise<void> {
        await api.delete(`/clientes/delete-cliente/${dni}`);
    },
};