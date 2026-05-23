// Servicio de clases.
// Expone getClases() y crearClase() para interactuar con el backend.
// Centraliza el contrato de datos con los endpoints de /clases.
import api from './api';

export type NuevaClaseData = {
    nombre: string;
    tipo: string;
    profesor: string;
    dias: string;
    horario: string;
    duracionMin: number;
    cupoMaximo: number;
};

export const clasesService = {
    async getClases() {
        const response = await api.get('/clases');
        return response.data;
    },

    async crearClase(data: NuevaClaseData) {
        const response = await api.post('/clases', data);  // POST con los datos del form
        return response.data;
    },
};