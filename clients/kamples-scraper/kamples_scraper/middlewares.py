"""
Kamples Scraper — Middlewares.

CurlCffiDownloaderMiddleware: usa curl_cffi para TLS fingerprinting (bypass Cloudflare).
BandwidthTrackerMiddleware: registra bytes consumidos y alerta al 80%.
"""

import logging

from scrapy import signals
from scrapy.http import HtmlResponse
from twisted.internet import threads

from curl_cffi import requests as curl_requests

logger = logging.getLogger(__name__)

WARMUP_URL = "https://www.whosampled.com/"


class CurlCffiDownloaderMiddleware:
    """
    Middleware que reemplaza el downloader HTTP estandar de Scrapy
    con curl_cffi para emular TLS fingerprint de Chrome.

    Resuelve el bloqueo de Cloudflare (403 "Just a moment...").
    Lee proxy de settings; si no hay proxy, request directo.
    Realiza warm-up automatico contra WhoSampled para establecer
    cookies de sesion antes de que cualquier spider haga requests.
    """

    @classmethod
    def from_crawler(cls, crawler):
        middleware = cls()
        host = crawler.settings.get("PROXY_HOST", "")
        port = crawler.settings.get("PROXY_PORT", "823")
        user = crawler.settings.get("PROXY_USER", "")
        password = crawler.settings.get("PROXY_PASSWORD", "")

        if user and password and host:
            proxy_url = f"http://{user}:{password}@{host}:{port}"
            middleware.proxies = {"https": proxy_url, "http": proxy_url}
        else:
            middleware.proxies = None

        # Sesion por defecto (puede ser reemplazada por _warmup si obtiene 200)
        middleware.session = curl_requests.Session(impersonate="chrome")
        crawler._curl_session = middleware.session
        crawler._curl_proxies = middleware.proxies
        middleware._crawler = crawler
        middleware._warmup_ok = False
        middleware._warmup(crawler)
        return middleware

    # Numero maximo de intentos de warm-up (cada intento usa una nueva IP del proxy)
    WARMUP_MAX_ATTEMPTS = 5

    def _warmup(self, crawler):
        """
        Request inicial a la homepage para establecer cookies de sesion
        y pasar el challenge de Cloudflare antes del scraping real.

        Reintenta hasta WARMUP_MAX_ATTEMPTS veces con nuevas sesiones (= nuevas IPs
        del proxy rotativo DataImpulse) hasta obtener HTTP 200 limpio.
        Solo cuando obtiene 200 fija self.session para que los requests del spider
        reutilicen la misma conexion (y por tanto la misma IP del proxy).
        """
        for attempt in range(1, self.WARMUP_MAX_ATTEMPTS + 1):
            session_candidate = curl_requests.Session(impersonate="chrome")
            try:
                resp = session_candidate.get(
                    WARMUP_URL,
                    proxies=self.proxies,
                    timeout=30,
                    allow_redirects=True,
                )
                if resp.status_code == 200:
                    self.session = session_candidate
                    crawler._curl_session = self.session
                    crawler._curl_proxies = self.proxies
                    self._warmup_ok = True
                    logger.info(
                        "Warm-up OK (intento %d/%d): status=200, cookies=%d",
                        attempt, self.WARMUP_MAX_ATTEMPTS,
                        len(self.session.cookies),
                    )
                    return
                else:
                    logger.warning(
                        "Warm-up intento %d/%d: status=%d (posible CF challenge, "
                        "probando nueva IP...)",
                        attempt, self.WARMUP_MAX_ATTEMPTS, resp.status_code,
                    )
            except Exception as exc:
                # [294A-3] Detectar 407 — credenciales proxy invalidas, no reintentar
                err_str = str(exc)
                if "407" in err_str or "ProxyError" in type(exc).__name__:
                    logger.error(
                        "Proxy auth 407 — credenciales DataImpulse invalidas o expiradas. "
                        "Regenerar en https://dataimpulse.com. Error: %s",
                        exc,
                    )
                    return  # 407 no se resuelve reintentando
                logger.warning(
                    "Warm-up intento %d/%d fallo: %s",
                    attempt, self.WARMUP_MAX_ATTEMPTS, exc,
                )

        # Todos los intentos fallaron — usar session de ultimo intento de todas formas
        logger.error(
            "Warm-up fallo %d intentos. El spider puede no obtener contenido de las listas. "
            "Verifique la calidad del pool de IPs en DataImpulse.",
            self.WARMUP_MAX_ATTEMPTS,
        )
        # self.session queda como fue creado en from_crawler (sin inicializar a warmup)
        # Los requests del spider seguiran pero probablemente obtendran paginas vacias

    def process_request(self, request):
        """
        Intercepta cada request, lo ejecuta con curl_cffi y retorna
        un HtmlResponse directamente (Scrapy no usa su downloader).
        Ejecutado en thread para no bloquear el reactor Twisted.
        """
        d = threads.deferToThread(self._fetch, request)
        return d

    def _fetch(self, request):
        """
        Ejecutar request con curl_cffi (sync, en thread separado).

        Si recibe 403 (Cloudflare bloquea la IP actual del proxy rotativo),
        re-ejecuta el warm-up para conseguir una nueva IP limpia y reintenta
        hasta REQUEST_MAX_ATTEMPTS veces. Esto es necesario porque DataImpulse
        rota IPs por CONNECT tunnel, y Cloudflare bloquea cada nueva IP que
        aparezca sin la cookie de sesion correspondiente.
        """
        headers = dict(request.headers.to_unicode_dict())
        if "Referer" not in headers:
            headers["Referer"] = "https://www.whosampled.com/"

        last_resp = None
        for attempt in range(1, self.REQUEST_MAX_ATTEMPTS + 1):
            try:
                resp = self.session.get(
                    request.url,
                    headers=headers,
                    proxies=self.proxies,
                    timeout=30,
                    allow_redirects=True,
                )
                last_resp = resp
            except Exception:
                logger.exception(
                    "curl_cffi error fetching %s (intento %d/%d)",
                    request.url, attempt, self.REQUEST_MAX_ATTEMPTS,
                )
                if attempt == self.REQUEST_MAX_ATTEMPTS:
                    raise
                # Reintentar en proxima iteracion (nueva IP probable)
                self._refresh_session()
                continue

            if resp.status_code != 403:
                # Respuesta valida (200, 404, 5xx, etc.) — devolver
                resp_headers = dict(resp.headers)
                resp_headers.pop("Content-Encoding", None)
                resp_headers.pop("content-encoding", None)
                return HtmlResponse(
                    url=str(resp.url),
                    status=resp.status_code,
                    headers=resp_headers,
                    body=resp.content,
                    request=request,
                )

            # 403: IP bloqueada por Cloudflare. Refrescar sesion (nueva IP) y reintentar.
            logger.warning(
                "403 en %s (intento %d/%d) — refrescando sesion para nueva IP",
                request.url, attempt, self.REQUEST_MAX_ATTEMPTS,
            )
            self._refresh_session()

        # Agotamos reintentos — devolver el ultimo 403
        resp_headers = dict(last_resp.headers)
        resp_headers.pop("Content-Encoding", None)
        resp_headers.pop("content-encoding", None)
        return HtmlResponse(
            url=str(last_resp.url),
            status=last_resp.status_code,
            headers=resp_headers,
            body=last_resp.content,
            request=request,
        )

    # Reintentos por request individual cuando hay 403 (cada uno fuerza nueva IP)
    REQUEST_MAX_ATTEMPTS = 4

    def _refresh_session(self):
        """
        Crear sesion fresca curl_cffi (= nueva IP del proxy rotativo) y hacer
        warm-up para obtener cookie __cf_bm valida. Reemplaza self.session
        si el warm-up devuelve 200.
        """
        for attempt in range(1, self.WARMUP_MAX_ATTEMPTS + 1):
            session_candidate = curl_requests.Session(impersonate="chrome")
            try:
                resp = session_candidate.get(
                    WARMUP_URL,
                    proxies=self.proxies,
                    timeout=30,
                    allow_redirects=True,
                )
                if resp.status_code == 200:
                    self.session = session_candidate
                    if hasattr(self, "_crawler"):
                        self._crawler._curl_session = self.session
                    logger.info(
                        "Sesion refrescada OK (intento %d/%d): cookies=%d",
                        attempt, self.WARMUP_MAX_ATTEMPTS,
                        len(self.session.cookies),
                    )
                    return True
            except Exception as exc:
                err_str = str(exc)
                if "407" in err_str or "ProxyError" in type(exc).__name__:
                    logger.error("Refresh fallo: proxy 407 (credenciales). Abort.")
                    return False
        logger.warning(
            "Refresh de sesion fallo %d intentos — manteniendo sesion anterior",
            self.WARMUP_MAX_ATTEMPTS,
        )
        return False


class BandwidthTrackerMiddleware:
    """
    Rastrea bytes descargados y alerta al alcanzar 80% del presupuesto.
    Cierra el spider si se excede el presupuesto.
    """

    def __init__(self):
        self.total_bytes = 0
        self.budget_bytes = 0
        self.alerted_80 = False
        self.crawler = None

    @classmethod
    def from_crawler(cls, crawler):
        middleware = cls()
        middleware.budget_bytes = crawler.settings.getint("PROXY_BUDGET_BYTES", 5368709120)
        middleware.crawler = crawler
        crawler.signals.connect(middleware.spider_closed, signal=signals.spider_closed)
        return middleware

    def process_response(self, request, response, spider):
        body_size = len(response.body) if response.body else 0
        self.total_bytes += body_size

        if not self.alerted_80 and self.total_bytes >= self.budget_bytes * 0.8:
            self.alerted_80 = True
            logger.warning(
                "ALERTA: 80%% del presupuesto de proxy consumido. "
                "Usado: %.2f MB de %.2f MB",
                self.total_bytes / (1024 * 1024),
                self.budget_bytes / (1024 * 1024),
            )

        if self.total_bytes >= self.budget_bytes:
            logger.error(
                "PRESUPUESTO EXCEDIDO: %.2f MB. Cerrando spider.",
                self.total_bytes / (1024 * 1024),
            )
            spider.crawler.engine.close_spider(spider, "budget_exceeded")

        return response

    def spider_closed(self, spider, reason):
        logger.info(
            "Bandwidth total consumido: %.2f MB (%d bytes)",
            self.total_bytes / (1024 * 1024),
            self.total_bytes,
        )
