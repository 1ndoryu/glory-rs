## Stack implementado

| Capa                 | Herramienta                                    |
| -------------------- | ---------------------------------------------- |
| Framework web        | Axum 0.7                                       |
| OpenAPI              | utoipa 4 + utoipa-swagger-ui 7                 |
| Serialización        | serde                                          |
| Base de datos        | SQLx 0.8 (PostgreSQL)                          |
| Migraciones          | SQLx migrate                                   |
| Validación           | validator 0.18                                 |
| Variables de entorno | dotenvy                                        |
| Logging              | tracing + tracing-subscriber                   |
| Errores              | thiserror 2                                    |
| Auth                 | jsonwebtoken + argon2                          |
| CORS                 | tower-http                                     |
| Linter               | clippy (deny all + warn pedantic)              |
| Frontend             | React 18 + TypeScript + Vite                   |
| State                | React Query + Zustand                          |
| Codegen              | Orval 8 (reemplaza openapi-typescript-codegen) |

## Notas

En caso de que puedas analizar todos los archivos en jpg y de audio en la carpeta cliente, hazlo para tener un roadmap mas detallado, las funcionalidades de reservas y marketing quedan pendientes pero puedes adelantar, por favor cualquier duda que se tenga dejarla anotada. Por favor usa un diseño minimalista, simplemente, intuitivo. Tambien ajusta coolify-manager-rs para desplegar. Probablemente haya cosas que el cliente no especifica o no sepa que necesita, debes intuirla como un panel de configuración para el dueño del restaurante. 

Puedes ajustar los pendientes. Si puedes analizar todo el contenido y sacar el texto de los audios, hazlo sin dudar. 

# Especificaciones del Proyecto: Plataforma de Gestión de Restaurantes

> **⚠️ NOTA IMPORTANTE DE ÚLTIMA HORA:** > El cliente ha notificado una instrucción directa del propietario del restaurante para simplificar radicalmente el proyecto. **En la parte económica, el sistema ahora solo debe incluir las funciones básicas de Gastos, Ventas y Margen** (Ventas menos Gastos).
> _Por lo tanto, la mayoría de los módulos analíticos, de conciliación y gestión documental detallados a continuación quedan temporalmente OMITIDOS o PAUSADOS para priorizar esta versión simplificada._

---

## 1. Acceso y Navegación Base (Activo)

- **Login:** Debe haber una pantalla inicial pidiendo correo electrónico y contraseña.
- **Recuperación:** Opción de "olvidé mi contraseña", que enviará un enlace al correo del usuario para actualizarla.
- **Estructura de la Interfaz:** Un menú de navegación fijo en la franja izquierda; al pulsar una opción, el contenido se despliega en el resto de la pantalla a la derecha.

## 2. Menú Lateral Izquierdo (Activo)

- Toma como base visual la plataforma "Haddock".
- **Secciones a Eliminar:** Se deben quitar los apartados de "Gente", el usuario "Fina", "Tesorería", "Bancos" y "Haddock HR" dentro de Administración.
- **Secciones a Añadir:** Se deben incluir apartados nuevos para "Reservas" y "Marketing". Los detalles de cómo funcionarán por dentro se definirán más adelante tras estudiar otras plataformas.

## 3. Pantalla Principal (Home)

- **Botones de Acción:** En la parte superior, dos botones principales: "Nueva Venta" (verde) y "Nuevo Gasto" (azul).
- **Módulo de Reservas:** Debe haber un apartado que muestre la cantidad de reservas programadas en el mes actual (ej. 200 o 300) y un conteo del día actual. Al pulsar, debe desplegar más detalles como listado de clientes y horarios.
- **Últimos Documentos (Omitido):** Una fila visual mostrando los últimos 10 documentos subidos, ordenados del más nuevo (izquierda) al más antiguo (derecha), con un botón "Ver documentos" para ir a la vista general.
- **Atajos (Omitidos):** Paneles rápidos para ver el estado de la Conciliación y las Incidencias pendientes.

## 4. Registro Manual de Operaciones (Activo en versión simple)

- **Nueva Venta:** Formulario que permite añadir fecha, comensales (opcional), descripción, porcentaje de IVA y si permite duplicados. Permite marcar el turno (mañana, mediodía, noche), el canal de venta (comedor, barra, terraza, delivery, Just Eat, eventos...) y el método de pago (efectivo, tarjeta, transferencia, etc.) junto con el importe base y el IVA.
- **Nuevo Gasto:** Desplegable con opciones de digitalizar archivo, crear gasto manual o por correo (esta última descartada por ahora). El modo manual pide fecha, proveedor, categoría, tipo de documento (factura, albarán, ticket), método de pago, número de documento y una opción de recurrencia.

---

> 🛑 **A PARTIR DE AQUÍ: TODO HA SIDO OMITIDO/PAUSADO POR LA SIMPLIFICACIÓN DE ÚLTIMA HORA** 🛑

## 5. Dashboard y Analítica (Omitido)

- **Resumen de Beneficios:** Tres bloques (Gastos, Ventas, Margen) que restan gastos a ventas, mostrando el resultado en rojo si es negativo.
- **Gráficos de Barras:** Visualización de actividad con ganancias en verde y gastos en negro/rojo, filtrables de forma semanal, mensual, trimestral o anual.
- **Gráfico Circular (Tarta):** Desglose con pestañas para Gastos, Resultados y Ventas. Al pulsar una categoría (ej. Bebidas), muestra el subtotal, el IVA sumado de las facturas, el total y los proveedores específicos de esa categoría. Al pulsar en un proveedor, redirige a los documentos filtrados de ese mes.
- **Dashboard Detallado (Multilocal):** Sección para propietarios con varios restaurantes ("Centros").
- **Gráficas Dinámicas:** Barras de evolución de ventas, gastos, clientes y ticket medio que cambian su escala (eje Y) según los importes (ej. saltos de 100 en 100) y muestran la cifra exacta al pasar el ratón por encima.

## 6. Administración de Documentos (Omitido)

- Listado de todos los archivos con su estado actual: digitalizado, requiere revisión o rechazado.
- Filtros por fecha de subida, proveedor, categoría, tipo de documento y estado de pago.
- Al abrir un documento, extrae el importe base, la cuota de IVA separada por porcentajes (0%, 4%, 10%, 21%) y el total.

## 7. Conciliación (Omitido)

- Sistema para emparejar lo que se pidió (albarán) con lo que se cobró (factura), verificando artículos y cantidades.
- Lista de conciliaciones manuales donde se escogen documentos compatibles de los últimos 90 días.
- Un botón de "Analizar con IA", cuyo funcionamiento estaba pendiente de aclararse con el dueño.

## 8. Incidencias (Omitido)

- Gestor de problemas (ej. devoluciones, productos en mal estado, diferencias de precios).
- Tres columnas de estado: Nuevas, En curso (estudiándose) y Resueltas.
- Ficha de incidencia con: Estado, proveedor, prioridad (baja, media, alta), motivo, documento vinculado y un botón de "Contactar proveedor" para enviar el reporte directamente.

# Pendientes.

(Sin tareas pendientes)