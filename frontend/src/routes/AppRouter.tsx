// AppRouter.tsx
// Define todas las URLs y qué pantalla muestra cada una.
// Estructura: Routes (mapa de pantallas) > guards de rol > pantallas

import { Routes, Route, Navigate } from 'react-router-dom';
import { ProtectedRoute } from '@/components/auth/ProtectedRoute';

// Públicas
import { LandingPage } from '@/pages/public/LandingPage';
import { LoginPage } from '@/pages/public/LoginPage';
import { RegisterPage } from '@/pages/public/RegisterPage';
import { ForgotPasswordPage } from '@/pages/public/ForgotPasswordPage';

// Cliente
import { ClasesPage } from '@/pages/cliente/ClasesPage';
import { PerfilPage } from '@/pages/cliente/PerfilPage';
import { PagoExitoPage } from '@/pages/cliente/PagoExitoPage';
import { PagoFalloPage } from '@/pages/cliente/PagoFalloPage';
import { PagoPendientePage } from '@/pages/cliente/PagoPendientePage';

// Admin - Layout
import { AdminLayout } from '@/components/layout/AdminLayout';
// Admin - Clases
import AdminClasesPage from '@/pages/admin/clases/ClasesPage';
import { NuevaClasePage } from '@/pages/admin/clases/NuevaClasePage';
import { EditarClasePage } from '@/pages/admin/clases/EditarClasePage';

// Admin - Empleados
import { EmpleadosPage } from '@/pages/admin/empleados/EmpleadosPage';
import { NuevoEmpleadoPage } from '@/pages/admin/empleados/NuevoEmpleadoPage';
import { EditarEmpleadoPage } from '@/pages/admin/empleados/EditarEmpleadoPage';
import { NuevoProfesorPage } from '@/pages/admin/profesores/NuevoProfesorPage';

//Admin - Profesores
import { ProfesoresPage } from '@/pages/admin/profesores/ProfesoresPage';
// Admin - Clientes
import { ClientesPage } from '@/pages/admin/clientes/ClientesPage';
import { NuevoClientePage } from '@/pages/admin/clientes/NuevoClientePage';
import { EditarClientePage } from '@/pages/admin/clientes/EditarClientePage';

// Admin - Asistencias y Reportes
import { AsistenciasPage } from '@/pages/admin/asistencias/AsistenciasPage';
import { ReportesPage } from '@/pages/admin/reportes/ReportesPage';

export const AppRouter = () => {
    return (
        <Routes>
            {/* Rutas públicas: cualquiera puede entrar */}
            <Route path="/" element={<LandingPage />} />
            <Route path="/login" element={<LoginPage />} />
            <Route path="/register" element={<RegisterPage />} />
            <Route path="/recuperar-contrasena" element={<ForgotPasswordPage />} />

            {/* Rutas del cliente: requieren sesión activa */}
            <Route path="/clases" element={<ProtectedRoute><ClasesPage /></ProtectedRoute>} />
            <Route path="/perfil" element={<ProtectedRoute><PerfilPage /></ProtectedRoute>} />

            {/* Resultado de pago — públicas porque MP redirige sin token */}
            <Route path="/pago/exito" element={<PagoExitoPage />} />
            <Route path="/pago/fallo" element={<PagoFalloPage />} />
            <Route path="/pago/pendiente" element={<PagoPendientePage />} />

            {/* Rutas de administración: requieren rol admin o dueño */}
            <Route path="/admin" element={
            <ProtectedRoute>
               <AdminLayout />
            </ProtectedRoute>
                                          }>
                {/* Clases */}
                <Route path="clases" element={<AdminClasesPage />} />
                <Route path="clases/nueva" element={<NuevaClasePage />} />
                <Route path="clases/:id/editar" element={<EditarClasePage />} />

                {/* Empleados - solo dueño */}
                <Route path="empleados" element={<EmpleadosPage />} />
                <Route path="empleados/nuevo" element={<NuevoEmpleadoPage />} />
                <Route path="empleados/:id/editar" element={<EditarEmpleadoPage />} />
              
                {/* Profesores - solo dueño */}
                 <Route path="profesores" element={<ProfesoresPage />} /> 
                 <Route path="profesores/nuevo" element={<NuevoProfesorPage />} />         
                                
                {/* Clientes */}
                <Route path="clientes" element={<ClientesPage />} />
                <Route path="clientes/nuevo" element={<NuevoClientePage />} />
                <Route path="clientes/:id/editar" element={<EditarClientePage />} />

                {/* Asistencias */}
                <Route path="asistencias" element={<AsistenciasPage />} />

                {/* Reportes */}
                <Route path="reportes" element={<ReportesPage />} />
            </Route>

            {/* Cualquier URL desconocida redirige al inicio */}
            <Route path="*" element={<Navigate to="/" replace />} />
        </Routes>
    );
};