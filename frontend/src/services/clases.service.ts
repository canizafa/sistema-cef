import api from './api';

export type EstadoClase = 'activa' | 'inactiva';

// Lo que devuelve el back
export type ClaseDTO = {
    id_clase: string;      // 
    dia: string;           // NaiveDate se serializa como string "YYYY-MM-DD"
    horario: string;
    estado: EstadoClase;
    lleno: boolean;        // faltaba
    descripcion: string;   // faltaba
    id_actividad: string;  // faltaba
};

// Lo que manda el front al crear
export type NuevaClaseData = {
    dia: string;           // "YYYY-MM-DD"
    horario: string;
    cupo_profe: number;
    cupo_maximo: number;
    estado: EstadoClase;
    id_actividad: string;  // era number
    id_sala: string;       // era number
    dni_profesor: string;  // era number
    descripcion: string;   // faltaba
};

export const clasesService = {
   

    async crearClase(data: NuevaClaseData) {
        const response = await api.post('/clase/create', data);
        return response.data;
    },

    
     async getClases(): Promise<ClaseDTO[]> {
        const response = await api.get('/clase/get-all');
        return response.data;
    },

    async getClase(params: unknown) {
        const response = await api.get('/clase/get-clase', { params });
        return response.data;
    },
    async actualizarClase(data: unknown) {
        const response = await api.post('/clase/update-clase', data);
        return response.data;
    },

    async eliminarClase(data: unknown) {
        const response = await api.post('/clase/delete-clase', data);
        return response.data;
    }, 
};