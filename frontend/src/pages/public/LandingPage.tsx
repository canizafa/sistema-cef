import { Link } from 'react-router-dom';
import { Header } from '@/components/layout/Header';
import { ArrowRight, ChevronDown } from 'lucide-react';
import { Footer } from '@/components/layout/Footer';
import { useAuth } from '@/context/AuthContext';
import { Navigate } from 'react-router-dom';

// Array con las disciplinas para renderizarlas dinámicamente
  const clases = ['FUNCIONAL', 'PILATES', 'CROSSFIT', 'YOGA'];

export function LandingPage() {
  const { user } = useAuth();

      // Si es admin o empleado, redirigir directo al panel
  if (user?.rol === 'duenio' || user?.rol === 'empleado') {
    return <Navigate to="/pages/admin/clases/ClasesPage" replace />;
  }

  return (
    <div className="min-h-screen bg-white text-gray-800">
      <Header />

      <section
        id="inicio"
        className="relative min-h-screen flex flex-col justify-center overflow-hidden bg-black"
      >
        <div
          className="absolute inset-0 bg-cover bg-center opacity-50 blur-md scale-105"
          style={{ backgroundImage: "url('https://images.unsplash.com/photo-1534438327276-14e5300c3a48?w=1400')" }}
        />
        <div className="absolute inset-0 bg-black/60" />

        <div className="relative z-10 max-w-7xl mx-auto w-full px-8 py-20 md:py-32 text-white">
          <h1 className="text-4xl sm:text-5xl md:text-6xl font-bold leading-tight mb-5">
            <span className="block">Construir Fuerza.</span>
            <span className="block">Construir Confianza.</span>
          </h1>

          <p className="text-base md:text-lg text-white/90 max-w-lg mb-8 leading-relaxed">
            Formá parte del mejor centro de actividades físicas. Clases profesionales,
            entrenadores certificados y una comunidad que te impulsa a alcanzar tus metas.
          </p>

          <div className="flex flex-col sm:flex-row gap-4">
            {!user && (
              <Link
                to="/register"
                className="inline-flex items-center justify-center gap-2 bg-red-600 hover:bg-red-700 text-white font-semibold px-6 py-3 rounded-md transition-colors text-sm"
              >
                Comenzar ahora <ArrowRight className="w-4 h-4" />
              </Link>
            )}
            <Link
              to="/clases"
              className="inline-flex items-center justify-center border-2 border-white/80 text-white hover:bg-white hover:text-black font-semibold px-6 py-3 rounded-md transition-colors text-sm"
            >
              Ver clases
            </Link>
          </div>
        </div>

        <div className="relative z-10 border-t border-white/10 bg-black/40 backdrop-blur-sm">
          <div className="max-w-7xl mx-auto px-8 py-6 grid grid-cols-2 md:grid-cols-4 gap-4">
            <div className="text-center">
              <p className="text-3xl font-bold text-white">+500</p>
              <p className="text-sm text-gray-400 mt-1">Socios activos</p>
            </div>
            <div className="text-center">
              <p className="text-3xl font-bold text-white">12</p>
              <p className="text-sm text-gray-400 mt-1">Profesores certificados</p>
            </div>
            <div className="text-center">
              <p className="text-3xl font-bold text-white">3</p>
              <p className="text-sm text-gray-400 mt-1">Salas equipadas</p>
            </div>
            <div className="text-center">
              <p className="text-3xl font-bold text-white">5★</p>
              <p className="text-sm text-gray-400 mt-1">Valoración promedio</p>
            </div>
          </div>
        </div>

        <a
          href="#clases"
          className="absolute bottom-10 left-1/2 -translate-x-1/2 z-10 text-white/30 hover:text-white transition-colors animate-bounce hidden md:block"
        >
          <ChevronDown className="w-8 h-8" />
        </a>
      </section>

   {/* SECCIÓN NUESTRAS CLASES MODIFICADA */}
      <section id="clases" className="py-20 md:py-28 bg-gray-50">
        <div className="max-w-7xl mx-auto px-8">
          <div className="text-center mb-14">
            <h2 className="text-3xl md:text-5xl font-bold text-gray-800">
              Nuestras <span className="text-red-600">Clases</span>
            </h2>
            <p className="mt-4 text-gray-500 max-w-2xl mx-auto">
              Explora nuestra variedad de disciplinas diseñadas para todos los niveles.
            </p>
          </div>

          {/* Contenedor Grid Responsivo */}
          <div className="grid grid-cols-1 md:grid-cols-4 gap-6">
            {clases.map((clase, index) => (
              <div
                key={index}
                className="aspect-square flex items-center justify-center bg-gray-600 text-white font-bold text-xl tracking-wider rounded-lg shadow-md hover:bg-red-600 transition-all duration-300 transform hover:-translate-y-1 cursor-pointer"
              >
                {clase}
              </div>
            ))}
          </div>
        </div>
      </section>
      <Footer />
    </div>
  );
}