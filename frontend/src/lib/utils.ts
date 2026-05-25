// Utilidad generada por shadcn/ui. No modificar.
// cn() combina clases de Tailwind resolviendo conflictos (ej: "px-4 px-2" → "px-2").
import { clsx, type ClassValue } from "clsx"
import { twMerge } from "tailwind-merge"

export function cn(...inputs: ClassValue[]) {
  return twMerge(clsx(inputs))
}
