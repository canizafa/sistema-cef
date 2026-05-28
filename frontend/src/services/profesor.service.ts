import api from './api';

export interface Profesor {
    dni: number;
    nombre_completo: string;
    estado: string;
}

export interface CreateProfesor {
    dni: number;
    nombre_completo: string;
    genero: string;
    estado: string;
}

export const profesorService = {
    async getProfesores(): Promise<Profesor[]> {
        const response = await api.get('/profesores/get-all');
        return response.data;
    },

    async getProfesor(dni: number): Promise<Profesor> {
        const response = await api.get(`/profesores/get-profesor/${dni}`);
        return response.data;
    },

    async crearProfesor(data: CreateProfesor): Promise<void> {
        await api.post('/profesores/create-profesor', data);
    },

    async actualizarProfesor(dni: number, data: Omit<CreateProfesor, 'dni'>): Promise<void> {
        await api.put(`/profesores/update-profesor/${dni}`, { dni, ...data });
    },

    async desactivarProfesor(profesor: Profesor): Promise<void> {
        await api.put(`/profesores/update-profesor/${profesor.dni}`, {
            dni: profesor.dni,
            nombre_completo: profesor.nombre_completo,
            genero: 'otro',
            estado: 'baja',
        });
    },

    async activarProfesor(profesor: Profesor): Promise<void> {
        await api.put(`/profesores/update-profesor/${profesor.dni}`, {
            dni: profesor.dni,
            nombre_completo: profesor.nombre_completo,
            genero: 'otro',
            estado: 'alta',
        });
    },

    async eliminarProfesor(dni: number): Promise<void> {
        await api.delete(`/profesores/delete-profesor/${dni}`);
    },
};