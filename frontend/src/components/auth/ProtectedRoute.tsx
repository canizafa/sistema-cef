// Componente de ruta protegida.
// Envuelve páginas que requieren estar logueado. Si no hay token, redirige a /login.
// Existe para no repetir la lógica de autenticación en cada página privada.
import { Navigate } from 'react-router-dom';
import { useAuth } from '@/context/AuthContext';
import type { ReactNode } from 'react';

export function ProtectedRoute({ children }: { children: ReactNode }){
    const { token } = useAuth();
    if( !token ) return <Navigate to="/login" replace />;
    return <>{children}</>
}