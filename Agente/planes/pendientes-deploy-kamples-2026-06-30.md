# Pendientes post-sesión 2026-06-30 — Deploy Kamples

> **Estado de la sesión:** Bloqueada en deploy Docker. Código correcto, Dockerfile corregido, pero build `--no-cache` toma ~15 min en servidor.
> **Producción actual:** `https://samples.nakomi.studio` → container `app-mo4so4440c488g8woow4cow0` (sano, sirve tráfico, **SIN** el fix SVG).

---

## Lo que se hizo esta sesión

### ✅ Fix 404 imágenes de colores en producción
- **Problema:** `legacy-assets/colors/` está en `.gitignore`, no existe en producción. El plugin Vite solo funciona en dev.
- **Solución:** Fallback SVG determinista en `src/handlers/image_proxy.rs` — `generar_svg_placeholder()` con hash MD5 como semilla para gradientes. Cache 24h para SVGs, 1 año para imágenes reales.
- **Commit:** `121f34ff` → push a `origin/kamples` ✅ (ya en GitHub)
- **Verificación:** Funciona en container de prueba (`docker exec curl` → 200)

### ✅ Dockerfile corregido con ffmpeg + HEALTHCHECK
- **Problema:** Dockerfile en servidor fue corrompido por heredoc de PowerShell (`${VAR}` → `\`, `&&` → `;`)
- **Solución:** Generado localmente, subido via base64 al servidor. Incluye:
  - `ffmpeg` en runtime (audio pipeline)
  - `HEALTHCHECK` explícito con `http://127.0.0.1:3000/api/health` (evita que Coolify inyecte el de IPv6)
  - Copiado a `Dockerfile` y `Dockerfile.rust` en servidor

### ✅ Push del fix a origin
- El commit `121f34ff` estaba solo local + `framework/kamples`. Se hizo `git push origin kamples` → ya está en `glory-rs-template.git`.

---

## Pendientes inmediatos

### 🔴 Deploy no completado
- **Builds corriendo:** Al cerrar sesión había 7 procesos `cargo/rustc` en el servidor (dos builds `--no-cache` simultáneos: manual + Coolify API)
- **Container sano actual** (`app-mo4so4440c488g8woow4cow0`) NO tiene el fix SVG (imagen de 22:34 CEST, antes del push)
- **Container nuevo** (`app-b8s0cks444o0sogo8kg8wcgw`) ciclando unhealthy (healthcheck IPv6 viejo)
- **Acción:** Esperar a que terminen los builds, verificar que la nueva imagen tiene el fix:
  ```bash
  # En servidor:
  docker exec <nuevo-container> curl -s -o /dev/null -w '%{http_code}' \
    'http://localhost:3000/api/img/legacy-assets/colors/04ffa3e324cb6b79acf9f17cd89b32da.jpg?fmt=webp'
  # Debe retornar 200
  ```
- Si los builds fallaron: `docker compose -f /data/coolify/services/mo4so4440c488g8woow4cow0/docker-compose.yml up -d` para volver al estado sano

### 🟡 Verificar healthcheck post-deploy
- Coolify inyecta su propio healthcheck con `hostname -i` (retorna IPv6 sin brackets → curl falla)
- El HEALTHCHECK en el Dockerfile debería prevalecer sobre el de Coolify
- Verificar: `docker inspect <container> --format '{{json .Config.Healthcheck}}'`
- Si sigue inyectando el de Coolify: necesitamos quitar el `healthcheck:` del `docker-compose.yml` en servidor

### 🟡 Tauri app → producción (no verificado)
- La app Tauri se conecta a `https://kamples.com/wp-json` (no a `samples.nakomi.studio`)
- Rust adapter rewrites `/wp-json/...` → `/api/...`
- CSP permite `samples.nakomi.studio`
- **Pendiente:** Verificar que `kamples.com` apunta a `66.94.100.241` (mismo servidor)

---

## Lecciones aprendidas

1. **PowerShell SSH + heredoc = destrucción.** `${VAR}`, `&&`, `"$@"` se mangan SIEMPRE. Usar base64 para subir archivos.
2. **Push ANTES de build.** El Dockerfile clona de GitHub — si el código no está en `origin/kamples`, el build no lo tiene.
3. **Coolify overridea HEALTHCHECK del compose.** Ponerlo en el Dockerfile como directiva `HEALTHCHECK` tiene prioridad.
4. **`docker compose build --no-cache` en servidor es lento** (~15 min para Rust). Considerar CI/CD o builds en local + push de imagen.
5. **Dos remotos (`origin` vs `framework`)** — los commits se van a `framework` por defecto. Verificar `git push origin kamples` siempre.

---

## Archivos en workspace (limpieza pendiente)

| Archivo | Acción | Razón |
|---|---|---|
| `frontend/.../imagenesColorLista.ts` | Commit | Timestamp update de script auto-generado |
| `logs/extraccion.lock` | Ignorar | Estado runtime, cambia constantemente |
| `logs/scraping.lock` | Ignorar | Estado runtime |
| `Agente/planes/plan-deploy-coolify-2026-06-30.md` | Commit | Plan de deploy |
| `check-compose.ps1` | Ignorar | Script debug temporal |
| `check-fqdn.ps1` | Ignorar | Script debug temporal |
| `check-service.ps1` | Ignorar | Script debug temporal |
| `fix-fqdn.ps1` | Ignorar | Script debug temporal |
| `fix-fqdn2.ps1` | Ignorar | Script debug temporal |
| `deploy-compose.yml` | Ignorar | Compose temporal |
| `docker-compose.coolify.yml` | Ignorar | Compose temporal |
| `get-compose.ps1` | Ignorar | Script debug temporal |
| `prepare-deploy.ps1` | Ignorar | Script debug temporal |
| `update-service.ps1` | Ignorar | Script debug temporal |
| `verify-compose.ps1` | Ignorar | Script debug temporal |
| `register_migrations.sql` | Ignorar | SQL temporal |
| `test-sine.wav` | Ignorar | Archivo de test |
| `clients/kamples-scraper/.lock_daily` | Ignorar | Lock scraper |
| `C:\Users\Owner\Dockerfile.rust` | Local only | Copia de trabajo del Dockerfile |
| `C:\Users\Owner\dfrust.b64` | Borrar | Base64 temporal |
