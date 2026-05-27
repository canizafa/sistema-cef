import api from './api';

export interface LoginData {
    email: string;
    password: string;
}

export interface ChangePasswordData {
    dni_cliente: number;
    old_password: string;
    new_password: string;
}

export interface CreateFichaMedicaRequest {

    enfermedades: boolean;

    operaciones_quirurgicas: boolean;

    detalle: string;

}

export interface RegisterData {

    nombre_apellido: string;

    email: string;

    dni: number;

    telefono: string;

    fecha_nacimiento: string;

    password: string;

    estado: string;

    ficha_medica: CreateFichaMedicaRequest;

}

export interface AuthResponse {

    dni: string;

    email: string;

    access_token: string;

    rol: string;

}

export const authService = {

    async login(data: LoginData): Promise<AuthResponse> {

        const response = await api.post('/auth/login', data);

        return response.data;

    },

    async register(data: RegisterData): Promise<void> {

        await api.post('/auth/register-cliente', data);

    },

    async forgotPassword(data: { email: string }): Promise<void> {
        await api.post('/auth/reset-password', data);
    },

    async changePassword(data: ChangePasswordData): Promise<void> {
        await api.put(`/auth/change-password/${data.dni_cliente}`, {
            old_password: data.old_password,
            new_password: data.new_password,
        });
    },

    logout(): void {
        localStorage.removeItem('token');
        localStorage.removeItem('user');
    },

};