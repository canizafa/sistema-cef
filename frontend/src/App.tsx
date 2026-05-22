// Punto de entrada visual de la app.
// Define todas las URLs y qué página muestra cada una.
// Estructura: BrowserRouter (navegación) > AuthProvider (sesión) > Routes (mapa de páginas)
import { BrowserRouter, Routes, Route, Navigate } from 'react-router-dom';
import { AuthProvider } from '@/context/AuthContext'; // Estado global de sesión (quién está logueado)
import { ProtectedRoute } from '@/components/auth/ProtectedRoute'; // Guarda: solo usuarios logueados
import { AdminRoute } from '@/components/auth/AdminRoute'; // Guarda: solo el dueño
import { LandingPage } from '@/pages/public/LandingPage';
import { LoginPage } from '@/pages/public/LoginPage';
import { RegisterPage } from '@/pages/public/RegisterPage';
import { ClasesPage } from '@/pages/ClasesPage';
import { PerfilPage } from '@/pages/PerfilPage';
import { AdminPage } from '@/pages/admin/AdminPage';
import { Toaster } from '@/components/ui/sonner'; // Notificaciones emergentes globales

export default function App() {
    return (
        // BrowserRouter: habilita navegación con URLs reales sin recargar la página
        <BrowserRouter>
            {/* AuthProvider: cualquier componente de la app puede saber quién está logueado */} 
            <AuthProvider>
                <Routes>
                    {/* Rutas públicas: cualquiera puede entrar */}
                    <Route path="/" element={<LandingPage />} />
                    <Route path="/login" element={<LoginPage />} />
                    <Route path="/register" element={<RegisterPage />} />

                    {/* Rutas protegidas: solo usuarios con sesión activa */}
                    <Route path="/clases" element={<ProtectedRoute><ClasesPage /></ProtectedRoute>} />
                    <Route path="/perfil" element={<ProtectedRoute><PerfilPage /></ProtectedRoute>} />

                    {/* Ruta de administración: solo el dueño */}
                    <Route path="/admin" element={<AdminRoute><AdminPage /></AdminRoute>} />

                    {/* Cualquier URL desconocida redirige al inicio */}
                    <Route path="*" element={<Navigate to="/" replace />} />
                </Routes>
                
                {/* Toaster: contenedor global para los mensajes de éxito/error */}
                <Toaster />
            </AuthProvider>
        </BrowserRouter>
    );
}