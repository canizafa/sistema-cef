// Configuración central para hablar con el servidor (instancia de Axios).
// Agrega automáticamente el token de sesión (JWT) en cada petición (request) que se hace al backend.
// Existe para no repetir esta configuración en cada archivo que necesite comunicarse con el servidor.

import axios from 'axios';

const api = axios.create({
    baseURL: import.meta.env.VITE_API_URL ?? 'http://localhost:8081',
});

api.interceptors.request.use((config) => {
    const token = localStorage.getItem('token');
    if (token) {
        config.headers.Authorization = `Bearer ${token}`;
    }
    return config;
});

export default api;
