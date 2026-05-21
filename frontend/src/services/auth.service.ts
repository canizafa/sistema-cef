// Servicio de autenticación.
// Expone las funciones login(), register() y changePassword() que llaman al backend.
// Existe para separar la lógica HTTP de los componentes de UI que manejan los formularios.
import api from './api';
import type { User } from '../context/AuthContext';

export interface LoginData {
    mail: string;
    password: string;
}

// Campos que acepta CrearAlumno en el backend
export interface RegisterData {
    nombre: string;
    apellido: string;
    dni: string;
    mail: string;
    password: string;
    fecha_nacimiento: string; // formato "YYYY-MM-DD"
    telefono: string;
    estado: string;
    ficha_medica: {
        enfermedades: boolean;
        operaciones_quirurgicas: boolean;
        detalles: string;
    };
}

export interface AuthResponse {
    token: string;
    user: User;
}

export const authService = {
    async login(data: LoginData): Promise<AuthResponse> {
        const response = await api.post<AuthResponse>('/auth/login', data);
        return response.data;
    },

    async register(data: RegisterData): Promise<void> {
        await api.post('/auth/register', data);
    },

    logout(): void {
        localStorage.removeItem('token');
    },
};