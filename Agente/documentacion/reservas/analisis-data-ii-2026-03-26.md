# Análisis Data II — Nuevos Requerimientos del Sistema de Reservas

> **Fecha:** 2026-03-26
> **Fuente:** 18 archivos multimedia (15 videos + 3 audios) en `cliente/Data II/`
> **Referencia visual:** Plataforma Cover Manager (sistema de reservas que usa el dueño del restaurante)

## Resumen

El cliente envió material detallado sobre cómo funciona Cover Manager, la plataforma de reservas que actualmente usa el dueño del restaurante. Los nuevos requerimientos expanden significativamente el módulo de reservas, añadiendo un CRM de clientes, gestor de etiquetas, planos de sala, no-shows y dashboards de ocupación.

**La simplificación de última hora ("solo Gastos/Ventas/Margen") aplica SOLO a la parte económica.** Las reservas y el marketing son módulos separados que siguen activos.

---

## Módulos Nuevos Identificados

### 1. Vista de Reservas — Día (Video 4, Video 7)
- Ver reservas por periodo: desayuno / comida / cena / día completo
- Por cada reserva mostrar: nº mesa, hora, nombre cliente, apellidos, personas, estado, teléfono
- Filtros por estado: confirmadas, pendientes, lista de espera
- Junto a la lista, mostrar plano del restaurante con mesas ocupadas

### 2. Vista de Reservas — Mes (Video 5)
- Cuadrícula tipo calendario con el mes seleccionado
- Por cada día: cantidad de personas + mesas ocupadas
- Totales mensuales arriba: personas totales + mesas totales
- Al hacer clic en un día, ir a la vista de día

### 3. Historial de Reservas (Video 8)
- Listado histórico de reservas con las mismas etiquetas/filtros

### 4. CRM — Listado de Clientes (Video 9, Audio 1/2)
- **Rendimiento crítico:** el restaurante tiene ~43,000 clientes — debe funcionar con paginación eficiente
- Campos: nombre, apellidos, teléfono, email, empresa, etiquetas, notas del cliente
- Buscar por: nombre, apellidos, teléfono, correo
- Acciones: añadir cliente manual, editar, eliminar, descargar reserva
- Al añadir cliente: foto (opcional), nombre, apellidos, teléfono (con prefijo), correo, notas, consentimiento comercial (email/SMS), encuestas de satisfacción, info adicional (alergias, preferencias de bebida, ubicación preferida)
- Futuro: unificar/merge de clientes duplicados

### 5. Gestor de Etiquetas (Video 10, Video 11)
- **Etiquetas para clientes:**
  - Preestablecidas del sistema: VIP, no paga, nivel de fidelización (frecuente/ocasional/poco)
  - Categorías: preferencias alimentarias, de bebida, de ubicación, alergias/intolerancias
  - El dueño puede crear etiquetas custom en cualquier categoría
  
- **Etiquetas para reservas:**
  - Eventos, peticiones especiales, servicios gastronómicos
  - Ejemplo: "cumpleaños", "evento corporativo"
  - También pueden ser custom

### 6. Plano de Sala (Video 13, Video 14, Video 15)
- **Constructor de plano (drag-and-drop):**
  - El dueño construye su propio plano del restaurante
  - Añadir mesas, arrastrar para posicionar
  - Configurar cada mesa: número, zona, mín/máx personas
  - Múltiples plantas/zonas: barra, restaurante, terraza, etc.
  
- **Combinación de mesas (Video 14):**
  - Seleccionar varias mesas que se pueden combinar
  - Definir máx/mín personas en la combinación
  - El sistema entiende que si necesita 8 asientos, puede combinar 4 mesas de 2
  
- **Exportar/importar plano (Video 15):**
  - Backup del plano para restaurar en caso de fallo

### 7. No-Shows (Audio 3)
- Tracking de reservas que no se presentaron
- Filtros: por período (día, mes)
- Ratio/porcentaje: "reservaron 50, cancelaron 25 → ratio 50%"
- Filtro por canal de entrada: WhatsApp, Instagram, teléfono, etc.

### 8. Dashboard de Reservas (Video 16, Video 17, Video 18)

**Panel 1 — Resumen del establecimiento (Video 16):**
- Total reservas + comparativa con mes anterior
- Reservas por día
- Reservas por día de la semana
- Canales de reserva (Instagram, WhatsApp, teléfono) + conteo
- Evolución de canales (gráfico de líneas por canal)
- Clientes nuevos (conteo + gráfica)
- Ocupación porcentual: por personas y por mesas

**Panel 2 — Ocupación (Video 17):**
- Media de personas por reserva
- Media de reservas al día
- Total de reservas
- Gráfico: reservas por hora (reservas + personas)
- Gráfico: reservas por día de la semana
- Ocupación por personas y mesas (%)
- Por turno/servicio
- Reservas con antelación (cuántos días antes se reserva)
- Distribución por procedencia/canal

**Panel 3 — Análisis de reservas (Video 18):**
- Reservas efectivas (excluyendo cancelaciones y no-shows)
- Total comensales de reservas efectivas
- Comensales por reserva
- Ticket medio por reserva y por persona

### 9. Configuración de Reservas (Video 12)
- Datos obligatorios al reservar: email, teléfono, nombre, apellidos
- IVA del cliente por defecto
- Idioma de la interfaz (español)

### 10. Marketing (Video 12)
- "Me lo salto porque me lo voy a explicar más adelante" — pendiente de detalle

---

## Priorización Sugerida

### Fase 1 — Core de Reservas (necesario)
1. CRM de clientes (listado, crear, editar, buscar) — alta prioridad por los 43k clientes
2. Vista de reservas mejorada (día/mes)
3. Gestor de etiquetas básico (clientes + reservas)
4. No-shows tracking

### Fase 2 — Experiencia Visual
5. Dashboard de reservas (3 paneles)
6. Plano de sala básico (mesas + zonas sin drag-and-drop visual complejo)

### Fase 3 — Avanzado
7. Plano de sala con drag-and-drop completo
8. Combinación de mesas
9. Exportar/importar plano
10. Marketing (pendiente de detalle del cliente)

---

## Video 6 — No transcrito
`WhatsApp Video 2026-03-25 at 1.36.22 PM.mp4` (37.5MB) falló por desconexión repetida de la API. Por contexto (entre Video 5 sobre vista mes y Video 7 sobre filtros de reserva día), probablemente continúa la explicación de la vista calendario o configuración de reservas. No es bloqueante.
