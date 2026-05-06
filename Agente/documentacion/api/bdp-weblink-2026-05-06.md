# BDP WebLink REST API — Integracion inicial

## Estado 2026-05-06

La base de integracion queda preparada para validar el PC del restaurante sin guardar credenciales en el repositorio. La documentacion completa pegada desde PDF vive en `# WEBLINK RESTAPI.md` y la configuracion operativa queda por restaurante en `configuracion_restaurante`.

## Implementado

- Columnas `bdp_*` en `configuracion_restaurante`: URL publica, login, password, codigo integrador, toggle sync y IDs operativos de POS/empleado/perfil de articulos.
- Cliente backend `BdpWeblinkClient` con `Health`, `Login`, `GetVersion`, timeout de 20s, manejo de `ErrorMessage` y sanitizado de bodies HTTP.
- Endpoint `GET /api/configuracion/bdp/diagnostico` para probar Health + Login + GetVersion sin exponer credenciales.
- Pantalla de configuracion BDP con URL, credenciales, IDs operativos, toggle y boton de diagnostico.
- Tags OpenAPI de reseñas renombrados a `resenas` y Orval configurado con `clean: true` para evitar carpetas generadas corruptas en Windows.

## Checklist remoto BDP

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
- La integracion de articulos, clientes, comandas y pagos queda para la siguiente fase, despues de validar conectividad real con el PC del restaurante.