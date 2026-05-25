// Tests unitarios del reducer de AuthContext.
// Verifica que las acciones LOGIN, LOGOUT y UPDATE_USER modifican el estado correctamente.
// Existe para garantizar que la lógica de autenticación funciona antes de integrarla con la UI.
import { describe, it, expect } from 'vitest';
import { authReducer } from './AuthContext';

const initialState = { user: null, token: null };

describe('authReducer', () => {
    it('LOGIN guarda el usuario y el token en el estado', () => {
        const user = { id: 1, nombre: 'Juan', email: 'juan@cef.com', rol: 'cliente' as const }; // 'as const' le dice a TypeScript que 'cliente' es un valor exacto, no un string genérico.
        const newState = authReducer(initialState,{
            type: 'LOGIN',
            payload: { user, token: 'abc123' },
        });
        expect(newState.user).toEqual(user);
        expect(newState.token).toBe('abc123');
    });

    it('LOGOUT pone user y token en null', () => {
        const startState = {
            user: { id: 1, nombre: 'Juan', email: 'juan@cef.com', rol: 'cliente' as const },
            token: 'abc123',
        };
        const newState = authReducer(startState, { type: 'LOGOUT' });
        expect(newState.user).toBeNull();
        expect(newState.token).toBeNull();
    });

    it('UPDATE_USER actualiza el usuario si hay uno logueado', () =>{
        const startState = {
            user: {id: 1, nombre: 'Juan', email: 'juan@cef.com', rol: 'cliente' as const },
            token: 'abc123',
        };
        const updated = { id: 1, nombre: 'Juan Editado', email: 'juan@cef.com', rol: 'cliente' as const };
        const newState = authReducer(startState, { type: 'UPDATE_USER', payload: { user: updated } });
        expect(newState.user).toEqual(updated);
    });

    it('UPDATE_USER no modifica el estado si no hay usuario logueado', () => {
        const user = { id: 1, nombre: 'Juan', email: 'juan@cef.com', rol: 'cliente' as const };
        const newState = authReducer(initialState, { type: 'UPDATE_USER', payload: { user } });
        expect(newState.user).toBeNull();
    });
});