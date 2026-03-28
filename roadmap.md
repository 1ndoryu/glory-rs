## Stack implementado

> **URL producción:** http://app-b8s0cks444o0sogo8kg8wcgw.66.94.100.241.sslip.io (Swagger UI: /swagger-ui/)
> **URL directa:** http://66.94.100.241:3001
> **Servidor:** 66.94.100.241 (Coolify, servicio UUID: b8s0cks444o0sogo8kg8wcgw)
> **Deploy:** `scripts/deploy-server.sh` via SSH (requiere DB_PASSWORD y JWT_SECRET como env vars)

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

# Pendientes

## Gaps de la revisión (features activos que faltan)

- ~~Recuperación de contraseña ("olvidé mi contraseña") — envía enlace al email para resetear. Pedido en Audio 1 original.~~ → 263A-15

## Módulo de Reservas — Expansión (Data II, basado en Cover Manager)

> Referencia visual: Cover Manager. Documentación completa en `Agente/documentacion/reservas/analisis-data-ii-2026-03-26.md`

### Fase 1 — Core (COMPLETADA)

- ~~No-shows: tracking de reservas no presentadas, ratio por día/mes (porcentaje), filtro por canal.~~ → 263A-8
- ~~Canales de reserva: definir canales por donde entran reservas para estadísticas.~~ → 263A-9
- ~~CRM clientes + Etiquetas~~ → 263A-5
- ~~Vista reservas día + mes~~ → 263A-6+263A-7
- ~~OpenAPI tags + Orval tags-split~~ → 263A-10

### Fase 2 — Dashboard y visualización (COMPLETADA)

- ~~Dashboard reservas — Panel resumen: total reservas + comparativa mes anterior, reservas por día, por día de semana, distribución por canal, clientes nuevos, ocupación % (personas y mesas).~~ → 263A-13
- ~~Dashboard reservas — Panel ocupación: media personas/reserva, media reservas/día, total reservas, gráfico por hora, por día semana, ocupación %, por turno, reservas con antelación, distribución por procedencia.~~ → 263A-13
- ~~Dashboard reservas — Panel análisis: reservas efectivas (sin cancelaciones/no-shows), total comensales efectivos, comensales/reserva, ticket medio por reserva y por persona.~~ → 263A-13

### Fase 3 — Plano de sala (COMPLETADA)

- ~~Constructor de plano: el dueño construye su plano arrastrando mesas. Configurar cada mesa: número, zona, mín/máx personas. Múltiples plantas/zonas (barra, restaurante, terraza).~~ → 263A-14
- ~~Combinación de mesas: seleccionar mesas combinables, definir máx/mín personas en combinación.~~ → 263A-14
- ~~Exportar/importar plano de sala para backup.~~ → 263A-14
- ~~Mostrar plano con mesas ocupadas en la vista de reservas por día.~~ → 263A-16 (integración Fase 3b)

### Fase 4 — Pendiente de detalle del cliente

- Marketing: el cliente indicó que dará detalles más adelante.
- Merge de clientes duplicados: unificar 2 clientes que son la misma persona.

### Fase 4 — Marketing (Data III, basado en Cover Manager SMS)

> Referencia: `cliente/Data III/mensajes.md` + transcripción video + captura Cover Manager SMS
> El cliente pide un módulo donde el propietario cree campañas de marketing multi-canal.

#### Fase 4a — Campañas manuales
- Crear campañas de publicidad multi-canal: SMS, email, WhatsApp (seleccionar una, varias o todas a la vez).
- Redactar mensaje con texto + adjuntar fotos/vídeos (para email y WhatsApp).
- Segmentación de destinatarios por actividad: clientes habituales, sin venir 1/3/6/9/12/+12 meses (extensible).
- Formulario campaña: nombre interno, descripción interna, cuerpo del mensaje, aviso de coste SMS, opción baja de comunicaciones comerciales.
- Contador de caracteres SMS (máx 160, caracteres especiales ¡¿ cuentan doble).
- Referencia visual: captura Cover Manager (`cliente/Data III/WhatsApp Image...`).

#### Fase 4b — Plantillas WhatsApp (Meta Business API)
- Pestaña "Plantillas" dentro de Marketing.
- Crear nueva plantilla: texto + archivos (fotos/vídeos) → enviar a Meta para aprobación.
- Historial de plantillas aprobadas (ordenadas por fecha, preview de imagen/primer frame + cuerpo).
- Historial de plantillas no aprobadas (misma vista + columna "Razón" del rechazo).
- Integración con Meta Business API / WhatsApp Business API para envío y gestión de plantillas.

#### Fase 4c — Recordatorios automáticos
- Los recordatorios de reserva deben enviarse automáticamente (no manualmente).
- Definir reglas: cuándo enviar (ej: 24h antes, 1h antes), por qué canal.

#### Fase 4d — Merge clientes duplicados
- Unificar 2+ clientes que son la misma persona en el CRM.

#### Fase 4e — Métricas de marketing (futuro)
- Medir cuántos clientes vinieron al restaurante después de recibir un mensaje de campaña.

## ~~Configuración del restaurante~~ → 263A-17

- ~~Configuración de datos obligatorios al reservar (email, teléfono, nombre, apellidos).~~
- ~~IVA por defecto del establecimiento.~~

## Tareas pendientes

~~6. deberia unificarse npm run verify y self-check.ps1~~ → 263A-18

~~7. Agregar una regla de que frontend\src\api\generated.ts siempre tiene que estar dividido~~ → 263A-19

~~8. Cuando todo este listo desplegar con coolify-manager-rs, ajustar lo que sea necesario~~ → 263A-20 (Fase 1 completada)

~~9. Error `cargo run` sin especificar binario en dev.ps1~~ → 263A-21

~~10. Crear template `rust-stack.yaml` en coolify-manager-rs y desplegar automáticamente con `new --name`~~ → 263A-22

~~11. Implementar Módulo de Marketing — Fase 4a (campañas manuales: backend + frontend)~~ → 263A-23

~~12. Implementar Módulo de Marketing — Fase 4b (plantillas WhatsApp / Meta Business API)~~ → 263A-24

~~13. Implementar Módulo de Marketing — Fase 4c (recordatorios automáticos de reservas)~~ → 263A-25

~~14. Merge de clientes duplicados — Fase 4d~~ → 263A-26

~~15. Veo algunos problemas como

Failed to load resource: net::ERR_CONNECTION_REFUSEDComprende este error
:3000/api/auth/login:1  Failed to load resource: the server responded with a status of 401 (Unauthorized)Comprende este error
:3000/api/reservas/no-shows:1  Failed to load resource: the server responded with a status of 500 (Internal Server Error)Comprende este error
:3000/api/reservas/no-shows:1  Failed to load resource: the server responded with a status of 500 (Internal Server Error)Comprende este error
:3000/api/reservas/no-shows:1  Failed to load resource: the server responded with a status of 500 (Internal Server Error)
DELETE http://localhost:3000/api/ventas/de507550-2c03-4a42-8717-d6a547058825 405 (Method Not Allowed)~~ → 263A-15


16. ~~me di cuenta qeu son muchos problemas de diseño que lo mejor en este caso es usar https://ui.shadcn.com, rehacer toda la interfaz en https://ui.shadcn.com, tambien seria bueno preparar el white y black mode, usar es block "npx shadcn@latest add dashboard-01" shadcn tambien tiene Charts & Graphs, usaremos esos, he creado este present para este proyecto --preset bKGlrC2C (https://ui.shadcn.com/docs/installation)~~ → 263A-16

(todo lo siguiente fue planificado antes de 16, no tomar tanto en cuenta si se refiere al diseño pues, shadcn solucionara alguans cosas)

~~17. los botoenes de eliminar deberian ser iconos en vez de texto,~~ → 263A-16

~~18. .menuGasto debería display flex column y esta usando emojis en vez de icono lo cual es una violacion~~ → 263A-16

~~19. Creo que inicio y dashboard deberían convinarse en una sola cosa al menos que claro que el cliente lo haya pedido por separado pero no croe.~~ → 263A-27

~~20. Siempre dice No hay reservas para esta fecha pero en calendario sale que si. (Datos de prueba) Pero ya vi que es porque "todos los estados" en realidad no muestran todos los estados.~~ → 263A-20

21. ~~En pieBarraLateral las letras son oscuras y el fondo tambien.~~ → 263A-16

22. ~~La barra lateral debería poder encogerse a solo iconos y expandirse, el estado debe perdurar aunque se recargue la pestaña.~~ → 263A-16

23. ~~La forma en la que se construye el plano de la sala, por ajemplo abre alertas en vez de modales, no tiene iconos, deberia poder arrastrarse primero y despues colocar el nombre, cosas basicas que mejoran la experiencia de usuario, tambien veo que los estados no se conservan al mover las mesas.~~ → 263A-16 (parcial: UI mejorada con shadcn, estados de mesas requiere revisión adicional)

24. ~~Me parece que tailwind no esta funcionando, pues todo se ve muy diferente a como es shadcn en en realidad.~~ → 263A-16

~~25. Cambia el preset de shadcn a --preset b0, cambiar de white a dark mode no funciona, revisa si tailwind esta funcionando correctamente~~ → 263A-25

~~25. La forma en la que se construye el plano de la sala, por ajemplo abre alertas en vez de modales, no tiene iconos, deberia poder arrastrarse primero y despues colocar el nombre, cosas basicas que mejoran la experiencia de usuario, tambien veo que los estados no se conservan al mover las mesas. (no se arreglo a pesar de shadcn, hace falta mejorar la ui ux), por ejemplo las alertas deberian ser dialogos (componente de shadcn)~~ → 263A-28

~~26. LOs botones en el panel lateral de navegación no tienen gap, y los botones de crear venta y gasto en el panel lateral la palabra "nueva es redudante"~~ → 263A-29

~~27. Cambia el present de shandc a --preset bJfDQCym~~ → 263A-30

~~28. el main necesita gap~~ → 263A-31

~~29. Cambiar de white a dark o viceversa no funciona.~~ → 263A-32

~~30. Donde dice nuevo gasto deberia ser un flex columna a demas de que esta usando emoji en vez de iconos~~ → 263A-33

~~31. las tab son muy feas, usan un borde blanco que desentona con el resto de cosas, esto en dashboard, deberian ser tabs de shandc~~ → 263A-34

32. El cliente comento que se necesita una api para que un chatbot de ia se pueda comunicar, sus palabras exactas son "Perdona Wan que no te lo comenté pero este sistema tienenq eu tener apis disponibles para conectar un chatbot externo que atienda a los clientes y pueda ver toda la info de las reservas para cuando hay hueco y que pueda registyrar las reservas en la base de datos", solo dice esto pero asumimos que necesita hacer mas cosas. 

33. El cliente tambien comento, sus palabras exactas "otro apunte también que, bueno, no sé si te influye en algo o no, pero te lo digo por si acaso. O sea, esta página web no es para que se registren un montón de propietarios de restaurantes. Es para que cada propietario entre ahí y registre los restaurantes que tiene, que normalmente será uno, porque es raro el caso de que haya una misma persona que tenga más de un restaurante.

Pero la idea es que, si vamos consiguiendo más negocios, creamos esta misma página web para cada propietario individual, o sea, una por restaurante, vamos, una por propietario. Entonces por eso te digo que no sé si te ayuda o no, pero te lo digo por si acaso.

También por si tienes algo... yo no entiendo de hacer páginas web, pero por si tienes algo así tipo plantilla que puedas guardar el diseño o algo para luego no tener que hacerlo desde cero, que lo guardes, ¿sabes? Y eso." 

Es decir, asumo que hay que hacer una revisión de todo para ver que partes, que cosas podemos mover al glory-rs para poder reutilizar en futuros proyectos para facilitar el desarrollo de futuras plataformas, no hablo solo de logica de restaurante, si todo lo que sea posible sin afectar el proyecto actual, logica de email, logica de formulario, lo que sea, todo debe ir muy bien organizado, con una arquitectura muy buena y eficiente. Esto requiere una planificacion y revision detallada de varios pasos supongo.

34. Vi que pusiste en español nombre de carpetas, las carpetas deben ir siempre en ingles. 