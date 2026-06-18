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

export interface UpdateEmpleado {
    dni: number;
    nombre_apellido: string;
    mail: string;
    genero: string;
    estado: string;
    rol: string;
}

export const empleadoService = {
    async registrarEmpleado(data: RegistrarEmpleado): Promise<void> {
        await api.post('/empleados/create', data);
    },

    async getEmpleado(dni: number) {
        const response = await api.get(`/empleados/get-empleado-by-dni/${dni}`);
        return response.data;
    },

    async getEmpleados() {
        const response = await api.get('/empleados/get-all');
        return response.data;
    },

    async actualizarEmpleado(dni: number, data: UpdateEmpleado) {
        const response = await api.put(`/empleados/update-empleado/${dni}`, data);
        return response.data;
    },

    async desactivarEmpleado(empleado: UpdateEmpleado) {
        return this.actualizarEmpleado(empleado.dni, { ...empleado, estado: 'baja' });
    },

    async activarEmpleado(empleado: UpdateEmpleado) {
        return this.actualizarEmpleado(empleado.dni, { ...empleado, estado: 'alta' });
    },
async eliminarEmpleado(dni: number, motivo: string): Promise<void> {
    await api.delete(`/empleados/delete-empleado/${dni}`, {
        data: {
            dni: dni,
            estado: 'baja',
            motivo_eliminacion: motivo,
        },
        headers: { 'Content-Type': 'application/json' }
    });
},
};