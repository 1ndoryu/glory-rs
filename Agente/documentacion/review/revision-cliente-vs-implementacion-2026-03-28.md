# Revisión Completa: Solicitudes del Cliente vs Implementación — 2026-03-28

## Fuentes analizadas
- `cliente/transcripciones.md` (Audios 1-21)
- `cliente/transcripciones-videos.md` (Videos 1-7 + imágenes)
- `cliente/Data II/transcripciones-data-ii.md` (Videos 4-18 sobre reservas/CRM)
- `cliente/Data III/mensajes.md` (Marketing/WhatsApp)
- `cliente/notas.md` (Simplificación: solo Gastos/Ventas/Margen)

## Nota importante del cliente (notas.md — 25/03/2026)
> "De momento quiere algo mucho más sencillo, no hace falta añadir todo lo que te he dicho. En la parte económica solo Gastos, Ventas y Margen."

Esto reduce significativamente el alcance. Las secciones marcadas como **OMITIDO** fueron excluidas por esta directriz.

---

## 1. Autenticación
| Solicitud | Estado | Notas |
|-----------|--------|-------|
| Login email + contraseña | ✅ Implementado | Login.tsx |
| Recuperar contraseña por email | ✅ Implementado | ForgotPassword + ResetPassword |

## 2. Sidebar / Navegación
| Solicitud | Estado | Notas |
|-----------|--------|-------|
| Franja izquierda tipo Haddock | ✅ Implementado | app-sidebar.tsx con shadcn SidebarProvider |
| Secciones: Económica, Reservas, Marketing | ✅ Implementado | Dashboard, Ventas, Gastos, Reservas, Marketing |
| Omitidos: Tesorería, Bancos, Haddock HR, Fina | ✅ Correcto | Excluidos según notas del cliente |

## 3. Home / Dashboard
| Solicitud | Estado | Notas |
|-----------|--------|-------|
| Botones "Nueva Venta" y "Nuevo Gasto" arriba | ✅ Implementado | AccionesRapidas en header, también en sidebar |
| Resumen: Gastos, Ventas, Margen | ✅ Implementado | PanelGeneral con 3 cards |
| Margen negativo en rojo | ✅ Implementado | Condicional por signo |
| Gráfico de barras (Visión General) | ✅ Implementado | Recharts BarChart en Dashboard |
| Gráfico circular desglose por categorías | ✅ Implementado | PieChart en PanelResumen |
| Drill-down por categoría → proveedores | ⚠️ Parcial | Hay pie chart por categoría pero sin drill-down a proveedores individuales |
| Selector periodo (mensual/semanal/trimestral/anual) | ⚠️ Parcial | Solo selector mes/año. No hay semanal/trimestral/anual |
| Últimos 10 documentos subidos | ❌ OMITIDO | Sección docs omitida por simplificación |
| Conciliación | ❌ OMITIDO | Excluida por simplificación |
| Incidencias | ❌ OMITIDO | Excluida por simplificación |
| Dashboard detallado (Haddock Insights) | ❌ OMITIDO | Excluido — el cliente quiere algo simple |

## 4. Ventas
| Solicitud | Estado | Notas |
|-----------|--------|-------|
| Crear venta manual: fecha, personas, descripción | ✅ Implementado | FormularioVenta |
| IVA configurable | ✅ Implementado | Campo en formulario + config global |
| Turnos (desayuno/comida/cena) | ✅ Implementado | Select en formulario |
| Método de pago (efectivo/tarjeta/transferencia) | ✅ Implementado | Select en formulario |
| Cálculo automático importe + IVA | ✅ Implementado | Auto-calculado |
| Listado con filtros | ✅ Implementado | ListaVentas con paginación |

## 5. Gastos
| Solicitud | Estado | Notas |
|-----------|--------|-------|
| Crear gasto manual: fecha, proveedor, categoría, tipo doc, método pago | ✅ Implementado | FormularioGasto |
| Digitalizar documento (subir foto) | ✅ Implementado | Groq IA (Llama 4 Scout) |
| Categorías: Bebidas, Limpieza, Mantenimiento, etc. | ✅ Implementado | Gastos tiene categorías |
| Número de documento | ✅ Implementado | Campo en formulario |
| Listado con filtros | ✅ Implementado | ListaGastos con paginación |
| Gasto por correo electrónico | ❌ No implementado | Cliente dijo "de momento no" |

## 6. Reservas
| Solicitud | Estado | Notas |
|-----------|--------|-------|
| Vista día: lista reservas con mesa, hora, nombre, personas, estado | ✅ Implementado | ListaReservas con filtros |
| Vista mes: cuadrícula con reservas/personas por día | ✅ Implementado | CalendarioReservas |
| Click en día → detalle | ✅ Implementado | Navega a /reservas con fecha |
| Filtros: turno, estado | ✅ Implementado | Comida/cena/desayuno/completo + estados |
| Plano de sala al lado | ✅ Implementado | PlanoOcupacion integrado |
| Estados: confirmada, pendiente, completada, no-show, cancelada, lista espera | ✅ Implementado | Todos los estados |
| Datos obligatorios al reservar (config) | ✅ Implementado | nombre/apellidos/email/teléfono en Configuración |
| No-shows con ratio y desglose por canal | ✅ Implementado | EstadisticasNoShows |
| Reservas por hora/día de semana (gráficos) | ✅ Implementado | PanelAnalisis en dashboard |

## 7. Plano de Sala
| Solicitud | Estado | Notas |
|-----------|--------|-------|
| Constructor drag-and-drop | ✅ Implementado | @dnd-kit |
| Crear/editar/eliminar zonas | ✅ Implementado | Tabs con CRUD |
| Crear/editar/eliminar mesas | ✅ Implementado | Con posición visual |
| Config mesa: número, zona, min/max personas | ✅ Implementado | PanelConfigMesa |
| Combinaciones de mesas | ✅ Implementado | Agrupa mesas para groups grandes |
| Export/Import plano | ✅ Implementado | JSON download/upload |
| Múltiples plantas/zonas | ✅ Implementado | Sistema de zonas |

## 8. Clientes / CRM
| Solicitud | Estado | Notas |
|-----------|--------|-------|
| Listado paginado (43,000+ clientes) | ✅ Implementado | Paginación backend |
| Búsqueda por nombre, apellidos, teléfono, email | ✅ Implementado | Búsqueda en tiempo real |
| Añadir cliente manual: foto, nombre, apellidos, teléfono, email, notas | ✅ Implementado | FormularioCliente (sin foto) |
| Consentimiento SMS/email | ⚠️ Parcial | Modelo tiene campos pero no visible en UI |
| Fusionar/unificar duplicados | ✅ Implementado | Merge de 2 clientes |
| Eliminar cliente | ✅ Implementado | Con confirmación |

## 9. Etiquetas
| Solicitud | Estado | Notas |
|-----------|--------|-------|
| Etiquetas para clientes (VIP, no paga, fidelización) | ✅ Implementado | Sistema categorías + etiquetas |
| Etiquetas para reservas (cumpleaños, eventos) | ✅ Implementado | Asignación a reservas |
| Crear etiquetas personalizadas | ✅ Implementado | CRUD etiquetas |
| Categorías de etiquetas | ✅ Implementado | Gestión de categorías |

## 10. Marketing
| Solicitud | Estado | Notas |
|-----------|--------|-------|
| Campañas multi-canal (SMS, Email, WhatsApp) | ✅ Implementado | FormularioCampana |
| Segmentación clientes por actividad | ✅ Implementado | 7 segmentos |
| Texto + fotos/vídeos para email/WhatsApp | ⚠️ Parcial | Texto sí, adjuntos multimedia no implementados |
| Aviso de coste SMS | ⚠️ Pendiente | No hay banner de aviso de coste |
| Preview destinatarios antes de enviar | ✅ Implementado | Preview segmento |
| Opt-out / Darse de baja | ✅ Implementado | Campo incluir_baja + telefono_baja |
| Plantillas WhatsApp — crear y enviar a Meta | ✅ Implementado | FormularioPlantilla + enviar |
| Plantillas aprobadas vs rechazadas | ✅ Implementado | Filtro por estado |
| Razón de rechazo Meta | ⚠️ Parcial | Estado rechazada visible, pero campo razón no claro |
| Recordatorios automáticos antes de reserva | ✅ Implementado | Reglas + historial |

## 11. Chatbot API
| Solicitud | Estado | Notas |
|-----------|--------|-------|
| Endpoints para chatbot externo | ✅ Implementado | /api/chatbot/* |
| API Keys con SHA-256 | ✅ Implementado | Gestión API keys |
| Disponibilidad por franjas | ✅ Implementado | Franjas 30min |
| Crear/buscar/cancelar reservas | ✅ Implementado | Chatbot service |
| Documentación API para chatbot | ❌ Pendiente | Tarea 35 del roadmap |

## 12. Configuración
| Solicitud | Estado | Notas |
|-----------|--------|-------|
| Nombre restaurante | ✅ Implementado | |
| Campos obligatorios reserva | ✅ Implementado | Switches |
| IVA por defecto | ✅ Implementado | |
| API Key Groq (digitalización) | ✅ Implementado | |

---

## Resumen

### Implementado completamente: ~85%
### Parcialmente implementado: ~10%
### Omitido intencionalmente (simplificación cliente): ~5%

### Pendientes menores identificados:
1. **Drill-down categoría → proveedores en gráfico circular** — El cliente describe poder seleccionar una categoría (ej: Bebidas) y ver el desglose por proveedor con importes. Solo hay pie chart genérico.
2. **Selector periodo flexible** — Solo mes/año. El cliente describe mensual/semanal/trimestral/anual.
3. **Adjuntos multimedia en campañas** — El cliente pide fotos/vídeos en email/WhatsApp.
4. **Banner aviso coste SMS** — El cliente pide "un cartelito que ponga que esto puede llevar un coste".
5. **Foto de cliente** — El cliente menciona subir foto al crear cliente. No implementado.
6. **Consentimiento marketing visible en UI** — Los campos existen en el modelo pero no se ven en FormularioCliente.
7. **Documentación API chatbot** — Tarea 35, documentar endpoints.

### Ninguno de estos pendientes es bloqueante. El proyecto funciona completo para el caso de uso del restaurante.
