# Plan: Diagnóstico y Recuperación de Kamples

> **Fecha:** 2026-06-16
> **Estado:** Planificación
> **UUID Stack:** `mo4so4440c488g8woow4cow0`
> **Dominio:** kamples.com
> **Template:** kamples (WordPress + PostgreSQL legacy)

---

## Resumen de la situación

Kamples era un sitio WordPress + PostgreSQL (2 bases de datos). Actualmente:
- **Estado en Coolify:** `exited` (caído)
- **Disco reportado:** 31.2 GB (según dashboard de infraestructura)
- **Backups disponibles:** 2 dumps PostgreSQL (muy pequeños, ~1.1 MB y ~252 KB — indican pérdida de datos anterior al backup)
- **Historial:** Se intentó restaurar antes sin éxito, creando bases de datos vacías

## Preguntas clave por resolver

| # | Pregunta | Método de investigación |
|---|----------|------------------------|
| 1 | ¿Los contenedores de kamples siguen existiendo? | `docker ps -a --filter label=coolify.stack-uuid=mo4so4440c488g8woow4cow0` |
| 2 | ¿Dónde están los 31.2 GB exactamente? | `docker system df`, `du -sh /data/coolify/services/mo4so4440c488g8woow4cow0/*` |
| 3 | ¿Hay volúmenes Docker húerfanos con datos? | `docker volume ls`, inspeccionar tamaños |
| 4 | ¿La BD PostgreSQL tiene datos reales o está vacía? | `docker exec postgres pg_dump` o `psql` count de tablas |
| 5 | ¿La BD WordPress (MySQL) tiene datos? | `docker exec mariadb mysqldump` o count de tablas |
| 6 | ¿Hay archivos de uploads o media en bind mounts? | `du -sh /data/uploads/studio /data/coolify/services/*/volumes/*` |
| 7 | ¿Qué dice el docker-compose.yml actual en disco? | Comparar bind mounts vs named volumes |
| 8 | ¿Por qué salió el contenedor? | Logs del contenedor: `docker logs --tail 50` |
| 9 | ¿Hay datos en el filesystem overlay de Docker? | `docker inspect` para ver tamaños de capas |
| 10 | ¿Qué backups existen en Google Drive/SSH remote? | `coolify-manager-rs backup --list --name kamples` |

---

## Fase 1: Crear comando `diagnose` en coolify-manager-rs

### Objetivo
Agregar un comando `diagnose --name kamples` que ejecute toda la investigación vía SSH (usando infraestructura `russh` existente) y devuelva un reporte estructurado.

### Comportamiento esperado

```bash
coolify-manager.exe diagnose --name kamples
```

### Output esperado

El comando debe imprimir un reporte como este:

```
═══ Diagnóstico: kamples ═══
UUID: mo4so4440c488g8woow4cow0

── Contenedores ──
  app:       exited (code: 0)
  postgres:  running
  mariadb:   running
  redis:     running

── Uso de disco ──
  Docker system:     31.2 GB
    └─ volumes:      28.0 GB
    └─ containers:   1.2 GB
    └─ images:       1.0 GB
    └─ build cache:  1.0 GB
  Docker volumes:
    - kamples_data_uploads:    15.0 GB
    - kamples_db_data:         12.0 GB
    - kamples_wordpress_data:  1.0 GB
  Bind mounts:
    - /data/uploads/kamples:   15.0 GB (existente)
    - /data/coolify/services/.../wordpress/wp-content/uploads:  5.0 GB

── Base de datos PostgreSQL ──
  Estado:          running
  Base de datos:   kamples
  Tablas:          37
  Registros totales: ~120,000
  Tamaño:          8.2 GB
  Último dump:     2026-04-27 (252 KB — sospechosamente pequeño)
  Nota:            dump muy pequeño para el tamaño actual de BD

── Base de datos MySQL (WordPress) ──
  Estado:          running
  Base de datos:   wordpress
  Tablas:          12
  Registros totales: ~5,000
  Tamaño:          256 MB

── Sistema de archivos ──
  Uploads bind:    /data/uploads/kamples → 15.0 GB
  WordPress media: /data/.../uploads → 5.0 GB
  Docker overlay:  2.0 GB

── Logs recientes ──
  [error] Fatal: No database selected
  [error] WP: Error establishing a database connection
  ...

── Resumen ──
  * Los datos de PostgreSQL PARECEN existir (~8 GB en volumen)
  * Los dumps de backup son sospechosamente pequeños
  * El contenedor app salió por error de conexión a BD
  * Uploads y media están intactos
```

### Datos concretos a recolectar (vía SSH)

```bash
# 1. Estado del stack Docker
docker ps -a --filter label=coolify.stack-uuid=mo4so4440c488g8woow4cow0 --format '{{.Names}}\t{{.Status}}\t{{.Image}}'

# 2. Uso de disco del stack
docker system df --format '{{.Type}}\t{{.TotalCount}}\t{{.Size}}'

# 3. Volúmenes Docker
docker volume ls --filter label=coolify.stack-uuid=mo4so4440c488g8woow4cow0 --format '{{.Name}}'
# Para cada volumen: du -sh /var/lib/docker/volumes/{name}/_data/

# 4. BD PostgreSQL - tamaño y conteo de tablas
docker exec postgres-{uuid} psql -U kamples_app -d kamples -c "
SELECT schemaname, tablename, n_live_tup, pg_size_pretty(pg_total_relation_size(quote_ident(schemaname)||'.'||quote_ident(tablename)))
FROM pg_stat_user_tables
ORDER BY n_live_tup DESC;"

# 5. BD WordPress - tamaño
docker exec mariadb-{uuid} mysql -u wordpress -p{PASSWORD} -e "
SELECT table_schema, SUM(data_length+index_length)/1024/1024 AS size_mb,
COUNT(*) AS tables FROM information_schema.tables WHERE table_schema='wordpress' GROUP BY table_schema;"

# 6. Docker compose on-disk
cat /data/coolify/services/mo4so4440c488g8woow4cow0/docker-compose.yml | head -100

# 7. Bind mounts reales
ls -la /data/uploads/kamples/ 2>/dev/null
du -sh /data/uploads/kamples/ 2>/dev/null

# 8. Logs del contenedor app
docker logs {container-name} --tail 30 2>&1

# 9. Docker inspect - tamaño real
docker inspect {container-name} --format '{{.SizeRootFs}} {{.SizeRw}}'
```

### Implementación

#### Archivos a modificar/crear

| Archivo | Acción |
|---------|--------|
| `src/cli/mod.rs` | Agregar variante `Diagnose` al enum `Command` con campo `--name` |
| `src/cli/dispatch/site.rs` | Agregar `Command::Diagnose => commands::diagnose::execute()` |
| `src/commands/diagnose.rs` | **NUEVO** — handler principal del comando |
| `src/services/diagnose_service.rs` | **NUEVO** — lógica de recolección de datos |
| `src/domain/mod.rs` | Agregar tipos `DiagnoseReport`, `ContainerStatus`, `DiskUsage`, `DbInfo` |
| `src/api/site_commands.rs` | Agregar función para API/MCP |

#### Flujo del comando

```
diagnose::execute(config_path, site_name)
  → buscar sitio en config (kamples → uuid mo4so4440c488g8woow4cow0)
  → conectar SSH al VPS target del sitio
  → recolectar en paralelo:
      ├─ container_status()       → docker ps
      ├─ disk_usage()             → docker system df + du
      ├─ docker_volumes()         → volume ls + du per volume
      ├─ postgres_info()          → psql queries
      ├─ mysql_info()             → mysql queries
      ├─ compose_inspection()     → cat docker-compose.yml
      ├─ bind_mounts()            → ls + du
      ├─ container_logs()         → docker logs --tail 30
      └─ docker_inspect()         → docker inspect size
  → ensamblar DiagnoseReport
  → imprimir reporte formateado
  → [opcional] --json para output estructurado
```

#### Consideraciones técnicas

1. **SSH ya existe** via `src/infra/ssh_client.rs` (russh) y `src/infra/docker.rs`
2. **Múltiples comandos SSH** — se pueden agrupar en un solo `ssh.exec()` con `&&` o ejecutar en paralelo
3. **Timeout** por comando (30s por defecto, configurable)
4. **Fallos parciales** — si un comando falla (ej: no existe mariadb), se reporta como `unknown` en lugar de abortar
5. **JSON output** — `--json` para consumo por otras herramientas

---

## Fase 2: Ejecutar diagnóstico en kamples

Una vez implementado `diagnose`, ejecutar:

```bash
coolify-manager.exe diagnose --name kamples
```

Analizar el reporte para determinar:
- Si los datos reales están en los volúmenes Docker (aunque el dump sea pequeño)
- Si el problema es solo de conexión/configuración (no pérdida real)
- Si los 31.2 GB son principalmente uploads + datos recuperables

---

## Fase 3: Decidir estrategia de recuperación

Según los resultados del diagnóstico, aplicar una de estas estrategias:

### Escenario A: Datos existen en volúmenes Docker
- El contenedor app está caído pero postgres/mariadb tienen datos
- **Acción:** Hacer dump verdadero desde los contenedores en ejecución, luego decidir restore
- Herramienta: Extensión de `diagnose` con flag `--dump` que exporte BD reales

### Escenario B: Volúmenes Docker vacíos, bind mounts con archivos
- Las BD fueron limpiadas pero uploads/media sobreviven
- **Acción:** Recuperar archivos, intentar reconstruir BD desde backups antiguos
- Usar `dump` de los dumps existentes (aunque pequeños)

### Escenario C: Todo perdido excepto backups
- Los 31.2 GB son basura (overlay, imágenes Docker, logs)
- **Acción:** Backup final de archivos, limpieza, replantear desde backups reales

### Escenario D: El dump es pequeño porque pg_dump --format=custom comprime mucho
- 1.1 MB comprimido puede ser ~10-20 MB descomprimido
- Verificar cuántas tablas y registros hay realmente
- Si hay datos pero pocos, tal vez el dump capturó BD en estado limpio (post-failed-restore)

---

## Fase 4: Prevención (a futuro)

Independientemente del resultado:

1. **Backups automáticos funcionando** — verificar que `schedule-backup` está activo para kamples
2. **Bind mounts críticos documentados** — migrar a modelo con backup periódico de `/data/uploads/kamples`
3. **Restore procedure probado** — documentar pasos exactos de restore exitoso
4. **Monitoreo de estado** — alertas cuando un sitio pasa a `exited` (comando `health --alert`)
5. **Lección aprendida** — registrar en `Agente/lecciones/lecciones-aprendidas.md`

---

## Próximos pasos inmediatos

- [ ] **Fase 1:** Implementar comando `diagnose` en coolify-manager-rs
  - [ ] Crear tipos de dominio `DiagnoseReport` y subtipos
  - [ ] Crear `diagnose_service.rs` con recolectores SSH
  - [ ] Crear `commands/diagnose.rs` con formateo de output
  - [ ] Registrar en CLI dispatch
  - [ ] Compilar y verificar con `cargo build --release --target-dir target`
- [ ] **Fase 2:** Ejecutar `diagnose --name kamples` contra VPS Principal
- [ ] **Fase 3:** Analizar resultados y decidir estrategia
- [ ] **Fase 4:** Documentar y prevenir recurrencia

---

## Referencias

- **Backups disponibles:** `backups/kamples-20260427-045238.dump` (1.1 MB), `backups/glory_kamples-20260427-045238.dump` (252 KB)
- **SQL de truncado:** `backups/01_truncate_glory_kamples.sql` (limpiaba tablas scraper)
- **UUID Stack:** `mo4so4440c488g8woow4cow0`
- **Rama glorytemplate:** `main-kamples`
- **Settings en manager:** template `kamples`, backupPolicy enabled (dailyKeep=2, weeklyKeep=3)
- **Variables de entorno PostgreSQL:** `KAMPLES_PG_*` (host, port, dbname, user, password)
- **Arquitectura:** WordPress (MySQL) + Kamples (PostgreSQL) — dos BD independientes
- **Dumps actuales:** formato PostgreSQL custom (`pg_dump -Fc`), comprimidos, fechados 2026-04-27
