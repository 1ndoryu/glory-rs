"""
Transcripcion de archivos multimedia de cliente/Data II usando Groq Whisper API.
Procesa .mp4 y .ogg. Resultados en cliente/Data II/transcripciones-data-ii.md
Soporta reanudacion: si el archivo de salida ya existe, salta los ya transcritos.
Videos > 20MB se convierten a audio con ffmpeg antes de enviar.
"""

import os
import sys
import time
import subprocess
import tempfile
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
    pass

CARPETA = Path(__file__).parent.parent / "cliente" / "Data II"
SALIDA = CARPETA / "transcripciones-data-ii.md"

MIME_TYPES = {
    ".mp4": "video/mp4",
    ".ogg": "audio/ogg",
}

MAX_RETRIES = 5
RETRY_DELAY_SECS = 15
MAX_SIZE_BYTES = 20 * 1024 * 1024  # 20MB

def extraer_audio(ruta_video):
    """Extrae audio de video con ffmpeg, retorna path temporal .ogg"""
    tmp = tempfile.NamedTemporaryFile(suffix=".ogg", delete=False)
    tmp.close()
    cmd = [
        "ffmpeg", "-y", "-i", str(ruta_video),
        "-vn", "-acodec", "libopus", "-b:a", "48k",
        tmp.name
    ]
    subprocess.run(cmd, capture_output=True, check=True)
    return Path(tmp.name)

def archivos_ya_transcritos():
    """Lee la salida existente y extrae nombres de archivos ya procesados."""
    if not SALIDA.exists():
        return set()
    contenido = SALIDA.read_text(encoding="utf-8")
    ya = set()
    for linea in contenido.splitlines():
        if linea.startswith("## ") and " — " in linea:
            nombre = linea.split(" — ", 1)[1].strip()
            ya.add(nombre)
    return ya

def transcribir(ruta, api_keys, key_index):
    """Transcribe un archivo con retries y rotacion de API keys."""
    archivo_a_enviar = ruta
    tmp_path = None
    
    if ruta.stat().st_size > MAX_SIZE_BYTES and ruta.suffix.lower() == ".mp4":
        print(f"    Archivo grande ({ruta.stat().st_size // 1024 // 1024}MB), extrayendo audio con ffmpeg...")
        tmp_path = extraer_audio(ruta)
        archivo_a_enviar = tmp_path
        print(f"    Audio extraido: {tmp_path.stat().st_size // 1024}KB")
    
    ext = archivo_a_enviar.suffix.lower()
    mime = MIME_TYPES.get(ext, "audio/ogg")
    print(f"  Transcribiendo: {ruta.name} ...")
    
    try:
        for retry in range(MAX_RETRIES):
            for intento in range(len(api_keys)):
                idx = (key_index[0] + intento) % len(api_keys)
                client = Groq(api_key=api_keys[idx])
                try:
                    with open(archivo_a_enviar, "rb") as f:
                        respuesta = client.audio.transcriptions.create(
                            model="whisper-large-v3",
                            file=(archivo_a_enviar.name, f, mime),
                            language="es",
                            response_format="text",
                        )
                    key_index[0] = (idx + 1) % len(api_keys)
                    return str(respuesta).strip()
                except Exception as e:
                    error_str = str(e).lower()
                    if "rate_limit" in error_str or "429" in error_str:
                        print(f"    Rate limit en key {idx+1}, rotando...")
                        continue
                    print(f"    Error ({type(e).__name__}), intento {retry+1}/{MAX_RETRIES}, esperando {RETRY_DELAY_SECS}s...")
                    time.sleep(RETRY_DELAY_SECS)
                    break
        
        return f"[ERROR: no se pudo transcribir {ruta.name} despues de {MAX_RETRIES} intentos]"
    finally:
        if tmp_path and tmp_path.exists():
            tmp_path.unlink()

def main():
    api_keys = []
    for var in ["GROQ_API", "GROQ_API_1", "GROQ_API_2", "GROQ_API_3"]:
        val = os.environ.get(var)
        if val:
            api_keys.append(val)
    
    seen = []
    unique_keys = []
    for k in api_keys:
        if k not in seen:
            seen.append(k)
            unique_keys.append(k)
    api_keys = unique_keys
    
    if not api_keys:
        print("Error: no se encontro GROQ_API en .env")
        sys.exit(1)
    
    print(f"API keys disponibles: {len(api_keys)}")
    
    archivos = sorted(
        [f for f in CARPETA.iterdir() if f.suffix.lower() in MIME_TYPES]
    )
    
    if not archivos:
        print(f"No se encontraron archivos multimedia en {CARPETA}")
        sys.exit(1)
    
    ya_hechos = archivos_ya_transcritos()
    pendientes = [f for f in archivos if f.name not in ya_hechos]
    
    print(f"Archivos totales: {len(archivos)}, ya transcritos: {len(ya_hechos)}, pendientes: {len(pendientes)}")
    
    if not pendientes:
        print("Todos los archivos ya estan transcritos.")
        return
    
    es_nuevo = not SALIDA.exists() or len(ya_hechos) == 0
    
    if es_nuevo:
        with open(SALIDA, "w", encoding="utf-8") as f:
            f.write("# Transcripciones Data II\n\n")
            f.write("> Transcritos con Groq Whisper large-v3.\n\n")
    
    key_index = [0]
    total = len(archivos)
    
    for archivo in pendientes:
        i = archivos.index(archivo) + 1
        tipo = "Audio" if archivo.suffix.lower() == ".ogg" else "Video"
        texto = transcribir(archivo, api_keys, key_index)
        bloque = f"## {tipo} {i} — {archivo.name}\n\n{texto}\n\n---\n\n"
        
        with open(SALIDA, "a", encoding="utf-8") as f:
            f.write(bloque)
        
        print(f"  OK ({i}/{total}): {len(texto)} caracteres")
    
    print(f"\nGuardado en: {SALIDA}")

if __name__ == "__main__":
    main()
