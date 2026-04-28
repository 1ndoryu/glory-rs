use serde::Deserialize;
use std::env;
use std::fs::File;
use std::io::ErrorKind;
use std::path::{Path, PathBuf};
use std::time::Duration;
use symphonia::core::audio::SampleBuffer;
use symphonia::core::codecs::{Decoder, DecoderOptions};
use symphonia::core::errors::Error as SymphoniaError;
use symphonia::core::formats::{FormatOptions, FormatReader};
use symphonia::core::io::{MediaSourceStream, MediaSourceStreamOptions};
use symphonia::core::meta::MetadataOptions;
use symphonia::core::probe::Hint;
use thiserror::Error;
use tokio::process::Command;
use uuid::Uuid;

#[path = "ffmpeg/detection.rs"]
mod detection;

pub const DEFAULT_WAVEFORM_BARS: usize = 120;
const WAVEFORM_SAMPLE_RATE: u32 = 8_000;
const MP3_BITRATE_KBPS: u32 = 320;

/* [174A-30] Capa híbrida de audio para Fase 4.
 * Usa ffprobe/ffmpeg CLI cuando están disponibles y cae en Symphonia para
 * mantener el pipeline operativo en desarrollo o CI sin dependencias C.
 * Gotcha: la conversión a MP3/FLAC sí requiere ffmpeg; metadata y waveform
 * pueden resolverse con Symphonia como fallback determinista. */

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct FFmpegBinaries {
    ffmpeg: Option<PathBuf>,
    ffprobe: Option<PathBuf>,
}

impl FFmpegBinaries {
    #[must_use]
    pub fn new(ffmpeg: Option<PathBuf>, ffprobe: Option<PathBuf>) -> Self {
        Self { ffmpeg, ffprobe }
    }

    #[must_use]
    pub fn detect() -> Self {
        Self {
            ffmpeg: detection::detect_binary("ffmpeg"),
            ffprobe: detection::detect_binary("ffprobe"),
        }
    }

    #[must_use]
    pub fn ffmpeg(&self) -> Option<&Path> {
        self.ffmpeg.as_deref()
    }

    #[must_use]
    pub fn ffprobe(&self) -> Option<&Path> {
        self.ffprobe.as_deref()
    }

    #[must_use]
    pub fn has_ffmpeg(&self) -> bool {
        self.ffmpeg.is_some()
    }

    #[must_use]
    pub fn has_ffprobe(&self) -> bool {
        self.ffprobe.is_some()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct AudioMetadata {
    pub format: String,
    pub duration_seconds: f32,
    pub sample_rate_hz: u32,
    pub channels: u16,
    pub file_size_bytes: u64,
    pub waveform_peaks: Vec<f32>,
}

#[derive(Debug, Error)]
pub enum AudioError {
    #[error("No se encontró el archivo de audio: {0}")]
    MissingFile(PathBuf),
    #[error("No se encontró ffmpeg para la operación solicitada")]
    FfmpegNotFound,
    #[error("No se encontró una pista de audio por defecto en {0}")]
    MissingDefaultTrack(PathBuf),
    #[error("La pista de audio no expone sample_rate en {0}")]
    MissingSampleRate(PathBuf),
    #[error("La pista de audio no expone canales en {0}")]
    MissingChannels(PathBuf),
    #[error("No fue posible detectar el formato del audio en {0}")]
    MissingFormat(PathBuf),
    #[error("{tool} falló con código {status:?}: {stderr}")]
    ExternalToolFailed {
        tool: String,
        status: Option<i32>,
        stderr: String,
    },
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    Json(#[from] serde_json::Error),
    #[error("Symphonia devolvió un error: {0}")]
    Symphonia(String),
}

impl From<SymphoniaError> for AudioError {
    fn from(value: SymphoniaError) -> Self {
        Self::Symphonia(value.to_string())
    }
}

#[derive(Debug)]
struct SymphoniaProbe {
    sample_rate_hz: u32,
    channels: u16,
    duration_seconds: f32,
}

struct AudioReader {
    format: Box<dyn FormatReader>,
    decoder: Box<dyn Decoder>,
    track_id: u32,
    sample_rate_hz: u32,
    channels: usize,
    estimated_frames: Option<u64>,
}

#[derive(Debug, Deserialize)]
struct FFprobeOutput {
    format: Option<FFprobeFormat>,
}

#[derive(Debug, Deserialize)]
struct FFprobeFormat {
    duration: Option<String>,
}

pub async fn inspect_audio_file(
    input_path: &Path,
    format_hint: Option<&str>,
    binaries: Option<&FFmpegBinaries>,
) -> Result<AudioMetadata, AudioError> {
    ensure_input_exists(input_path)?;

    let binaries = binaries.cloned().unwrap_or_else(FFmpegBinaries::detect);
    let format = detect_audio_format(input_path, format_hint)?;
    let file_size_bytes = std::fs::metadata(input_path)?.len();
    let mut symphonia_probe = probe_with_symphonia(input_path, format_hint)?;

    if let Some(ffprobe_path) = binaries.ffprobe() {
        match probe_duration_with_ffprobe(ffprobe_path, input_path).await {
            Ok(duration_seconds) if duration_seconds > 0.0 => {
                symphonia_probe.duration_seconds = duration_seconds;
            }
            Ok(_) => {}
            Err(error) => {
                tracing::warn!(path = %input_path.display(), ?error, "ffprobe no pudo calcular duración; se usa Symphonia");
            }
        }
    }

    let waveform_peaks = if let Some(ffmpeg_path) = binaries.ffmpeg() {
        match waveform_peaks_with_ffmpeg(ffmpeg_path, input_path, DEFAULT_WAVEFORM_BARS).await {
            Ok(peaks) => peaks,
            Err(error) => {
                tracing::warn!(path = %input_path.display(), ?error, "ffmpeg no pudo generar waveform; se usa Symphonia");
                waveform_peaks_with_symphonia(input_path, format_hint, DEFAULT_WAVEFORM_BARS)?
            }
        }
    } else {
        waveform_peaks_with_symphonia(input_path, format_hint, DEFAULT_WAVEFORM_BARS)?
    };

    Ok(AudioMetadata {
        format,
        duration_seconds: symphonia_probe.duration_seconds,
        sample_rate_hz: symphonia_probe.sample_rate_hz,
        channels: symphonia_probe.channels,
        file_size_bytes,
        waveform_peaks,
    })
}

pub async fn convert_to_mp3(
    input_path: &Path,
    output_path: &Path,
    binaries: Option<&FFmpegBinaries>,
) -> Result<(), AudioError> {
    ensure_input_exists(input_path)?;

    let binaries = binaries.cloned().unwrap_or_else(FFmpegBinaries::detect);
    let ffmpeg_path = binaries.ffmpeg().ok_or(AudioError::FfmpegNotFound)?;

    run_ffmpeg(
        ffmpeg_path,
        input_path,
        output_path,
        &[
            "-codec:a",
            "libmp3lame",
            "-b:a",
            &format!("{MP3_BITRATE_KBPS}k"),
            "-ar",
            "44100",
        ],
    )
    .await
}

pub async fn convert_to_flac(
    input_path: &Path,
    output_path: &Path,
    binaries: Option<&FFmpegBinaries>,
) -> Result<(), AudioError> {
    ensure_input_exists(input_path)?;

    let binaries = binaries.cloned().unwrap_or_else(FFmpegBinaries::detect);
    let ffmpeg_path = binaries.ffmpeg().ok_or(AudioError::FfmpegNotFound)?;

    run_ffmpeg(ffmpeg_path, input_path, output_path, &["-codec:a", "flac"]).await
}

/* [254A-8c] Recorta un rango [start_sec, start_sec + duration_sec] del audio
 * fuente y lo convierte a MP3 320 kbps mono.
 *
 * `-ss` ANTES de `-i` es la forma rápida (seek antes de decode). Para audio
 * completo descargado esto es correcto y mucho más rápido que leer todo.
 * `-t` fija la duración máxima de salida.
 *
 * Gotcha: si start_sec + duration_sec > duración real del audio, FFmpeg
 * simplemente corta al final sin error — comportamiento deseado. */
pub async fn cut_to_mp3(
    input_path: &Path,
    output_path: &Path,
    start_sec: f64,
    duration_sec: f64,
    binaries: Option<&FFmpegBinaries>,
) -> Result<(), AudioError> {
    ensure_input_exists(input_path)?;

    let binaries = binaries.cloned().unwrap_or_else(FFmpegBinaries::detect);
    let ffmpeg_path = binaries.ffmpeg().ok_or(AudioError::FfmpegNotFound)?;

    let start_str = format!("{start_sec:.6}");
    let duration_str = format!("{duration_sec:.6}");

    // -ss antes de -i = fast seek; los extra_args se insertan DESPUÉS de -i
    // Para eso usamos run_ffmpeg_with_input_seek que pasa -ss al principio
    run_ffmpeg_seek(
        ffmpeg_path,
        input_path,
        output_path,
        &start_str,
        &duration_str,
    )
    .await
}

async fn run_ffmpeg_seek(
    ffmpeg_path: &Path,
    input_path: &Path,
    output_path: &Path,
    start_sec: &str,
    duration_sec: &str,
) -> Result<(), AudioError> {
    if let Some(parent) = output_path.parent() {
        std::fs::create_dir_all(parent)?;
    }

    let args = vec![
        "-y".to_owned(),
        "-ss".to_owned(),
        start_sec.to_owned(),
        "-i".to_owned(),
        input_path.to_string_lossy().into_owned(),
        "-t".to_owned(),
        duration_sec.to_owned(),
        "-codec:a".to_owned(),
        "libmp3lame".to_owned(),
        "-b:a".to_owned(),
        format!("{MP3_BITRATE_KBPS}k"),
        "-ar".to_owned(),
        "44100".to_owned(),
        output_path.to_string_lossy().into_owned(),
    ];

    let output = Command::new(ffmpeg_path).args(&args).output().await?;

    if output.status.success() && output_path.is_file() {
        return Ok(());
    }

    Err(AudioError::ExternalToolFailed {
        tool: ffmpeg_path.display().to_string(),
        status: output.status.code(),
        stderr: String::from_utf8_lossy(&output.stderr).trim().to_owned(),
    })
}

fn ensure_input_exists(input_path: &Path) -> Result<(), AudioError> {
    if input_path.is_file() {
        return Ok(());
    }

    Err(AudioError::MissingFile(input_path.to_path_buf()))
}

fn detect_audio_format(input_path: &Path, format_hint: Option<&str>) -> Result<String, AudioError> {
    format_hint
        .and_then(normalize_audio_format)
        .or_else(|| {
            input_path
                .extension()
                .and_then(|value| value.to_str())
                .and_then(normalize_audio_format)
        })
        .or_else(|| {
            mime_guess::from_path(input_path)
                .first_raw()
                .and_then(normalize_audio_format)
        })
        .ok_or_else(|| AudioError::MissingFormat(input_path.to_path_buf()))
}

fn normalize_audio_format(value: &str) -> Option<String> {
    let normalized = value.trim().to_ascii_lowercase();
    let normalized = normalized.strip_prefix("audio/").unwrap_or(&normalized);

    match normalized {
        "wav" | "wave" | "x-wav" | "vnd.wave" => Some("wav".to_owned()),
        "mp3" | "mpeg" => Some("mp3".to_owned()),
        "flac" | "x-flac" => Some("flac".to_owned()),
        "ogg" | "oga" | "vorbis" | "opus" => Some("ogg".to_owned()),
        "aif" | "aiff" => Some("aiff".to_owned()),
        _ => None,
    }
}

fn probe_with_symphonia(
    input_path: &Path,
    format_hint: Option<&str>,
) -> Result<SymphoniaProbe, AudioError> {
    let mut reader = open_audio_reader(input_path, format_hint)?;
    let total_frames = reader
        .estimated_frames
        .unwrap_or(count_frames(&mut reader)?);

    let duration_seconds = if total_frames > 0 {
        duration_from_frames(total_frames, reader.sample_rate_hz).as_secs_f32()
    } else {
        0.0
    };

    Ok(SymphoniaProbe {
        sample_rate_hz: reader.sample_rate_hz,
        channels: u16::try_from(reader.channels).unwrap_or(u16::MAX),
        duration_seconds,
    })
}

fn waveform_peaks_with_symphonia(
    input_path: &Path,
    format_hint: Option<&str>,
    bars: usize,
) -> Result<Vec<f32>, AudioError> {
    let mut reader = open_audio_reader(input_path, format_hint)?;
    let total_frames = reader
        .estimated_frames
        .unwrap_or(count_frames(&mut reader)?);

    if total_frames == 0 || bars == 0 {
        return Ok(vec![0.0; bars]);
    }

    let mut reader = open_audio_reader(input_path, format_hint)?;
    let frames_per_bar = total_frames.div_ceil(u64::try_from(bars).unwrap_or(1));
    let mut peaks = vec![0.0_f32; bars];
    let mut frame_index = 0_u64;
    let mut sample_buffer: Option<SampleBuffer<f32>> = None;

    loop {
        let packet = match reader.format.next_packet() {
            Ok(packet) => packet,
            Err(SymphoniaError::IoError(error)) if error.kind() == ErrorKind::UnexpectedEof => {
                break
            }
            Err(SymphoniaError::ResetRequired) => {
                return Err(AudioError::Symphonia(
                    "Symphonia pidió reset del decoder".to_owned(),
                ));
            }
            Err(error) => return Err(error.into()),
        };

        if packet.track_id() != reader.track_id {
            continue;
        }

        match reader.decoder.decode(&packet) {
            Ok(decoded) => {
                let buffer = sample_buffer.get_or_insert_with(|| {
                    SampleBuffer::<f32>::new(decoded.capacity() as u64, *decoded.spec())
                });

                if buffer.capacity() < decoded.capacity() {
                    *buffer = SampleBuffer::<f32>::new(decoded.capacity() as u64, *decoded.spec());
                }

                buffer.copy_interleaved_ref(decoded);

                for frame in buffer.samples().chunks(reader.channels) {
                    let amplitude = frame
                        .iter()
                        .fold(0.0_f32, |max_value, sample| max_value.max(sample.abs()));
                    let bucket = usize::try_from(frame_index / frames_per_bar)
                        .unwrap_or(bars - 1)
                        .min(bars - 1);
                    peaks[bucket] = peaks[bucket].max(amplitude.min(1.0_f32));
                    frame_index += 1;
                }
            }
            Err(SymphoniaError::DecodeError(error)) => {
                tracing::warn!(path = %input_path.display(), error, "packet corrupto al generar waveform con Symphonia");
            }
            Err(SymphoniaError::IoError(error)) if error.kind() == ErrorKind::UnexpectedEof => {
                break
            }
            Err(error) => return Err(error.into()),
        }
    }

    Ok(peaks)
}

async fn probe_duration_with_ffprobe(
    ffprobe_path: &Path,
    input_path: &Path,
) -> Result<f32, AudioError> {
    let output = Command::new(ffprobe_path)
        .args([
            "-v",
            "quiet",
            "-print_format",
            "json",
            "-show_format",
            input_path.to_string_lossy().as_ref(),
        ])
        .output()
        .await?;

    if !output.status.success() {
        return Err(AudioError::ExternalToolFailed {
            tool: ffprobe_path.display().to_string(),
            status: output.status.code(),
            stderr: String::from_utf8_lossy(&output.stderr).trim().to_owned(),
        });
    }

    let parsed: FFprobeOutput = serde_json::from_slice(&output.stdout)?;
    Ok(parsed
        .format
        .and_then(|format| format.duration)
        .and_then(|value| value.parse::<f32>().ok())
        .unwrap_or(0.0))
}

async fn waveform_peaks_with_ffmpeg(
    ffmpeg_path: &Path,
    input_path: &Path,
    bars: usize,
) -> Result<Vec<f32>, AudioError> {
    let temp_path = temp_file_path("pcm");
    let args = vec![
        "-y".to_owned(),
        "-i".to_owned(),
        input_path.to_string_lossy().into_owned(),
        "-ac".to_owned(),
        "1".to_owned(),
        "-ar".to_owned(),
        WAVEFORM_SAMPLE_RATE.to_string(),
        "-f".to_owned(),
        "s16le".to_owned(),
        temp_path.to_string_lossy().into_owned(),
    ];

    let output = Command::new(ffmpeg_path).args(&args).output().await?;

    if !output.status.success() {
        let _ = std::fs::remove_file(&temp_path);
        return Err(AudioError::ExternalToolFailed {
            tool: ffmpeg_path.display().to_string(),
            status: output.status.code(),
            stderr: String::from_utf8_lossy(&output.stderr).trim().to_owned(),
        });
    }

    let bytes = std::fs::read(&temp_path)?;
    let _ = std::fs::remove_file(&temp_path);

    if bytes.len() < 2 || bars == 0 {
        return Ok(vec![0.0; bars]);
    }

    let mut peaks = vec![0.0_f32; bars];
    let total_samples = bytes.len() / 2;
    let samples_per_bar = usize::max(1, total_samples.div_ceil(bars));

    for (index, chunk) in bytes.chunks_exact(2).enumerate() {
        let amplitude =
            f32::from(i16::from_le_bytes([chunk[0], chunk[1]]).abs()) / f32::from(i16::MAX);
        let bucket = usize::min(index / samples_per_bar, bars - 1);
        peaks[bucket] = peaks[bucket].max(amplitude.min(1.0_f32));
    }

    Ok(peaks)
}

async fn run_ffmpeg(
    ffmpeg_path: &Path,
    input_path: &Path,
    output_path: &Path,
    extra_args: &[&str],
) -> Result<(), AudioError> {
    if let Some(parent) = output_path.parent() {
        std::fs::create_dir_all(parent)?;
    }

    let mut args = vec![
        "-y".to_owned(),
        "-i".to_owned(),
        input_path.to_string_lossy().into_owned(),
    ];
    args.extend(extra_args.iter().map(|value| (*value).to_owned()));
    args.push(output_path.to_string_lossy().into_owned());

    let output = Command::new(ffmpeg_path).args(&args).output().await?;

    if output.status.success() && output_path.is_file() {
        return Ok(());
    }

    Err(AudioError::ExternalToolFailed {
        tool: ffmpeg_path.display().to_string(),
        status: output.status.code(),
        stderr: String::from_utf8_lossy(&output.stderr).trim().to_owned(),
    })
}

fn temp_file_path(extension: &str) -> PathBuf {
    let mut path = env::temp_dir();
    path.push(format!(
        "kamples-audio-{}.{}",
        Uuid::new_v4(),
        extension.trim_start_matches('.')
    ));
    path
}

fn duration_from_frames(total_frames: u64, sample_rate_hz: u32) -> Duration {
    let sample_rate_hz = u64::from(sample_rate_hz.max(1));
    let seconds = total_frames / sample_rate_hz;
    let remainder_frames = total_frames % sample_rate_hz;
    let nanos = (u128::from(remainder_frames) * 1_000_000_000_u128) / u128::from(sample_rate_hz);

    Duration::new(seconds, u32::try_from(nanos).unwrap_or(u32::MAX))
}

fn open_audio_reader(
    input_path: &Path,
    format_hint: Option<&str>,
) -> Result<AudioReader, AudioError> {
    let file = File::open(input_path)?;
    let source = MediaSourceStream::new(Box::new(file), MediaSourceStreamOptions::default());
    let mut hint = Hint::new();

    if let Some(extension) = input_path.extension().and_then(|value| value.to_str()) {
        hint.with_extension(extension);
    }
    if let Some(format_hint) = format_hint.and_then(normalize_audio_format) {
        hint.with_extension(&format_hint);
    }

    let probed = symphonia::default::get_probe().format(
        &hint,
        source,
        &FormatOptions::default(),
        &MetadataOptions::default(),
    )?;
    let format = probed.format;
    let track = format
        .default_track()
        .ok_or_else(|| AudioError::MissingDefaultTrack(input_path.to_path_buf()))?;
    let track_id = track.id;
    let codec_params = track.codec_params.clone();
    let sample_rate_hz = codec_params
        .sample_rate
        .ok_or_else(|| AudioError::MissingSampleRate(input_path.to_path_buf()))?;
    let channels = codec_params
        .channels
        .map(symphonia::core::audio::Channels::count)
        .ok_or_else(|| AudioError::MissingChannels(input_path.to_path_buf()))?;
    let estimated_frames = codec_params.n_frames;
    let decoder =
        symphonia::default::get_codecs().make(&codec_params, &DecoderOptions::default())?;

    Ok(AudioReader {
        format,
        decoder,
        track_id,
        sample_rate_hz,
        channels,
        estimated_frames,
    })
}

fn count_frames(reader: &mut AudioReader) -> Result<u64, AudioError> {
    let mut total_frames = 0_u64;

    loop {
        let packet = match reader.format.next_packet() {
            Ok(packet) => packet,
            Err(SymphoniaError::IoError(error)) if error.kind() == ErrorKind::UnexpectedEof => {
                break
            }
            Err(SymphoniaError::ResetRequired) => {
                return Err(AudioError::Symphonia(
                    "Symphonia pidió reset del decoder".to_owned(),
                ));
            }
            Err(error) => return Err(error.into()),
        };

        if packet.track_id() != reader.track_id {
            continue;
        }

        match reader.decoder.decode(&packet) {
            Ok(decoded) => {
                total_frames += decoded.frames() as u64;
            }
            Err(SymphoniaError::DecodeError(error)) => {
                tracing::warn!(error, "packet corrupto al contar frames con Symphonia");
            }
            Err(SymphoniaError::IoError(error)) if error.kind() == ErrorKind::UnexpectedEof => {
                break
            }
            Err(error) => return Err(error.into()),
        }
    }

    Ok(total_frames)
}

#[cfg(test)]
mod tests;
