# Plan: Reescritura Frontend con shadcn/ui — 263A-16

## Contexto
Usuario pidió rehacer toda la interfaz con shadcn/ui. Tareas 17-23 quedan absorbidas por esta.
Preset: `bKGlrC2C`. Block: `dashboard-01`. Dark/light mode requerido.

## Estado actual
- 22 componentes + 9 hooks + 12 CSS files + 1 Zustand store
- API generada por Orval (tags-split) — SE MANTIENE intacta
- Submodulo glory-rs/frontend UI components (Input, Boton, Select, Textarea, Modal) — SE REEMPLAZAN con shadcn

## Fases

### Fase 1: Setup shadcn/ui en proyecto existente
- [ ] Instalar Tailwind CSS + @tailwindcss/vite
- [ ] Configurar tsconfig paths (@/*)
- [ ] Actualizar vite.config.ts (alias @, plugin tailwindcss)
- [ ] Ejecutar `npx shadcn@latest init --preset bKGlrC2C`
- [ ] Agregar dashboard-01 block: `npx shadcn@latest add dashboard-01`
- [ ] Agregar componentes base: button, input, select, dialog, table, card, tabs, calendar, chart, dropdown-menu, sidebar, sheet, separator, badge, avatar, tooltip, label, textarea, switch, toggle, theme-toggle
- [ ] Configurar dark/light mode (ThemeProvider)

### Fase 2: Infraestructura (Layout, Auth, Routing)
- [ ] Login.tsx → shadcn card + form + input + button
- [ ] ForgotPassword.tsx + ResetPassword.tsx → shadcn
- [ ] Layout.tsx → shadcn sidebar (collapsible, estado persiste con localStorage)
- [ ] BarraLateral.tsx → shadcn sidebar con iconos lucide
- [ ] ThemeToggle (dark/light mode) en sidebar footer
- [ ] Eliminar CSS: Login.css, BarraLateral.css, Layout.css

### Fase 3: Dashboard + Home
- [ ] Inicio.tsx → usar dashboard-01 block como base, agregar KPIs de ventas/gastos/reservas
- [ ] Integrar charts de shadcn (ventas/gastos mensuales, reservas por canal)
- [ ] Combinar Inicio + Dashboard en una sola página (tarea 19)
- [ ] Eliminar CSS: Inicio.css, DashboardReservas.css

### Fase 4: CRUD pages (Ventas, Gastos, Clientes, Canales, Etiquetas)
- [ ] ListaVentas.tsx → shadcn table + dialog para crear
- [ ] FormularioVenta.tsx → shadcn form + inputs
- [ ] ListaGastos.tsx → shadcn table + dialog
- [ ] FormularioGasto.tsx → shadcn form
- [ ] ListaClientes.tsx → shadcn table + dialog + search
- [ ] FormularioCliente.tsx → shadcn form
- [ ] ListaCanales.tsx → shadcn table + dialog
- [ ] Eliminar CSS: Formularios.css
- [ ] Botones eliminar = iconos (tarea 17)
- [ ] menuGasto flex column + iconos en vez de emojis (tarea 18)

### Fase 5: Reservas + Calendario + No-Shows
- [ ] ListaReservas.tsx → shadcn table + filtros + dialog
- [ ] CalendarioReservas.tsx → shadcn calendar/custom grid
- [ ] EstadisticasNoShows.tsx → shadcn charts + cards
- [ ] DashboardReservas.tsx → shadcn charts (absorbe en Fase 3 dashboard unificado)
- [ ] Eliminar CSS: Calendario.css, NoShows.css

### Fase 6: Plano de Sala
- [ ] PlanoSala.tsx → mantener @dnd-kit, reestilizar con Tailwind
- [ ] PlanoOcupacion.tsx → Tailwind
- [ ] MesaDraggable.tsx → Tailwind
- [ ] PanelConfigMesa.tsx → shadcn card + form
- [ ] Alertas → shadcn dialog/sheet (tarea 23)
- [ ] Mejorar UX: drag primero, nombre después (tarea 23)
- [ ] Eliminar CSS: PlanoSala.css, PlanoOcupacion.css

### Fase 7: Configuración + Limpieza
- [ ] Configuracion.tsx → shadcn form + switch
- [ ] Eliminar CSS: Configuracion.css
- [ ] Eliminar todos los CSS files restantes (global.css, App.css)
- [ ] Eliminar dependencia de glory-rs/frontend UI components
- [ ] Sidebar collapsible con estado persistente (tarea 22)
- [ ] Verificar responsive en 320px, 768px, 1024px (regla 15)
- [ ] `npx tsc --noEmit` limpio
- [ ] Build producción exitoso

## Tareas absorbidas
- 17: botones eliminar → iconos (Fase 4)
- 18: menuGasto flex + iconos (Fase 4)
- 19: combinar inicio + dashboard (Fase 3)
- 21: pieBarraLateral colores (Fase 2, sidebar shadcn)
- 22: sidebar collapsible + estado persistente (Fase 7)
- 23: plano sala UX (Fase 6)

## Decisiones técnicas
- API generada NO se toca — solo cambia UI
- Hooks se pueden reutilizar en gran medida (la lógica no cambia, solo la presentación)
- recharts → shadcn charts (basado en recharts 2, compatible)
- @dnd-kit se mantiene para plano de sala
- Zustand se mantiene para auth
