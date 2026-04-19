use std::cmp::Ordering;
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

pub const TARGET_SAMPLE_RATE_HZ: u32 = 8_000;
pub const MAX_ANALYSIS_SECONDS: u32 = 30;
pub const FRAME_MS: u32 = 50;
pub const HOP_MS: u32 = 25;
pub const BPM_MIN: u32 = 60;
pub const BPM_MAX: u32 = 200;
const MIN_ONSET_FRAMES: usize = 20;

/* [174A-31] Detector BPM determinista basado en Symphonia.
 * Replica la estrategia del legado: energía RMS por ventana, función de onset
 * half-wave rectified y autocorrelación en el rango 60-200 BPM.
 * Gotcha: el análisis se limita a 30s y remuestrea a 8 kHz para mantener
 * costo acotado y resultados estables entre formatos. */

#[derive(Debug, Clone, PartialEq)]
pub struct BpmAnalysis {
    pub bpm: u32,
    pub confidence: f32,
    pub analyzed_seconds: f32,
    pub frame_count: usize,
}

#[derive(Debug, Error)]
pub enum BpmError {
    #[error("No se encontró el archivo de audio: {0}")]
    MissingFile(PathBuf),
    #[error("No se encontró una pista de audio por defecto en {0}")]
    MissingDefaultTrack(PathBuf),
    #[error("La pista de audio no expone sample_rate en {0}")]
    MissingSampleRate(PathBuf),
    #[error("La pista de audio no expone canales en {0}")]
    MissingChannels(PathBuf),
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error("Symphonia devolvió un error: {0}")]
    Symphonia(String),
}

impl From<SymphoniaError> for BpmError {
    fn from(value: SymphoniaError) -> Self {
        Self::Symphonia(value.to_string())
    }
}

struct AudioReader {
    format: Box<dyn FormatReader>,
    decoder: Box<dyn Decoder>,
    track_id: u32,
    sample_rate_hz: u32,
    channels: usize,
}

pub fn detect_bpm(samples: &[f32], sample_rate_hz: u32) -> Option<BpmAnalysis> {
    if samples.is_empty() || sample_rate_hz == 0 {
        return None;
    }

    let limited = limit_analysis_window(samples, sample_rate_hz);
    let mono = resample_to_target(limited, sample_rate_hz, TARGET_SAMPLE_RATE_HZ);
    let frame_samples = frame_size_samples();
    let hop_samples = hop_size_samples();
    let energy = compute_rms_energy(&mono, frame_samples, hop_samples);

    if energy.len() < MIN_ONSET_FRAMES {
        return None;
    }

    let onsets = build_onset_envelope(&energy);
    if onsets.iter().all(|value| *value <= 0.0) {
        return None;
    }

    let lag_min = usize::try_from(60_000 / (BPM_MAX * HOP_MS)).ok()?;
    let lag_max = usize::try_from(60_000 / (BPM_MIN * HOP_MS))
        .ok()?
        .min(onsets.len() / 2);

    if lag_min >= lag_max {
        return None;
    }

    let correlations = autocorrelate(&onsets, lag_min, lag_max);
    let (best_lag, best_correlation) = correlations
        .iter()
        .copied()
        .max_by(|left, right| left.1.partial_cmp(&right.1).unwrap_or(Ordering::Equal))?;

    let bpm = bpm_from_lag(best_lag)?;
    if !(BPM_MIN..=BPM_MAX).contains(&bpm) {
        return None;
    }

    let mean_correlation = mean(
        &correlations
            .iter()
            .map(|(_, value)| *value)
            .collect::<Vec<_>>(),
    );
    let confidence = if mean_correlation > 0.0 {
        (best_correlation / (mean_correlation * 3.0)).min(1.0)
    } else {
        0.0
    };

    Some(BpmAnalysis {
        bpm,
        confidence,
        analyzed_seconds: duration_from_sample_count(mono.len(), TARGET_SAMPLE_RATE_HZ)
            .as_secs_f32(),
        frame_count: energy.len(),
    })
}

pub fn detect_bpm_from_file(
    input_path: &Path,
    format_hint: Option<&str>,
) -> Result<Option<BpmAnalysis>, BpmError> {
    if !input_path.is_file() {
        return Err(BpmError::MissingFile(input_path.to_path_buf()));
    }

    let (samples, sample_rate_hz) = decode_mono_samples(input_path, format_hint)?;
    Ok(detect_bpm(&samples, sample_rate_hz))
}

fn limit_analysis_window(samples: &[f32], sample_rate_hz: u32) -> &[f32] {
    let max_samples = usize::try_from(u64::from(sample_rate_hz) * u64::from(MAX_ANALYSIS_SECONDS))
        .unwrap_or(samples.len())
        .min(samples.len());
    &samples[..max_samples]
}

fn resample_to_target(samples: &[f32], input_rate_hz: u32, target_rate_hz: u32) -> Vec<f32> {
    if input_rate_hz == target_rate_hz {
        return samples.to_vec();
    }

    let output_len = usize::try_from(
        (u128::try_from(samples.len()).unwrap_or(u128::MAX) * u128::from(target_rate_hz))
            / u128::from(input_rate_hz.max(1)),
    )
    .unwrap_or(samples.len())
    .max(1);

    let mut output = Vec::with_capacity(output_len);
    for output_index in 0..output_len {
        let input_index = usize::try_from(
            (u128::try_from(output_index).unwrap_or(u128::MAX) * u128::from(input_rate_hz))
                / u128::from(target_rate_hz.max(1)),
        )
        .unwrap_or(samples.len().saturating_sub(1))
        .min(samples.len().saturating_sub(1));
        output.push(samples[input_index]);
    }

    output
}

fn frame_size_samples() -> usize {
    usize::try_from((u64::from(TARGET_SAMPLE_RATE_HZ) * u64::from(FRAME_MS)) / 1_000).unwrap_or(400)
}

fn hop_size_samples() -> usize {
    usize::try_from((u64::from(TARGET_SAMPLE_RATE_HZ) * u64::from(HOP_MS)) / 1_000).unwrap_or(200)
}

fn compute_rms_energy(samples: &[f32], frame_samples: usize, hop_samples: usize) -> Vec<f32> {
    if frame_samples == 0 || hop_samples == 0 || samples.len() < frame_samples {
        return Vec::new();
    }

    let frame_scale = 1.0_f32 / f32::from(u16::try_from(frame_samples).unwrap_or(u16::MAX));
    let mut energy = Vec::new();
    let mut position = 0_usize;

    while position + frame_samples <= samples.len() {
        let mut squared_sum = 0.0_f32;
        for sample in &samples[position..position + frame_samples] {
            squared_sum += sample * sample;
        }
        energy.push((squared_sum * frame_scale).sqrt());
        position += hop_samples;
    }

    energy
}

fn build_onset_envelope(energy: &[f32]) -> Vec<f32> {
    if energy.is_empty() {
        return Vec::new();
    }

    let mut onsets = Vec::with_capacity(energy.len());
    onsets.push(0.0);
    for window in energy.windows(2) {
        onsets.push((window[1] - window[0]).max(0.0));
    }

    let max_onset = onsets.iter().copied().fold(0.0_f32, f32::max);
    if max_onset <= 0.0 {
        return onsets;
    }

    onsets.into_iter().map(|value| value / max_onset).collect()
}

fn autocorrelate(onsets: &[f32], lag_min: usize, lag_max: usize) -> Vec<(usize, f32)> {
    let mut correlations = Vec::with_capacity(lag_max.saturating_sub(lag_min) + 1);

    for lag in lag_min..=lag_max {
        let frame_count = onsets.len().saturating_sub(lag);
        if frame_count == 0 {
            continue;
        }

        let mut sum = 0.0_f32;
        for index in 0..frame_count {
            sum += onsets[index] * onsets[index + lag];
        }

        let scale = 1.0_f32 / f32::from(u16::try_from(frame_count).unwrap_or(u16::MAX));
        correlations.push((lag, sum * scale));
    }

    correlations
}

fn mean(values: &[f32]) -> f32 {
    if values.is_empty() {
        return 0.0;
    }

    let total = values.iter().sum::<f32>();
    total / f32::from(u16::try_from(values.len()).unwrap_or(u16::MAX))
}

fn bpm_from_lag(lag: usize) -> Option<u32> {
    let lag = u32::try_from(lag).ok()?.max(1);
    Some((60_000 + (lag * HOP_MS) / 2) / (lag * HOP_MS))
}

fn duration_from_sample_count(sample_count: usize, sample_rate_hz: u32) -> Duration {
    let sample_count = u64::try_from(sample_count).unwrap_or(u64::MAX);
    let sample_rate_hz = u64::from(sample_rate_hz.max(1));
    let seconds = sample_count / sample_rate_hz;
    let remainder = sample_count % sample_rate_hz;
    let nanos = (u128::from(remainder) * 1_000_000_000_u128) / u128::from(sample_rate_hz);

    Duration::new(seconds, u32::try_from(nanos).unwrap_or(u32::MAX))
}

fn decode_mono_samples(
    input_path: &Path,
    format_hint: Option<&str>,
) -> Result<(Vec<f32>, u32), BpmError> {
    let mut reader = open_audio_reader(input_path, format_hint)?;
    let max_frames =
        usize::try_from(u64::from(reader.sample_rate_hz) * u64::from(MAX_ANALYSIS_SECONDS))
            .unwrap_or(usize::MAX);
    let channel_scale = 1.0_f32 / f32::from(u16::try_from(reader.channels).unwrap_or(u16::MAX));
    let mut mono_samples = Vec::new();
    let mut sample_buffer: Option<SampleBuffer<f32>> = None;

    loop {
        if mono_samples.len() >= max_frames {
            break;
        }

        let packet = match reader.format.next_packet() {
            Ok(packet) => packet,
            Err(SymphoniaError::IoError(error)) if error.kind() == ErrorKind::UnexpectedEof => {
                break
            }
            Err(SymphoniaError::ResetRequired) => {
                return Err(BpmError::Symphonia(
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
                    let mono = frame.iter().copied().sum::<f32>() * channel_scale;
                    mono_samples.push(mono);
                    if mono_samples.len() >= max_frames {
                        break;
                    }
                }
            }
            Err(SymphoniaError::DecodeError(error)) => {
                tracing::warn!(path = %input_path.display(), error, "packet corrupto al decodificar para BPM");
            }
            Err(SymphoniaError::IoError(error)) if error.kind() == ErrorKind::UnexpectedEof => {
                break
            }
            Err(error) => return Err(error.into()),
        }
    }

    Ok((mono_samples, reader.sample_rate_hz))
}

fn open_audio_reader(
    input_path: &Path,
    format_hint: Option<&str>,
) -> Result<AudioReader, BpmError> {
    let file = File::open(input_path)?;
    let source = MediaSourceStream::new(Box::new(file), MediaSourceStreamOptions::default());
    let mut hint = Hint::new();

    if let Some(extension) = input_path.extension().and_then(|value| value.to_str()) {
        hint.with_extension(extension);
    }
    if let Some(format_hint) = format_hint {
        hint.with_extension(format_hint);
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
        .ok_or_else(|| BpmError::MissingDefaultTrack(input_path.to_path_buf()))?;
    let track_id = track.id;
    let codec_params = track.codec_params.clone();
    let sample_rate_hz = codec_params
        .sample_rate
        .ok_or_else(|| BpmError::MissingSampleRate(input_path.to_path_buf()))?;
    let channels = codec_params
        .channels
        .map(symphonia::core::audio::Channels::count)
        .ok_or_else(|| BpmError::MissingChannels(input_path.to_path_buf()))?;
    let decoder =
        symphonia::default::get_codecs().make(&codec_params, &DecoderOptions::default())?;

    Ok(AudioReader {
        format,
        decoder,
        track_id,
        sample_rate_hz,
        channels,
    })
}

#[cfg(test)]
#[path = "bpm/tests.rs"]
mod tests;
