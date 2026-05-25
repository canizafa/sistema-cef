import api from './api';

export interface RegistrarEmpleado {
    dni: number;
    nombre_apellido: string;
    password: string;
    mail: string;
    genero: string;
    estado: string;
    rol: string;
}


export const empleadoService = {
    async registrarEmpleado(data: RegistrarEmpleado): Promise<void> {
        await api.post('/auth/register-empleado', data);
    },
    
    async getEmpleado(params: unknown) {
        const response = await api.get('/empleados/get-empleado', { params });
        return response.data;
    },

    async getEmpleados() {
        const response = await api.get('/empleados/get-all');
        return response.data;
    },

    async actualizarEmpleado(data: unknown) {
        const response = await api.post('/empleados/update-empleado', data);
        return response.data;
    },

    async eliminarEmpleado(data: unknown) {
        const response = await api.post('/empleados/delete-empleado', data);
        return response.data;
    },
};
