// Componente de ruta exclusiva para el dueño.
// Si no hay token redirige a /login; si el rol no es "dueno" redirige a /clases.
// Existe para proteger /admin sin duplicar la lógica de control de acceso en la página.
export {};
