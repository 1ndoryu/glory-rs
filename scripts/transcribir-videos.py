"""
Transcripcion de videos MP4 usando Groq Whisper API (whisper-large-v3).
Lee GROQ_API desde el archivo .env del proyecto automaticamente.
Uso:
  pip install groq python-dotenv
  python scripts/transcribir-videos.py

Los resultados se guardan en cliente/transcripciones-videos.md
"""

import os
import sys
from pathlib import Path

try:
    from groq import Groq
except ImportError:
    print("Error: falta 'groq'. Ejecuta: pip install groq python-dotenv")
    sys.exit(1)

try:
    from dotenv import load_dotenv
    load_dotenv(Path(__file__).parent.parent / ".env")
except ImportError:
    pass  # sin python-dotenv intentamos con variables de entorno directas

CARPETA_CLIENTE = Path(__file__).parent.parent / "cliente"
SALIDA = CARPETA_CLIENTE / "transcripciones-videos.md"

def transcribir(client: Groq, ruta_video: Path) -> str:
    print(f"  Transcribiendo: {ruta_video.name} ...")
    with open(ruta_video, "rb") as f:
        respuesta = client.audio.transcriptions.create(
            model="whisper-large-v3",
            file=(ruta_video.name, f, "video/mp4"),
            language="es",
            response_format="text",
        )
    return str(respuesta).strip()

def main() -> None:
    api_key = os.environ.get("GROQ_API")
    if not api_key:
        print("Error: no se encontro GROQ_API en el .env ni en variables de entorno")
        sys.exit(1)

    client = Groq(api_key=api_key)

    videos = sorted(CARPETA_CLIENTE.rglob("*.mp4"))
    if not videos:
        print(f"No se encontraron .mp4 en {CARPETA_CLIENTE}")
        sys.exit(1)

    print(f"Videos encontrados: {len(videos)}")
    lineas = ["# Transcripciones de Videos del Cliente\n\n"]

    for video in videos:
        texto = transcribir(client, video)
        bloque = f"## {video.name}\n\n{texto}\n\n---\n\n"
        lineas.append(bloque)
        print(f"  OK: {len(texto)} caracteres")

    SALIDA.write_text("".join(lineas), encoding="utf-8")
    print(f"\nGuardado en: {SALIDA}")

if __name__ == "__main__":
    main()
