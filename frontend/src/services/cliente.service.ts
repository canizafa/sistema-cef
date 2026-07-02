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

export interface NotificacionRequest {
    email: string;
    motivo: string;
    cuerpo: string;
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

    // CORREGIDO: Toma el estado actual que viene de la UI e invierte el string de forma segura
    async toggleEstado(cliente: ClienteResponse): Promise<ClienteResponse> {
        const nuevoEstado = cliente.estado === 'alta' ? 'baja' : 'alta';
        
        const response = await api.put<ClienteResponse>('/clientes/update-cliente', {
            dni: cliente.dni,
            nombre_apellido: cliente.nombre_apellido,
            email: cliente.email,
            telefono: cliente.telefono,
            fecha_nacimiento: cliente.fecha_nacimiento,
            motivo_eliminacion: cliente.motivo_eliminacion,
            estado: nuevoEstado, 
            id_ficha: cliente.id_ficha,
        });
        return response.data;
    },

    async eliminarCliente(dni: number, motivo: string): Promise<void> {
        await api.delete(`/clientes/delete-cliente`, {
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

    async programarNotificaciones(dias: number): Promise<void> {
        await api.put('/notificaciones/update-date', { dias });
    },

    // ➡️ CORREGIDO: Se eliminó la palabra "function", quedando estructurado como método válido de objeto TypeScript.
    async enviarNotificacionDirecta(data: NotificacionRequest): Promise<void> {
        await api.post('/api/notificaciones/notify', data);
    },
};