// Layout del panel de administración.
// Muestra un sidebar con navegación filtrada según el rol del usuario logueado.
// DUENIO ve todo. EMPLEADO ve todo menos Empleados. CLIENTE es redirigido al inicio.
import { NavLink, Outlet, Navigate, useNavigate } from 'react-router-dom'
import { useAuth } from '@/context/AuthContext'
import logo from '@/assets/Logo.png';

const allNavItems = [
  { to: '/admin/clientes',    label: 'Clientes',    roles: ['duenio', 'empleado'] },
  { to: '/admin/clases',      label: 'Clases',      roles: ['duenio', 'empleado'] },
  { to: '/admin/profesores',  label: 'Profesores',  roles: ['duenio', ] },
  { to: '/admin/empleados',   label: 'Empleados',   roles: ['duenio'] },
  { to: '/admin/asistencias', label: 'Asistencias', roles: ['duenio', 'empleado'] },
  { to: '/admin/reportes',    label: 'Reportes',    roles: ['duenio'] },
]

export function AdminLayout() {
  const { user, dispatch } = useAuth()
  const navigate = useNavigate()
  const rol = user?.rol

  if (!rol || rol === 'cliente' || rol === 'profesor') return <Navigate to="/" />

  const navItems = allNavItems.filter(item => item.roles.includes(rol))
  const inicial = user?.nombre?.[0]?.toUpperCase() ?? 'A'
  const rolLabel = rol === 'duenio' ? 'Dueño' : 'Empleado'

  function handleLogout() {
    localStorage.removeItem('token')
    localStorage.removeItem('user')
    dispatch({ type: 'LOGOUT' })
    navigate('/')
  }

  return (
    <div className="flex min-h-screen bg-background">

      {/* ── Sidebar ── */}
      <aside className="w-52 shrink-0 flex flex-col bg-[#1A1A1A] border-r border-[#2A2A2A]">

        {/* Logo */}
        <div className="flex flex-col items-center gap-1 py-6 border-b border-[#2A2A2A]">
          <img src={logo} alt="Logo CEF" className="h-16 w-auto object-contain" />
          <span className="text-xs text-[#6B7280] font-medium mt-1">Panel de Administración</span>
        </div>

        {/* Nav */}
        <nav className="flex-1 py-4 px-3 space-y-1">
          {navItems.map(({ to, label }) => (
            <NavLink
              key={to}
              to={to}
              className={({ isActive }) =>
                [
                  'block px-3 py-2.5 rounded-lg text-sm font-medium transition-colors',
                  isActive
                    ? 'bg-[#D01F25] text-white'
                    : 'text-[#9CA3AF] hover:bg-[#252525] hover:text-white',
                ].join(' ')
              }
            >
              {label}
            </NavLink>
          ))}
        </nav>

        {/* Usuario + Cerrar sesión */}
        <div className="px-4 py-4 border-t border-[#2A2A2A] space-y-3">
          <div className="flex items-center gap-3">
            <div className="w-8 h-8 rounded-full bg-[#163F7A] flex items-center justify-center text-white text-xs font-bold shrink-0">
              {inicial}
            </div>
            <div className="min-w-0">
              <p className="text-sm font-medium text-white truncate">{user?.nombre}</p>
              <p className="text-xs text-[#6B7280] truncate">{rolLabel}</p>
            </div>
          </div>
          <button
            onClick={handleLogout}
            className="w-full text-sm text-[#f0f4fa] hover:text-white border border-[#9a9999] hover:bg-[#dc1414] rounded-lg px-3 py-2 transition-colors text-left"
          >
            Cerrar sesión
          </button>
        </div>
      </aside>

      {/* ── Contenido ── */}
      <main className="flex-1 overflow-auto bg-background">
        <Outlet />
      </main>

    </div>
  )
}