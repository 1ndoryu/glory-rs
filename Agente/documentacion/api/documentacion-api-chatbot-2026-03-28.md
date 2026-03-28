# API del Chatbot — Documentación

## Descripción
API REST para integrar un chatbot externo con el sistema de gestión del restaurante. Permite consultar disponibilidad, crear y gestionar reservas, y obtener información del restaurante.

## Autenticación
Todos los endpoints del chatbot usan **API key** en el header `X-API-Key`.

```
X-API-Key: glry_abc123...
```

Las API keys se gestionan desde el panel de administración (sección Configuración) o vía los endpoints `/api/api-keys` (autenticados por JWT).

### Crear una API key
```
POST /api/api-keys
Authorization: Bearer <jwt_token>
Content-Type: application/json

{ "nombre": "Chatbot WhatsApp" }
```

Respuesta (201):
```json
{
  "id": "uuid",
  "nombre": "Chatbot WhatsApp",
  "key": "glry_abc123...",       // Solo visible una vez
  "key_prefix": "glry_abc"
}
```

### Listar API keys
```
GET /api/api-keys
Authorization: Bearer <jwt_token>
```

### Revocar API key
```
DELETE /api/api-keys/{id}
Authorization: Bearer <jwt_token>
```

---

## Endpoints del Chatbot

Base URL: `https://<dominio>/api/chatbot`

### 1. Consultar disponibilidad

```
GET /api/chatbot/disponibilidad?fecha=2026-03-28
X-API-Key: <key>
```

**Respuesta (200):**
```json
{
  "fecha": "2026-03-28",
  "capacidad_total": 60,
  "franjas": [
    {
      "hora": "12:00:00",
      "personas_reservadas": 8,
      "mesas_ocupadas": 3,
      "mesas_disponibles": 12,
      "capacidad_disponible": 52
    },
    {
      "hora": "12:30:00",
      "personas_reservadas": 12,
      "mesas_ocupadas": 5,
      "mesas_disponibles": 10,
      "capacidad_disponible": 48
    }
  ]
}
```

Las franjas son intervalos de 30 minutos. `capacidad_disponible` indica cuántas personas más pueden reservar en esa franja.

---

### 2. Información del restaurante

```
GET /api/chatbot/restaurante
X-API-Key: <key>
```

**Respuesta (200):**
```json
{
  "nombre": "La Española",
  "capacidad_total": 60,
  "campos_obligatorios": {
    "nombre": true,
    "apellidos": false,
    "email": true,
    "telefono": true
  },
  "zonas": [
    {
      "nombre": "Comedor principal",
      "mesas": 10,
      "capacidad_min": 2,
      "capacidad_max": 6
    },
    {
      "nombre": "Terraza",
      "mesas": 5,
      "capacidad_min": 2,
      "capacidad_max": 4
    }
  ]
}
```

`campos_obligatorios` indica qué datos del cliente son requeridos al crear una reserva.

---

### 3. Crear reserva

```
POST /api/chatbot/reservas
X-API-Key: <key>
Content-Type: application/json

{
  "fecha": "2026-03-30",
  "hora": "14:00:00",
  "nombre_cliente": "María García",
  "num_personas": 4,
  "telefono": "+34612345678",
  "apellidos_cliente": "García López",
  "email": "maria@email.com",
  "notas": "Mesa junto a la ventana si es posible"
}
```

**Campos requeridos:** `fecha`, `hora`, `nombre_cliente`, `num_personas`
**Campos opcionales:** `telefono`, `apellidos_cliente`, `email`, `notas`

**Respuesta (201):**
```json
{
  "id": "550e8400-e29b-41d4-a716-446655440000",
  "fecha": "2026-03-30",
  "hora": "14:00:00",
  "nombre_cliente": "María García",
  "apellidos_cliente": "García López",
  "num_personas": 4,
  "estado": "pendiente",
  "telefono": "+34612345678",
  "notas": "Mesa junto a la ventana si es posible",
  "mesa_numero": null
}
```

**Errores posibles:**
- `409 Conflict` — No hay capacidad disponible para la fecha/hora solicitada
- `422 Unprocessable Entity` — Error de validación (campos requeridos faltantes, formato inválido)

---

### 4. Buscar reservas

```
GET /api/chatbot/reservas?telefono=+34612345678
GET /api/chatbot/reservas?nombre=María
GET /api/chatbot/reservas?fecha=2026-03-30
GET /api/chatbot/reservas?telefono=+34612345678&fecha=2026-03-30
X-API-Key: <key>
```

Todos los parámetros son opcionales. `nombre` busca en nombre Y apellidos.

**Respuesta (200):**
```json
[
  {
    "id": "550e8400-e29b-41d4-a716-446655440000",
    "fecha": "2026-03-30",
    "hora": "14:00:00",
    "nombre_cliente": "María García",
    "apellidos_cliente": "García López",
    "num_personas": 4,
    "estado": "pendiente",
    "telefono": "+34612345678",
    "notas": "",
    "mesa_numero": null
  }
]
```

---

### 5. Obtener reserva por ID

```
GET /api/chatbot/reservas/{id}
X-API-Key: <key>
```

**Respuesta (200):** Mismo formato que la respuesta de crear reserva.
**Error:** `404` si la reserva no existe o no pertenece al restaurante de la API key.

---

### 6. Cancelar reserva

```
DELETE /api/chatbot/reservas/{id}
X-API-Key: <key>
```

**Respuesta (200):**
```json
{
  "ok": true,
  "message": "Reserva cancelada"
}
```

La reserva no se elimina, su estado cambia a `cancelada`.
**Error:** `404` si la reserva no existe.

---

## Estados de reserva

| Estado | Descripción |
|--------|-------------|
| `pendiente` | Reserva creada, pendiente de confirmar |
| `confirmada` | Reserva confirmada por el restaurante |
| `completada` | El cliente asistió |
| `cancelada` | Reserva cancelada |
| `no_show` | El cliente no se presentó |
| `lista_espera` | En espera de disponibilidad |

---

## Errores comunes

| Código | Significado |
|--------|-------------|
| 401 | API key inválida, revocada o ausente |
| 404 | Recurso no encontrado |
| 409 | Conflicto (sin capacidad disponible) |
| 422 | Error de validación |
| 500 | Error interno del servidor |

---

## Notas de integración

- Las API keys se hashean con SHA-256 en el servidor. La key completa solo se muestra una vez al crearla.
- Cada API key está asociada a un usuario (propietario del restaurante). Todas las operaciones se ejecutan en el contexto de ese usuario.
- El sistema valida automáticamente la disponibilidad al crear reservas (capacidad de mesas y personas).
- Las horas deben enviarse en formato `HH:MM:SS`. Las fechas en formato `YYYY-MM-DD`.
- El Swagger UI está disponible en `/swagger-ui/` para explorar interactivamente todos los endpoints.
