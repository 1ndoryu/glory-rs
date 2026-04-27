"""
[274A-1] Backfill de imagenes externas a mirror local.

Recorre canciones y artistas_musicales cuyo imagen_url apunta al CDN externo
de WhoSampled (https://www.whosampled.com/...) y las descarga al directorio
configurado por IMAGES_STORE_PATH, dejando IMAGES_BASE_URL/<hash>.<ext> en BD.

Uso:
    cd clients/kamples-scraper
    python scripts/backfill_imagenes.py [--limite N] [--solo canciones|artistas]

Reutiliza la sesion + proxy + headers de ImageDescargaPipeline para mantener
paridad con el flujo del scraper.
"""

import argparse
import hashlib
import logging
import os
import sys
import time
from pathlib import Path
from urllib.parse import urlparse

import psycopg2
from curl_cffi import requests as curl_requests
from dotenv import load_dotenv

# Cargar .env del scraper (desde el dir actual o el del repo).
load_dotenv()

# Hacer importable el paquete kamples_scraper (uso solo para utils/db si hace falta).
PKG_ROOT = Path(__file__).resolve().parents[1]
if str(PKG_ROOT) not in sys.path:
    sys.path.insert(0, str(PKG_ROOT))

from kamples_scraper.utils.db import get_connection  # noqa: E402

logging.basicConfig(
    level=logging.INFO,
    format="%(asctime)s | %(levelname)s | %(message)s",
)
logger = logging.getLogger("backfill_imagenes")

EXTENSIONES_VALIDAS = (".jpg", ".jpeg", ".png", ".webp", ".gif")
HEADERS = {
    "User-Agent": (
        "Mozilla/5.0 (Windows NT 10.0; Win64; x64) "
        "AppleWebKit/537.36 (KHTML, like Gecko) Chrome/125.0.0.0 Safari/537.36"
    ),
    "Accept": "image/avif,image/webp,image/apng,image/*,*/*;q=0.8",
    "Referer": "https://www.whosampled.com/",
}
MAX_REINTENTOS = 3
ESPERA_BASE_SEG = 2


def construir_proxies():
    user = os.getenv("PROXY_USER") or os.getenv("DATAIMPULSE_LOGIN")
    pwd = os.getenv("PROXY_PASSWORD") or os.getenv("DATAIMPULSE_PASSWORD")
    host = os.getenv("PROXY_HOST") or os.getenv("DATAIMPULSE_HOST")
    port = os.getenv("PROXY_PORT") or os.getenv("DATAIMPULSE_PORT")
    if not (user and pwd and host and port):
        logger.warning("Proxy no configurado, descargando directo (puede fallar por hotlink).")
        return None
    url = f"http://{user}:{pwd}@{host}:{port}"
    return {"http": url, "https": url}


def descargar(session, proxies, store_path: Path, base_url: str, url: str) -> str | None:
    nombre = hashlib.sha256(url.encode()).hexdigest()[:40]
    ext = Path(urlparse(url).path).suffix.lower()
    if ext not in EXTENSIONES_VALIDAS:
        ext = ".jpg"

    archivo = store_path / f"{nombre}{ext}"
    url_local = f"{base_url}/{nombre}{ext}"

    if archivo.exists():
        return url_local

    for intento in range(1, MAX_REINTENTOS + 1):
        try:
            resp = session.get(url, proxies=proxies, timeout=20)
            resp.raise_for_status()
            content_type = resp.headers.get("content-type", "")
            if "text/html" in content_type or len(resp.content) < 100:
                logger.warning(
                    "Imagen sospechosa ct=%s size=%d url=%s intento=%d",
                    content_type, len(resp.content), url, intento,
                )
                if intento < MAX_REINTENTOS:
                    time.sleep(ESPERA_BASE_SEG * intento)
                    continue
                return None
            archivo.write_bytes(resp.content)
            return url_local
        except Exception as exc:
            logger.warning("Fallo descarga %s intento=%d: %s", url, intento, exc)
            if intento < MAX_REINTENTOS:
                time.sleep(ESPERA_BASE_SEG * intento)
    return None


def es_externa(imagen_url: str | None) -> bool:
    if not imagen_url:
        return False
    return "whosampled.com" in imagen_url and imagen_url.startswith("http")


def procesar_tabla(
    conn,
    session,
    proxies,
    store_path: Path,
    base_url: str,
    tabla: str,
    limite: int | None,
):
    sql_select = f"SELECT id, imagen_url FROM {tabla} WHERE imagen_url LIKE 'http%whosampled%' ORDER BY id"
    if limite:
        sql_select += f" LIMIT {int(limite)}"

    with conn.cursor() as cur:
        cur.execute(sql_select)
        filas = cur.fetchall()

    total = len(filas)
    logger.info("[%s] %d filas con imagen externa", tabla, total)
    if total == 0:
        return

    ok, fallos = 0, 0
    for idx, (fila_id, url_externa) in enumerate(filas, 1):
        url_local = descargar(session, proxies, store_path, base_url, url_externa)
        with conn.cursor() as cur:
            if url_local:
                cur.execute(
                    f"UPDATE {tabla} SET imagen_url = %s WHERE id = %s",
                    (url_local, fila_id),
                )
                ok += 1
            else:
                cur.execute(
                    f"UPDATE {tabla} SET imagen_url = NULL WHERE id = %s",
                    (fila_id,),
                )
                fallos += 1
        # Commit por chunks para no perder progreso en caidas
        if idx % 25 == 0:
            conn.commit()
            logger.info("[%s] %d/%d (ok=%d fallos=%d)", tabla, idx, total, ok, fallos)
    conn.commit()
    logger.info("[%s] FIN ok=%d fallos=%d", tabla, ok, fallos)


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--limite", type=int, default=None, help="Limitar filas por tabla")
    parser.add_argument(
        "--solo",
        choices=("canciones", "artistas"),
        default=None,
        help="Procesar solo una tabla",
    )
    args = parser.parse_args()

    store_path_raw = os.getenv("IMAGES_STORE_PATH")
    base_url = (os.getenv("IMAGES_BASE_URL") or "").rstrip("/")
    if not store_path_raw or not base_url:
        logger.error("IMAGES_STORE_PATH y IMAGES_BASE_URL deben estar definidos en .env")
        sys.exit(2)

    store_path = Path(store_path_raw)
    store_path.mkdir(parents=True, exist_ok=True)
    logger.info("Mirror local: %s -> %s", store_path, base_url)

    proxies = construir_proxies()
    session = curl_requests.Session(impersonate="chrome124")
    session.headers.update(HEADERS)

    conn = get_connection()
    conn.autocommit = False
    try:
        if args.solo in (None, "canciones"):
            procesar_tabla(conn, session, proxies, store_path, base_url, "canciones", args.limite)
        if args.solo in (None, "artistas"):
            procesar_tabla(conn, session, proxies, store_path, base_url, "artistas_musicales", args.limite)
    finally:
        session.close()
        conn.close()


if __name__ == "__main__":
    main()
