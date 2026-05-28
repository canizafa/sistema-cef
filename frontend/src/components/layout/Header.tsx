// Barra de navegación superior compartida por todas las páginas.
// Muestra links distintos según el estado de sesión: si hay token muestra Clases, Perfil y Cerrar sesión;
// si el rol es "duenio" o "empleado" también muestra el link al panel admin. Sin sesión muestra Iniciar sesión y Registrarse.
import { Link, useNavigate } from 'react-router-dom';
import { useAuth } from '@/context/AuthContext';
import logo from '@/assets/Logo.png';

export function Header() {
    const { token, user, dispatch } = useAuth();
    const navigate = useNavigate();

    function handleLogout() {
        localStorage.removeItem('token');
        localStorage.removeItem('user');
        dispatch({ type: 'LOGOUT' });
        navigate('/');
    }

    return (
      <header className="sticky top-0 z-50 w-full bg-white border-b border-gray-200 shadow-sm">
            <div className="max-w-375 mx-auto px-8 h-24 flex items-center justify-between">
                <Link to="/" className="shrink-0">
                    <img src={logo} alt="Logo CEF" className="h-16 w-auto object-contain" />
                </Link>

                <nav className="flex items-center gap-4">
                    {token ? (
                        <>
                            <Link to="/clases" className="text-sm font-medium hover:text-brand">
                                Clases
                            </Link>
                            <Link to="/perfil" className="text-sm font-medium hover:text-brand">
                                Mi perfil
                            </Link>
                            {(user?.rol === 'duenio' || user?.rol === 'empleado') && (
                                <Link to="/admin" className="text-sm font-medium hover:text-brand">
                                    Panel Admin
                                </Link>
                            )}
                            <button
                                onClick={handleLogout}
                                className="text-sm border border-gray-300 rounded px-3 py-1 hover:bg-gray-100"
                            >
                                Cerrar sesión
                            </button>
                        </>
                    ) : (
                        <>
                            <Link to="/login" className="text-sm border border-gray-300 rounded px-3 py-1 hover:bg-gray-100">
                                Iniciar sesión
                            </Link>
                            <Link to="/register" className="text-sm bg-brand text-white rounded px-3 py-1 hover:opacity-90">
                                Registrarse
                            </Link>
                        </>
                    )}
                </nav>
            </div>
        </header>
    );
}