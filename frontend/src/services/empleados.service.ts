// Servicio de empleados.
// Expone registrarEmpleado() que llama al endpoint POST /empleados.
// Existe porque el alta de empleados es exclusiva del dueño y requiere su propio contrato de datos.
import api from './api';

export interface RegistrarEmpleado {
    nombre: string;
    apellido: string;
    dni: string;
    mail: string;
    password: string;
    rol: 'recepcionista';
}

export const empleadoService = {
    async registrarEmpleado(data: RegistrarEmpleado): Promise<void> {
        await api.post('/empleados', data);
    },
}