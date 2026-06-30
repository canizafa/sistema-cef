import { createContext, useCallback, useContext, useEffect, useState, type ReactNode } from 'react';
import { clienteService } from '@/services/cliente.service';
import { useAuth } from './AuthContext';

type CreditosContextValue = {
    creditos: number;
    refrescarCreditos: () => Promise<void>;
};

const CreditosContext = createContext<CreditosContextValue | null>(null);

export function CreditosProvider({ children }: { children: ReactNode }) {
    const { user } = useAuth();
    const [creditos, setCreditos] = useState(0);

    const refrescarCreditos = useCallback(async () => {
        if (!user || user.rol !== 'cliente') {
            setCreditos(0);
            return;
        }
        try {
            const perfil = await clienteService.getPerfil(user.dni);
            setCreditos(perfil.cliente.creditos ?? 0);
        } catch {
            // fallo silencioso — el saldo se queda en el último valor conocido
        }
    }, [user]);

    useEffect(() => {
        refrescarCreditos();
    }, [refrescarCreditos]);

    return (
        <CreditosContext.Provider value={{ creditos, refrescarCreditos }}>
            {children}
        </CreditosContext.Provider>
    );
}

export function useCreditos(): CreditosContextValue {
    const ctx = useContext(CreditosContext);
    if (!ctx) throw new Error('useCreditos debe usarse dentro de CreditosProvider');
    return ctx;
}
