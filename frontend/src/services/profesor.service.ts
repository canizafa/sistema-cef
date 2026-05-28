import api from './api';

export interface Profesor {
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
};