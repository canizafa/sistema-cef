// Barra de navegación superior compartida por todas las páginas.
// Muestra links distintos según el estado de sesión: si hay token muestra Clases, Perfil y Cerrar sesión;
// si el rol es "dueno" también muestra el link a Empleados. Sin sesión muestra Iniciar sesión y Registrarse.
import { Link, useNavigate } from 'react-router-dom';
import { useAuth } from '@/context/AuthContext';
import logo from '@/assets/Logo.png';

export function Header() {
    const { token, user, dispatch } = useAuth();
    const navigate = useNavigate();

    function handleLogout() {
        dispatch({ type: 'LOGOUT' }) //Limpia el usuario y el token del estado global
        navigate('/'); //Manda al usuario a la pagina de inicio
    }

    return (
      <header className="sticky top-0 z-50 w-full bg-white border-b border-gray-200 shadow-sm">
            {/* Aumenté el padding horizontal a px-8 para que no se pegue a los bordes */}
            <div className="max-w-375 mx-auto px-8 h-24 flex items-center justify-between">

                {/* Logo más grande: h-16 (64px) */}
                <Link to="/" className="shrink-0">
                    <img src={logo} alt="Logo CEF" className="h-16 w-auto object-contain" />
                </Link>

                <nav className="flex items-center gap-4">
                    {token ? (
                        // Usuario logueado: muestra links de navegacion y boton logout
                        <>
                            <Link to="/clases" className="text-sm font-medium hover:text-brand">
                                Clases
                            </Link>
                            <Link to="/perfil" className="text-sm font-medium hover:text-brand">
                                Mi perfil
                            </Link>
                            {user?.rol === 'dueno' && (
                                //Solo visible para el dueno
                                <Link to="/admin" className="text-sm font-medium hover:text-brand">
                                    Empleados
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
                        // Sin sesion: muestra botones de login y registro
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