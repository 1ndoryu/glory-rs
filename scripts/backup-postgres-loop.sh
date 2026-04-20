#!/bin/sh
# [174A-114] Loop de backup ejecutado por el sidecar postgres-backup.
# - Hace pg_dump custom-format cada $BACKUP_INTERVAL_SECONDS.
# - Rota dumps más antiguos que $BACKUP_RETENTION_DAYS.
# - Falla loud (no silencia errores).
#
# Restore manual desde un dump:
#   docker compose exec postgres pg_restore --clean --if-exists \
#       -U $POSTGRES_USER -d $POSTGRES_DB /dumps/<archivo>.dump

set -eu

DUMP_DIR=/dumps
INTERVAL="${BACKUP_INTERVAL_SECONDS:-21600}"
RETENTION_DAYS="${BACKUP_RETENTION_DAYS:-14}"

mkdir -p "$DUMP_DIR"

echo "[backup] iniciando loop. intervalo=${INTERVAL}s retencion=${RETENTION_DAYS}d host=${PGHOST} db=${PGDATABASE}"

while true; do
    TS=$(date -u +%Y%m%dT%H%M%SZ)
    OUT="$DUMP_DIR/${PGDATABASE}-${TS}.dump"
    TMP="${OUT}.partial"

    echo "[backup] $(date -u -Iseconds) iniciando dump -> $OUT"

    if pg_dump --format=custom --no-owner --no-acl --file="$TMP" 2>&1; then
        mv "$TMP" "$OUT"
        SIZE=$(stat -c '%s' "$OUT" 2>/dev/null || stat -f '%z' "$OUT")
        echo "[backup] $(date -u -Iseconds) dump OK ${SIZE} bytes"
    else
        rm -f "$TMP"
        echo "[backup] $(date -u -Iseconds) ERROR en pg_dump (continuando, reintento en ${INTERVAL}s)" >&2
    fi

    # Rotación: borrar dumps anteriores a RETENTION_DAYS
    find "$DUMP_DIR" -maxdepth 1 -name "${PGDATABASE}-*.dump" -type f -mtime +${RETENTION_DAYS} -delete 2>/dev/null || true

    sleep "$INTERVAL"
done
