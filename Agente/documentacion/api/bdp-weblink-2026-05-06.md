# BDP WebLink REST API — Integracion inicial

## Estado 2026-05-06

La base de integracion queda preparada para validar el PC del restaurante sin guardar credenciales en el repositorio. La documentacion completa pegada desde PDF vive en `# WEBLINK RESTAPI.md` y la configuracion operativa queda por restaurante en `configuracion_restaurante`.

## Implementado

- Columnas `bdp_*` en `configuracion_restaurante`: URL publica, login, password, codigo integrador, toggle sync y IDs operativos de POS/empleado/perfil de articulos.
- Cliente backend `BdpWeblinkClient` con `Health`, `Login`, `GetVersion`, timeout de 20s, manejo de `ErrorMessage` y sanitizado de bodies HTTP.
- Catalogo backend de rutas/payloads BDP para articulos, clientes, comandas, pagos, departamentos, terminales y empleados.
- Endpoint `GET /api/configuracion/bdp/diagnostico` para probar Health + Login + GetVersion sin exponer credenciales.
- Pantalla de configuracion BDP con URL, credenciales, IDs operativos, toggle y boton de diagnostico.
- Tags OpenAPI de reseñas renombrados a `resenas` y Orval configurado con `clean: true` para evitar carpetas generadas corruptas en Windows.

## Mapa tecnico extraido del manual

| Area | Ruta | Uso previsto |
| --- | --- | --- |
| Servicio | `/Service/Health`, `/Service/GetVersion`, `/Auth/Login` | Diagnostico remoto y sesion autenticada. |
| Articulos | `/API/Articles/Export` | Sincronizar catalogo web marcado como Articulo Web en BDP. |
| Articulos | `/API/Articles/GetPOSList` | Leer articulos filtrados por perfil de departamentos/articulos del TPV. |
| Clientes | `/API/Customers/Export`, `/API/Customers/Create` | Exportar clientes o crear/sobrescribir cliente antes de una venta/comanda. |
| Comandas | `/API/Orders/Create`, `/API/Orders/Get`, `/API/Orders/Cancel` | Crear, consultar o cancelar comandas por `OrderId`, marketplace o mesa. |
| Pagos | `/API/Orders/Payment/Add`, `/API/Orders/Invoice` | Registrar cobros y facturar comandas desde POS/empleado configurados. |
| Departamentos | `/API/Departments/Export`, `/API/Departments/ExportFromProfile` | Obtener departamentos generales o por perfil operativo. |
| Terminales | `/API/POS/Get`, `/API/POSes/Get` | Resolver terminales validos para cobros/cancelaciones. |
| Empleados | `/API/Employee/Get`, `/API/Employees/Get`, `/API/POS/Employees/Get` | Resolver camareros/vendedores validos para crear y cerrar comandas. |
| Formas de pago | `/API/Tenders/GetList`, `/API/Tenders/GetPOSList` | Mapear metodos de pago locales contra `TenderId` de BDP. |

## Mapeo operativo previsto

- Catalogo: `BdpExportArticlesRequest::all_web_articles(1)` cubre el barrido inicial; `BdpGetPosArticlesRequest::first_page(profile, page_size)` queda para perfiles de TPV.
- Cliente local: antes de enviar una comanda con datos fiscales, crear/sobrescribir cliente con `/API/Customers/Create` y guardar el codigo BDP si el entorno real lo exige.
- Venta/reserva: se convierte en `/API/Orders/Create` con `EmployeeId`, `ItemsProfileId`, `OrderEndType` y estructura `Order`. La respuesta real decidira si guardamos `OrderId`, par `MarketId`/`MarketplaceOrderId` o mesa.
- Pago: los metodos locales deben mapearse contra `/API/Tenders/GetPOSList` para enviar `TenderId` correcto a `/API/Orders/Payment/Add`.
- Facturacion: si el pago no factura automaticamente, usar `/API/Orders/Invoice` con `PosId`, `EmployeeId` y el `OrderIdentifier` persistido.

## Checklist remoto BDP

- Si no hay tecnico local, compilar y usar `remote_access_bootstrap.exe` desde el `release` del target activo de Cargo para dejar Tailscale, RustDesk y reporte final listos en el PC remoto.
- Confirmar que BDP-NET esta activo y sin modo demo.
- Confirmar subscripcion extendida de WebLink REST API.
- Abrir `Utilidades -> Configuracion Servicios Web` en el PC servidor BDP.
- Confirmar puerto publico, firewall de Windows y NAT/router.
- Confirmar que el servicio exige login/password en entorno real.
- Pedir a BDP el `CodigoIntegrador` y validar que Login devuelve `AuthSession.Token`.
- Ejecutar diagnostico desde Configuracion: Health debe responder `IsAlive=true`; Login debe devolver sesion; GetVersion debe devolver `ErrorMessage` vacio.

## Supuestos y gotchas

- El manual no explicita el header de sesion para comandos autenticados. El cliente usa `Authorization: Bearer <token>` y esta encapsulado en un unico punto para ajustar si BDP usa otro header.
- `TiempoSession` se fija en 59 minutos porque el manual lo declara como maximo.
- La API BDP usa nombres PascalCase y errores de negocio en `ErrorMessage`; no tratar HTTP 200 como exito sin revisar ese campo.
- Las respuestas de articulos, clientes, comandas y pagos quedan como JSON hasta probar contra el PC real. El manual es grande y no conviene cerrar structs definitivos sin ver datos reales de BDP-NET.
- La validacion real queda pendiente hasta tener URL/puerto alcanzable del PC, firewall/NAT abierto y credenciales definitivas del restaurante.
