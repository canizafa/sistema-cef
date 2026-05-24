import api from './api';
import type { User } from '../context/AuthContext';

export interface LoginData {
    mail: string;
    password: string;
}

export interface RegisterData {
    nombre: string;
    email: string;
    dni: number;
    telefono: string;
    fecha_nacimiento: string;
    estado: string;
    ficha: string;
}

export interface AuthResponse {
    token: string;
    user: User;
}

export const authService = {
    async login(data: LoginData): Promise<AuthResponse> {
        const response = await api.post('/auth/login', data);
        return response.data;
    },

    async register(data: RegisterData): Promise<void> {
        await api.post('/auth/register', data);
    },

    logout(): void {
        localStorage.removeItem('token');
    },
};