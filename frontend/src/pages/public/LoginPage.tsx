// Página de inicio de sesión.
// Página de inicio de sesión.
// Formulario de inicio de sesión con email y contraseña. Guarda el JWT en el estado global al autenticar.
// Llama a authService.login(), guarda la sesión en el contexto global y redirige según el rol del usuario.
import { useState } from 'react';
import { Link, useNavigate } from 'react-router-dom';
import { Header } from '@/components/layout/Header';
import { useAuth } from '@/context/AuthContext';
import { authService } from '@/services/auth.service';

export function LoginPage() {
  const [mail, setMail] = useState('');
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
      const data = await authService.login({ mail, password });
      dispatch({ type: 'LOGIN', payload: { user: data.user, token: data.token } });

      // Redirigir según rol
      if (data.user.rol === 'cliente') {
        navigate('/');                  // Cliente ve la landing
      } else {
        navigate('/admin/clientes');    // Dueño y recepcionista van al panel
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
              <label htmlFor='mail' className='text-sm font-medium'>Email</label>
              <input
                id='mail'
                type='email'
                placeholder='tu@email.com'
                value={mail}
                onChange={(e) => setMail(e.target.value)}
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
              {loading ? 'Ingresando...' : 'Ingresar'}
            </button>
          </form>
          <p className="text-sm text-center text-gray-500 mt-4">
            ¿No tenés cuenta?{' '}
            <Link to="/register" className="text-brand hover:underline">
              Registrarse
            </Link>
          </p>
        </div>
      </main>
    </div>
  )
}