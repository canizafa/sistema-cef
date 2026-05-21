// Servicio de reservas.
// Expone crearReserva() y getMisReservas() para interactuar con el backend.
// Existe para separar la lógica de reservas de la UI y centralizar el manejo de errores (membresía vencida, sin cupo).
import api from './api';

export const reservasService = {
    async crearReserva(claseId: number) {
        const response = await api.post('/reservas', {claseId});
        return response.data;
    },

    async getMisReservas() {
        const response = await api.get('/reservas/mis-reservas');
        return response.data;
    },
};