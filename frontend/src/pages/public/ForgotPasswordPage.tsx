import { useState } from 'react';
import { Link } from 'react-router-dom';
import { Header } from '@/components/layout/Header';
import { authService } from '@/services/auth.service';

export function ForgotPasswordPage() {
  const [email, setEmail] = useState('');
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [success, setSuccess] = useState(false);

  async function handleSubmit(e: React.FormEvent) {
    e.preventDefault();
    setError(null);
    setLoading(true);
    try {
      await authService.forgotPassword({ email });
      setSuccess(true);
    } catch {
      setError('Email no registrado/incorrecto');
    } finally {
      setLoading(false);
    }
  }

  if (success) {
    return (
      <div className='min-h-screen flex flex-col'>
        <Header />
        <main className='flex-1 flex items-center justify-center px-4 py-12'>
          <div className='w-full max-w-sm text-center space-y-4'>
            <h1 className='text-2xl font-bold'>Revisá tu correo</h1>
            <p className='text-sm text-gray-500'>
              Te enviamos una nueva contraseña al email registrado.
            </p>
            <Link to='/login' className='text-brand hover:underline text-sm block'>
              Volver al inicio de sesión
            </Link>
          </div>
        </main>
      </div>
    );
  }

  return (
    <div className='min-h-screen flex flex-col'>
      <Header />
      <main className='flex-1 flex items-center justify-center px-4 py-12'>
        <div className='w-full max-w-sm'>
          <h1 className='text-2xl font-bold mb-6 text-center'>Recuperar contraseña</h1>
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
            {error && <p className='text-sm text-red-600'>{error}</p>}
            <button
              type='submit'
              disabled={loading}
              className='w-full bg-brand text-white rounded-md h-10 text-sm font-medium hover:opacity-90 disabled:opacity-50'
            >
              {loading ? 'Enviando...' : 'Recuperar contraseña'}
            </button>
          </form>
          <p className='text-sm text-center text-gray-500 mt-4'>
            <Link to='/login' className='text-brand hover:underline'>
              Volver al inicio de sesión
            </Link>
          </p>
        </div>
      </main>
    </div>
  );
}
