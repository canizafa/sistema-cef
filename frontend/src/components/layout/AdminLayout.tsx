import { NavLink, Outlet } from 'react-router-dom'
import logo from '@/assets/Logo.png';
const navItems = [
  { to: '/admin/clientes',    label: 'Clientes' },
  { to: '/admin/clases',      label: 'Clases' },
  { to: '/admin/empleados',   label: 'Empleados' },
  { to: '/admin/asistencias', label: 'Asistencias' },
  { to: '/admin/reportes',    label: 'Reportes' },
]

export function AdminLayout() {
  return (
    <div className="flex min-h-screen bg-[#121212]">

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

        {/* Usuario */}
        <div className="flex items-center gap-3 px-4 py-4 border-t border-[#2A2A2A]">
          <div className="w-8 h-8 rounded-full bg-[#163F7A] flex items-center justify-center text-white text-xs font-bold shrink-0">
            A
          </div>
          <div className="min-w-0">
            <p className="text-sm font-medium text-white truncate">Admin</p>
            <p className="text-xs text-[#6B7280] truncate">Administrador</p>
          </div>
        </div>
      </aside>

      {/* ── Contenido ── */}
      <main className="flex-1 overflow-auto bg-[#121212]">
        <Outlet />
      </main>

    </div>
  )
}