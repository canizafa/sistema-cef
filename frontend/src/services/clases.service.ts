// Servicio de clases.
// Expone getClases() que obtiene el listado de clases del backend y el tipo Clase.
// Existe para centralizar el contrato de datos con el endpoint GET /clases.
import api from './api';

export const clasesService = {
    async getClases() {
        const response = await api.get('/clases');
        return response.data;
    },
};