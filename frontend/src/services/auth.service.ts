import api from './api';

export interface LoginData {
    email: string;
    password: string;
    rol: string;
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

    logout(): void {
        localStorage.removeItem('token');
        localStorage.removeItem('user');
    },

};