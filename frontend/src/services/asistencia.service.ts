import api from './api';

export interface CreateAsistenciaData {
  fecha: string;
  metodo: string;
  id_reserva: string;
}

export interface AsistenciaResponse {
  id_asistencia: string;
  fecha: string;
  metodo: string;
  id_reserva: string;
}

export const asistenciaService = {
  async crearAsistencia(data: CreateAsistenciaData): Promise<AsistenciaResponse> {
    const response = await api.post('/asistencia/create', data);
    return response.data;
  },

  async getAsistencia(id: string): Promise<AsistenciaResponse> {
    const response = await api.get(`/asistencia/get-asistencia/${id}`);
    return response.data;
  },

  async getAsistenciaPorReserva(idReserva: string): Promise<AsistenciaResponse | null> {
    try {
      const response = await api.get(`/asistencia/get-asistencia-by-reserva/${idReserva}`);
      return response.data;
    } catch {
      return null;
    }
  },
};