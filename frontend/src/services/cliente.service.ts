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
    motivo_eliminacion: string | null;
    creditos: number;
}

export interface FichaMedicaResponse {
    id_ficha: string;
    enfermedades: boolean;
    operaciones_quirurgicas: boolean;
    detalle: string;
}

export interface PerfilResponse {
    cliente: ClienteResponse;
    ficha_medica: FichaMedicaResponse;
}

export interface UpdateClienteRequest {
    dni: number;
    nombre_apellido: string;
    email: string;
    telefono: string;
    fecha_nacimiento: string;
    motivo_eliminacion: string | null;
    estado: string;
    id_ficha: string;
}

export const clienteService = {
    async getClientes(): Promise<ClienteResponse[]> {
        const response = await api.get<ClienteResponse[]>('/clientes/get-all');
        return response.data;
    },

    async getPerfil(dni: number): Promise<PerfilResponse> {
        const response = await api.get<[ClienteResponse, FichaMedicaResponse]>(`/clientes/get-cliente/${dni}`);
        return {
            cliente: response.data[0],
            ficha_medica: response.data[1],
        };
    },

    async updatePerfil(data: UpdateClienteRequest): Promise<ClienteResponse> {
        const response = await api.put<ClienteResponse>('/clientes/update-cliente', data);
        return response.data;
    },

    async eliminarCliente(dni: number, motivo: string): Promise<void> {
        await api.delete(`/clientes/delete-cliente/${dni}`, {
            data: {
                dni: dni,
                estado: 'baja',
                motivo_eliminacion: motivo,
            },
            headers: { 'Content-Type': 'application/json' }
        });
    },

    async usarCreditos(dni: number, monto: number): Promise<void> {
        await api.post('/clientes/usar-creditos', { dni, monto });
    },
};