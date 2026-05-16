// Memoria compartida de la aplicación para saber quién está logueado (Context API + useReducer).
// Guarda el usuario y su token de sesión (JWT). Cualquier página puede leerlo sin que se lo pasen explícitamente (via props).
// Existe porque múltiples pantallas necesitan saber quién es el usuario y qué permisos tiene.
export {};
