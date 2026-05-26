// Memoria compartida de la aplicación para saber quién está logueado (Context API + useReducer).
// Guarda el usuario y su token de sesión (JWT). Cualquier página puede leerlo sin que se lo pasen explícitamente (via props).
// Existe porque múltiples pantallas necesitan saber quién es el usuario y qué permisos tiene.
import { createContext, useContext, useReducer, type ReactNode } from 'react';

// --- Tipos ---

export type Rol = 'dueno' | 'recepcionista' | 'cliente';

export type User = {
    id: number;
    nombre: string;
    email: string;
    rol: Rol;
    dni: number;  
};

type AuthState = {
    user: User | null;
    token: string | null;
};

type AuthAction =
    | { type: 'LOGIN'; payload: { user: User; token: string} }
    | { type: 'LOGOUT' }
    | { type: 'UPDATE_USER'; payload: { user: User } };

// --- Reducer ---

function loadInitialState(): AuthState {
    const token = localStorage.getItem('token');
    const userJson = localStorage.getItem('user');
    if (token && userJson) {
        try {
            return { token, user: JSON.parse(userJson) };
        } catch {
            // JSON corrupto — sesión inválida
        }
    }
    return { user: null, token: null };
}

const initialState: AuthState = loadInitialState();

export function authReducer(state: AuthState, action: AuthAction): AuthState {
    switch (action.type){
        case 'LOGIN':
            return { user: action.payload.user, token: action.payload.token };
        case 'LOGOUT':
            return { user: null, token: null };
        case 'UPDATE_USER':
            if(!state.user) return state;
            return { ...state, user: action.payload.user };
        default:
            return state;
    }
}

// --- Contexto ---

type AuthContextValue = AuthState & {
    dispatch: React.Dispatch<AuthAction>;
};

const AuthContext = createContext<AuthContextValue | null>(null);

export function AuthProvider({ children }: {children: ReactNode }){
    const [state, dispatch] = useReducer(authReducer, initialState);
    return (
        <AuthContext.Provider value={{ ...state, dispatch}}>
            {children}
        </AuthContext.Provider>
    )
}

export function useAuth(): AuthContextValue {
    const ctx = useContext(AuthContext);
    if (!ctx) throw new Error('useAuth debe usarse dentro de AuthProvider');
    return ctx; 
}