import api from './api';
//import type { User } from '../context/AuthContext';

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
    email:string;
    token: string;
    rol: String;
}

// ===================== MOCK =====================
const MOCK_LOGIN = true; // cambiar a false cuando el back esté listo

const mockAuthResponse: AuthResponse = {
    dni: '12345678',
    email: 'juan@email.com',
    token: 'mock-token-123',
    rol: 'cliente',
};
// ================================================

export const authService = {
    async login(data: LoginData): Promise<AuthResponse> {
        if (MOCK_LOGIN) {
            if (data.email === 'juan@email.com' && data.password === '123456') {
                return mockAuthResponse;
            }
            throw new Error('Credenciales incorrectas');
        }
        // const response = await api.post('/auth/login', data);
        // return response.data;
        const response = await api.post('/auth/login', data);
        return response.data;
    },

    async register(data: RegisterData): Promise<void> {
        await api.post('/auth/register-cliente', data);
    },

    async forgotPassword(data: { email: string }): Promise<void> {
        await api.post('/auth/forgot-password', data);
    },

    logout(): void {
        localStorage.removeItem('token');
    },
};