import api from './api';

export interface Profesor {
    dni: number;
    nombre_completo: string;
    estado: string;
    genero?: string;
    motivo_eliminacion?: string | null;
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

    // SOLUCIÓN REAL: Forzamos la persistencia en Rust usando su struct de cambio de estado
    // pero mandando el motivo vacío para que tu front lo deje en la lista de Inactivos.
    async desactivarProfesor(profesor: Profesor): Promise<void> {
        await api.delete(`/profesores/delete-profesor/${profesor.dni}`, {
            data: {
                profesor_dni: profesor.dni,
                estado: 'baja', 
                motivo_eliminacion: '', // Mandamos texto vacío para engañar al filtro del front
            }
        });
    },

    async activarProfesor(profesor: Profesor): Promise<void> {
        // Para volver a activarlo pasamos por el mismo canal indicando estado 'alta'
        await api.delete(`/profesores/delete-profesor/${profesor.dni}`, {
            data: {
                profesor_dni: profesor.dni,
                estado: 'alta', 
                motivo_eliminacion: '', 
            }
        });
    },

    async eliminarProfesor(dni: number, motivo: string): Promise<void> {
        await api.delete(`/profesores/delete-profesor/${dni}`, {
            data: {
                profesor_dni: dni,
                estado: 'baja',
                motivo_eliminacion: motivo, // Acá sí mandamos el motivo real para que se oculte
            }
        });
    },

    async tieneClasesAsociadas(dniProfesor: number): Promise<boolean> {
        const response = await api.get('/clase/get-all');
        const clases = response.data;
        return clases.some((clase: any) => clase.dni_profesor === dniProfesor);
    },
};