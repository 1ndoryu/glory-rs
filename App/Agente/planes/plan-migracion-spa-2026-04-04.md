# Plan: Migración Frontend Islands → SPA React Router

## Fases

### Fase 1: Infraestructura (deps + routing + entry points)
- [x] Copiar archivos App/React/ → frontend/src/
- [x] Copiar App/Assets/ → frontend/public/assets/
- [ ] Instalar deps faltantes (lucide-react, react-router-dom ya está)
- [ ] Crear navegacionSPA.tsx como wrapper de react-router useNavigate (mantener API `navegar()`)
- [ ] Reescribir App.tsx con React Router rutas
- [ ] Actualizar main.tsx
- [ ] Eliminar appIslands.tsx (reemplazado por rutas)

### Fase 2: Fix imports
- [ ] miembros.ts: cambiar import de equipo images a /assets/equipo/
- [ ] useImagenes.ts: eliminar import.meta.glob de Glory/, usar /assets/ o placeholders
- [ ] Eliminar todas las ref a GLORY_CONTEXT (usar fallback directo)

### Fase 3: Validación
- [ ] npm run type-check
- [ ] npm run build
- [ ] Corregir todos los errores

## Estado: Fase 1 en progreso
