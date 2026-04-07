# Plan: Chat en Vivo con IA para Nakomi Studio
**Fecha:** 2026-04-04  
**ID tarea:** 044A-36  
**Estado:** Planificación

---

## Resumen

Sistema de chat en vivo donde clientes (anónimos) pueden comunicarse con el equipo de Nakomi Studio. IA responde cuando no hay personal disponible. El personal puede intervenir en cualquier momento, ver lo que el cliente escribe en tiempo real y gestionar conversaciones desde un panel dedicado.

---

## Requisitos del usuario

1. Clientes pueden escribir **sin estar logueados**
2. **IA responde automáticamente** cuando no hay personal disponible
3. Personal puede **intervenir en conversaciones** atendidas por IA
4. **Panel de gestión** para el personal: ver todas las conversaciones
5. Personal puede **ver lo que escribe el usuario antes de enviar** (typing preview)

---

## Arquitectura propuesta

### Stack

| Componente | Tecnología | Justificación |
|---|---|---|
| WebSocket server | Axum + tokio-tungstenite | Ya usamos Axum, soporte nativo de WS |
| Persistencia | PostgreSQL (SQLx) | Consistente con el stack actual |
| IA | OpenAI GPT-4o / Gemini API | API madura, streaming de respuestas |
| Frontend cliente | React widget (embebido) | Se monta como overlay flotante |
| Frontend panel | React (ruta protegida /panel/chat) | Integrado al panel existente |
| Sesiones anónimas | UUID + cookie/localStorage | Sin autenticación requerida |

### Flujo de datos

```
Cliente (widget) ←→ WebSocket ←→ Chat Service ←→ PostgreSQL
                                      ↕
                                   AI Service (OpenAI/Gemini)
                                      ↕
                    Panel Staff ←→ WebSocket ←→ Chat Service
```

---

## Fases de implementación

### Fase 1 — Backend: modelo de datos y WebSocket (más difícil)

**Tablas:**

```sql
CREATE TABLE chat_sessions (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    visitor_id VARCHAR(64) NOT NULL,        -- UUID anónimo del visitante
    visitor_name VARCHAR(100),               -- Nombre opcional
    status VARCHAR(20) NOT NULL DEFAULT 'active',  -- active | ai_handling | staff_handling | closed
    assigned_staff_id UUID REFERENCES users(id),
    ai_enabled BOOLEAN NOT NULL DEFAULT true,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE chat_messages (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    session_id UUID NOT NULL REFERENCES chat_sessions(id) ON DELETE CASCADE,
    sender_type VARCHAR(10) NOT NULL,  -- 'visitor' | 'ai' | 'staff'
    sender_id VARCHAR(64),              -- visitor_id, 'ai', o user.id
    content TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE chat_typing (
    session_id UUID PRIMARY KEY REFERENCES chat_sessions(id) ON DELETE CASCADE,
    content TEXT NOT NULL DEFAULT '',
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_chat_messages_session ON chat_messages(session_id, created_at);
CREATE INDEX idx_chat_sessions_status ON chat_sessions(status);
```

**WebSocket endpoints:**
- `GET /ws/chat/visitor?session_id=X` — Conexión del visitante
- `GET /ws/chat/staff` — Conexión del staff (requiere JWT)

**Mensajes WebSocket (JSON):**

```typescript
// Cliente → Servidor
{ type: 'message', content: string }
{ type: 'typing', content: string }          // Lo que está escribiendo (preview)
{ type: 'close' }

// Servidor → Cliente (visitante)
{ type: 'message', sender: 'ai' | 'staff', content: string }
{ type: 'typing', sender: 'staff', content: string }
{ type: 'status', value: 'ai_handling' | 'staff_handling' }

// Servidor → Staff
{ type: 'message', session_id: string, sender: 'visitor', content: string }
{ type: 'typing', session_id: string, content: string }   // Preview de lo que escribe el visitante
{ type: 'session_new', session: ChatSession }
{ type: 'session_closed', session_id: string }

// Staff → Servidor
{ type: 'join', session_id: string }          // Tomar conversación
{ type: 'message', session_id: string, content: string }
{ type: 'close', session_id: string }
{ type: 'toggle_ai', session_id: string, enabled: boolean }
```

**Chat Service (lógica central):**
1. Nuevo visitante → crear `chat_session` con `status: 'ai_handling'`
2. Mensaje de visitante:
   - Si `ai_enabled` y no hay staff asignado → enviar a AI Service → responder
   - Si hay staff asignado → reenviar al staff por WS
3. Staff hace `join` → `status: 'staff_handling'`, `ai_enabled: false`
4. Staff puede reactivar IA con `toggle_ai`
5. `typing` del visitante → broadcast a staff conectado a esa sesión
6. `typing` del staff → enviar al visitante

**Archivos Rust:**
- `src/handlers/chat.rs` — WebSocket upgrade, routing WS
- `src/services/chat.rs` — Lógica de negocio, gestión de sesiones en memoria
- `src/services/ai_chat.rs` — Integración con API de IA
- `src/models/chat.rs` — Structs de BD
- `src/repositories/chat.rs` — Queries SQLx
- `migrations/YYYYMMDD_chat.up.sql`

### Fase 2 — AI Service

**Flujo:**
1. Recibe mensaje del visitante + historial de la sesión
2. System prompt: contexto de Nakomi Studio (servicios, precios, info general)
3. Llama a OpenAI/Gemini con streaming
4. Cada chunk de respuesta → enviar por WS al visitante en tiempo real
5. Al completar → guardar mensaje completo en BD

**System prompt template:**
```
Eres el asistente virtual de Nakomi Studio, una agencia de desarrollo web y diseño.
Servicios: {lista de servicios con precios}.
Responde de forma concisa, amable y profesional. Si la pregunta requiere atención
humana (presupuesto específico, problema técnico, reunión), indica que un miembro
del equipo se conectará pronto.
Idioma: responde en el mismo idioma que el visitante.
```

**Secrets:** `OPENAI_API_KEY` o `GEMINI_API_KEY` en `.env`

### Fase 3 — Widget de chat (frontend cliente)

**Componente:** `ChatWidget.tsx` — Overlay flotante en esquina inferior derecha.

**Funcionalidad:**
- Botón flotante para abrir/cerrar
- Input de mensaje + enviar
- Lista de mensajes con scroll auto
- Indicador "escribiendo..." del staff/IA
- Identificación de mensajes por sender_type (colores distintos)
- Sonido de notificación en mensaje nuevo
- Almacena `visitor_id` y `session_id` en localStorage para reconectar
- Se monta en todas las páginas excepto /panel

**CSS:** Overlay con z-index alto, responsive, animación de entrada/salida.

**Tamaño estimado:** ~200 líneas TSX + ~150 líneas CSS + hook `useChatWidget.ts` ~100 líneas

### Fase 4 — Panel de staff

**Ruta:** `/panel/chat` (protegida por JWT)

**Layout:**
```
┌──────────────────────────────────────────────┐
│  Conversaciones activas  │  Chat seleccionado│
│                          │                    │
│  [●] Visitante abc-123   │  [visitor] Hola,   │
│      "Quiero un sitio.." │  necesito...       │
│                          │                    │
│  [●] Visitante def-456   │  [ai] ¡Hola! Te   │
│      "Cuanto cuesta..."  │  puedo ayudar...   │
│                          │                    │
│                          │  [visitor typing:]  │
│                          │  "Cuanto cuesta un" │
│                          │                    │
│                          │  ┌──────────────┐  │
│                          │  │ Escribir...   │  │
│                          │  └──────────────┘  │
│                          │  [Tomar] [IA On/Off]│
└──────────────────────────────────────────────┘
```

**Funcionalidad:**
- Lista de sesiones activas con preview del último mensaje
- Indicador de quién atiende: IA (ícono robot) o Staff (ícono persona)
- Preview en tiempo real de lo que escribe el visitante (typing preview)
- Botón "Tomar conversación" → desactiva IA, asigna staff
- Toggle IA on/off por conversación
- Botón "Cerrar conversación"
- Notificación de nueva sesión (sonido + badge)

**Archivos:**
- `frontend/src/islands/PanelChatIsland.tsx`
- `frontend/src/islands/PanelChatIsland.css`
- `frontend/src/hooks/usePanelChat.ts`

### Fase 5 — Typing preview (lo más innovador)

**Implementación:**
- Cada `onChange` del input del visitante → enviar `{ type: 'typing', content: value }` por WS
- Throttle a 200ms para no saturar
- El servidor reenvía al staff conectado a esa sesión
- En el panel, se muestra en un area dedicada debajo del chat: texto gris, cursiva, actualización en tiempo real
- Privacy: el visitante NO ve lo que escribe el staff (unidireccional)

---

## Estimación de complejidad

| Fase | Complejidad | Dependencias |
|---|---|---|
| 1. Backend WS + modelo | Alta | Ninguna |
| 2. AI Service | Media | Fase 1, API key |
| 3. Widget cliente | Media | Fase 1 |
| 4. Panel staff | Media-Alta | Fase 1 |
| 5. Typing preview | Baja | Fase 1 + 3 + 4 |

**Orden de implementación:** 1 → 2 → 3 → 4 → 5

---

## Consideraciones de seguridad

- **Rate limiting** en el WebSocket: max 10 msg/min por visitante, max 1 sesión activa por visitor_id
- **Sanitización** de contenido: escapar HTML en mensajes antes de renderizar
- **API keys de IA** solo en servidor, nunca expuestas al frontend
- **JWT obligatorio** para endpoints de staff
- **Cleanup automático**: cerrar sesiones inactivas >30 min
- **Almacenamiento**: retener mensajes 90 días, luego purgar

---

## Dependencias nuevas

**Rust (Cargo.toml):**
- `tokio-tungstenite` — WebSocket support para Axum (ya soportado nativamente)
- `async-openai` o `reqwest` — Cliente para API de IA
- `dashmap` — Concurrent hashmap para sesiones en memoria

**Frontend (package.json):**
- Ninguna nueva — WebSocket API es nativa del navegador

---

## Notas

- El WebSocket server puede vivir en el mismo binding de Axum (misma instancia)
- ws.nakomi.studio ya está configurado como servicio en Coolify (nakomi-rust stack)
- Para producción: considerar Redis pub/sub si se escala a múltiples instancias del backend
- El system prompt de la IA debe actualizarse automáticamente con los datos de servicios/precios
