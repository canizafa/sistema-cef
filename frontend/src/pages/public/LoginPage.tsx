// Página de inicio de sesión.
// Formulario de inicio de sesión con email y contraseña. Guarda el JWT en el estado global al autenticar.
// Llama a authService.login(), guarda la sesión en el contexto global y redirige según el rol del usuario.
import { useState } from 'react';
import { Link, useNavigate } from 'react-router-dom';
import { Header } from '@/components/layout/Header';
import { useAuth, type Rol } from '@/context/AuthContext';
import { authService } from '@/services/auth.service';

export function LoginPage() {
  const [email, setEmail] = useState('');
  const [password, setPassword] = useState('');
  const [error, setError] = useState<string | null>(null);
  const [loading, setLoading] = useState(false);
  const { dispatch } = useAuth();
  const navigate = useNavigate();

  async function handleSubmit(e: React.FormEvent) {
    e.preventDefault();
    setError(null);
    setLoading(true);
    try {
      const data = await authService.login({ email, password });

      const user = {
        id: Number(data.dni),
        nombre: data.email,
        email: data.email,
        rol: data.rol as Rol,
        dni: Number(data.dni),
      };

      localStorage.setItem('token', data.access_token);
      localStorage.setItem('user', JSON.stringify(user));

      dispatch({ type: 'LOGIN', payload: { user, token: data.access_token } });

      if (data.rol === 'cliente') {
        navigate('/');
      } else if (data.rol === 'duenio') {
        navigate('/admin/clases');
      } else if (data.rol === 'empleado') {
        navigate('/admin/clientes');
      } else {
        navigate('/admin/clases');
      }
    } catch {
      setError('Email o contraseña incorrectos');
    } finally {
      setLoading(false);
    }
  }

  return (
    <div className='min-h-screen flex flex-col'>
      <Header />
      <main className='flex-1 flex items-center justify-center px-4 py-12'>
        <div className='w-full max-w-sm'>
          <h1 className='text-2xl font-bold mb-6 text-center'>Iniciar sesión</h1>
          <form onSubmit={handleSubmit} className='space-y-4'>
            <div className='space-y-1'>
              <label htmlFor='email' className='text-sm font-medium'>Email</label>
              <input
                id='email'
                type='email'
                placeholder='tu@email.com'
                value={email}
                onChange={(e) => setEmail(e.target.value)}
                required
                className='flex h-10 w-full rounded-md border border-input bg-background px-3 py-2 text-sm'
              />
            </div>
            <div className='space-y-1'>
              <label htmlFor='password' className='text-sm font-medium'>Contraseña</label>
              <input
                id='password'
                type='password'
                placeholder='••••••••'
                value={password}
                onChange={(e) => setPassword(e.target.value)}
                required
                className='flex h-10 w-full rounded-md border border-input bg-background px-3 py-2 text-sm'
              />
            </div>
            {error && <p className='text-sm text-red-600'>{error}</p>}
            <button
              type="submit"
              disabled={loading}
              className='w-full bg-brand text-white rounded-md h-10 text-sm font-medium hover:opacity-90 disabled:opacity-50'
            >
              {loading ? 'Ingresando...' : 'Iniciar Sesión'}
            </button>
          </form>
          <p className="text-sm text-center text-gray-500 mt-4">
            ¿No tenés cuenta?{' '}
            <Link to="/register" className="text-brand hover:underline">
              Registrarse
            </Link>
          </p>
          <p className="text-sm text-center text-gray-500 mt-2">
            ¿No recordás tu contraseña?{' '}
            <Link to="/recuperar-contrasena" className="text-brand hover:underline">
              Recuperar contraseña
            </Link>
          </p>
        </div>
      </main>
    </div>
  )
}