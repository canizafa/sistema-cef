// App.tsx
// Punto de entrada visual de la app.
import { BrowserRouter } from 'react-router-dom';
import { AuthProvider } from '@/context/AuthContext';
import { CreditosProvider } from '@/context/CreditosContext';
import { AppRouter } from '@/routes/AppRouter';
import { Toaster } from '@/components/ui/sonner';

export default function App() {
    return (
        <BrowserRouter>
            <AuthProvider>
                <CreditosProvider>
                    <AppRouter />
                    <Toaster />
                </CreditosProvider>
            </AuthProvider>
        </BrowserRouter>
    );
}