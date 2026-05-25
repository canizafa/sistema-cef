// Componente de ruta exclusiva para el dueño.
// Si no hay token redirige a /login; si el rol no es "dueno" redirige a /clases.
// Existe para proteger /admin sin duplicar la lógica de control de acceso en la página.
import { Navigate } from 'react-router-dom';
import { useAuth } from '@/context/AuthContext';
import type { ReactNode } from 'react';

export function AdminRoute({ children }: { children: ReactNode }){
    const { token, user } = useAuth(); // token = hay sesión?, user = datos del usuario logueado
    if( !token ) return <Navigate to="/login" replace />; // No hay sesión → al login
    if( user?.rol !== 'dueno' ) return <Navigate to="/clases" replace />; // Hay sesión pero no es dueño → a clases
    return <>{children}</> // Es dueño → mostrá la página de admin
}