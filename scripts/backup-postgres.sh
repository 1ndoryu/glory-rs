#!/usr/bin/env bash
# [174A-114] Backup manual on-demand. Pensado para ejecutar ANTES de un
# deploy o de cualquier operación riesgosa contra producción.
#
# Uso (desde el host del servidor, dentro del directorio del stack Coolify):
#   ./scripts/backup-postgres.sh [etiqueta]
#
# El dump queda en /data/kamples/db_dumps/<db>-<ISO8601>-<etiqueta>.dump
# y NO se rota (el sidecar postgres-backup solo rota los suyos).

set -euo pipefail

LABEL="${1:-manual}"
DUMP_DIR=/data/kamples/db_dumps
TS=$(date -u +%Y%m%dT%H%M%SZ)

mkdir -p "$DUMP_DIR"

# Carga env del compose (POSTGRES_USER/POSTGRES_DB) si existe .env adyacente.
if [ -f ".env" ]; then
    # shellcheck disable=SC1091
    set -a; . ./.env; set +a
fi

: "${POSTGRES_USER:?POSTGRES_USER no definido}"
: "${POSTGRES_DB:?POSTGRES_DB no definido}"

OUT="$DUMP_DIR/${POSTGRES_DB}-${TS}-${LABEL}.dump"

echo "[backup-manual] generando $OUT"
docker compose exec -T postgres \
    pg_dump --format=custom --no-owner --no-acl \
        -U "$POSTGRES_USER" -d "$POSTGRES_DB" > "$OUT"

SIZE=$(stat -c '%s' "$OUT" 2>/dev/null || stat -f '%z' "$OUT")
echo "[backup-manual] OK $OUT (${SIZE} bytes)"
