"""Procesa solo las filas pendientes del backfill 294A-6.

Uso:
    python scripts/process_backfill_294a6.py

Lee las filas desde cola_extraccion_backup_294a6, ejecuta el pipeline de
extraccion solo para esas entradas y dispara la publicacion incremental hacia
el backend Rust usando BACKEND_URL + KAMPLES_CRON_SECRET/SCRAPER_SECRET.
"""

from __future__ import annotations

import json
import sys
import tempfile
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
if str(ROOT) not in sys.path:
    sys.path.insert(0, str(ROOT))

from extractor import pipeline
from kamples_scraper.utils.db import get_connection


SQL = """
SELECT ce.id, ce.relacion_id, ce.youtube_id, ce.spotify_id,
       ce.timing_inicio_seg, ce.lado,
       rs.tipo_relacion, rs.tipo_elemento,
       rs.cancion_destino_id, rs.cancion_fuente_id,
       c_dest.titulo AS destino_titulo,
       a_dest.nombre AS destino_artista,
       c_fuente.titulo AS fuente_titulo,
       a_fuente.nombre AS fuente_artista,
       rs.votos_total
FROM cola_extraccion_backup_294a6 b
JOIN cola_extraccion_samples ce ON ce.id = b.id
JOIN relaciones_sample rs ON ce.relacion_id = rs.id
JOIN canciones c_dest ON rs.cancion_destino_id = c_dest.id
JOIN artistas_musicales a_dest ON c_dest.artista_id = a_dest.id
JOIN canciones c_fuente ON rs.cancion_fuente_id = c_fuente.id
JOIN artistas_musicales a_fuente ON c_fuente.artista_id = a_fuente.id
WHERE ce.estado = 'pendiente'
ORDER BY ce.id
"""


def cargar_items() -> list[dict]:
    conn = get_connection()
    try:
        with conn.cursor() as cur:
            cur.execute(SQL)
            columnas = [desc[0] for desc in cur.description]
            return [dict(zip(columnas, row)) for row in cur.fetchall()]
    finally:
        conn.close()


def main() -> None:
    items = cargar_items()
    print(json.dumps({"pendientes_backfill": len(items)}))

    exitosos = 0
    fallidos = 0
    motivos: dict[str, int] = {}

    with tempfile.TemporaryDirectory(prefix="kamples_audio_") as output_dir:
        for item in items:
            ok = pipeline.procesar_elemento(item, output_dir)
            if ok:
                exitosos += 1
                pipeline.notificar_publicacion(1)
            else:
                fallidos += 1
                motivo = item.get("_motivo_fallo", "desconocido")
                motivos[motivo] = motivos.get(motivo, 0) + 1

    print(
        json.dumps(
            {"exitosos": exitosos, "fallidos": fallidos, "motivos": motivos},
            ensure_ascii=False,
        )
    )


if __name__ == "__main__":
    main()