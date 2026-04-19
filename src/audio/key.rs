use rustfft::num_complex::Complex32;
use rustfft::FftPlanner;
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

pub const TARGET_SAMPLE_RATE_HZ: u32 = 22_050;
pub const MAX_ANALYSIS_SECONDS: u32 = 30;
pub const FFT_WINDOW_SIZE: usize = 4_096;
pub const FFT_HOP_SIZE: usize = 2_048;
const MIN_FREQ_HZ: f32 = 55.0;
const MAX_FREQ_HZ: f32 = 2_000.0;
const CHROMA_FREQUENCIES_HZ: [f32; 12] = [
    261.62558, 277.18262, 293.66476, 311.12698, 329.62756, 349.22824, 369.99442, 391.99542,
    415.3047, 440.0, 466.16376, 493.8833,
];
const MAJOR_PROFILE: [f32; 12] = [
    6.35, 2.23, 3.48, 2.33, 4.38, 4.09, 2.52, 5.19, 2.39, 3.66, 2.29, 2.88,
];
const MINOR_PROFILE: [f32; 12] = [
    6.33, 2.68, 3.52, 5.38, 2.6, 3.53, 2.54, 4.75, 3.98, 2.69, 3.34, 3.17,
];

/* [174A-32] Detector de tonalidad con cromagramas + perfiles mayor/menor.
 * Decodifica a mono con Symphonia, remuestrea a 22.05 kHz y acumula energía
 * cromática a partir de FFTs solapadas. La key final sale de correlacionar el
 * croma global contra perfiles rotados de 12 tonalidades mayores y menores. */

#[derive(Debug, Clone, PartialEq)]
pub struct KeyAnalysis {
    pub music_key: u8,
    pub scale: String,
    pub confidence: f32,
    pub analyzed_seconds: f32,
    pub chroma: Vec<f32>,
}

#[derive(Debug, Error)]
pub enum KeyError {
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

impl From<SymphoniaError> for KeyError {
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

pub fn detect_key(samples: &[f32], sample_rate_hz: u32) -> Option<KeyAnalysis> {
    if samples.is_empty() || sample_rate_hz == 0 {
        return None;
    }

    let limited = limit_analysis_window(samples, sample_rate_hz);
    let mono = resample_to_target(limited, sample_rate_hz, TARGET_SAMPLE_RATE_HZ);
    if mono.len() < FFT_WINDOW_SIZE {
        return None;
    }

    let chroma = compute_chroma_vector(&mono, TARGET_SAMPLE_RATE_HZ);
    if chroma.iter().all(|value| *value <= 0.0) {
        return None;
    }

    let normalized_chroma = normalize_l2(chroma);
    let mut candidates = Vec::with_capacity(24);
    for tonic in 0..12_u8 {
        candidates.push((
            tonic,
            "major",
            cosine_similarity(&normalized_chroma, &rotate_profile(&MAJOR_PROFILE, tonic)),
        ));
        candidates.push((
            tonic,
            "minor",
            cosine_similarity(&normalized_chroma, &rotate_profile(&MINOR_PROFILE, tonic)),
        ));
    }

    let mut ordered = candidates;
    ordered.sort_by(|left, right| right.2.partial_cmp(&left.2).unwrap_or(Ordering::Equal));
    let best = ordered.first()?;
    let mean_score = ordered.iter().map(|candidate| candidate.2).sum::<f32>()
        / f32::from(u16::try_from(ordered.len()).unwrap_or(u16::MAX));
    let confidence = if best.2 > 0.0 {
        ((best.2 - mean_score) / best.2).clamp(0.0, 1.0)
    } else {
        0.0
    };

    Some(KeyAnalysis {
        music_key: best.0,
        scale: best.1.to_owned(),
        confidence,
        analyzed_seconds: duration_from_sample_count(mono.len(), TARGET_SAMPLE_RATE_HZ)
            .as_secs_f32(),
        chroma: normalized_chroma,
    })
}

pub fn detect_key_from_file(
    input_path: &Path,
    format_hint: Option<&str>,
) -> Result<Option<KeyAnalysis>, KeyError> {
    if !input_path.is_file() {
        return Err(KeyError::MissingFile(input_path.to_path_buf()));
    }

    let (samples, sample_rate_hz) = decode_mono_samples(input_path, format_hint)?;
    Ok(detect_key(&samples, sample_rate_hz))
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

fn compute_chroma_vector(samples: &[f32], sample_rate_hz: u32) -> Vec<f32> {
    let mut planner = FftPlanner::<f32>::new();
    let fft = planner.plan_fft_forward(FFT_WINDOW_SIZE);
    let mut chroma_accumulator = [0.0_f32; 12];
    let bin_scale = f32::from(u16::try_from(sample_rate_hz).unwrap_or(u16::MAX))
        / f32::from(u16::try_from(FFT_WINDOW_SIZE).unwrap_or(u16::MAX));
    let mut frame_count = 0_u16;

    let mut start = 0_usize;
    while start + FFT_WINDOW_SIZE <= samples.len() {
        let mut spectrum = vec![Complex32::new(0.0, 0.0); FFT_WINDOW_SIZE];
        for (index, sample) in samples[start..start + FFT_WINDOW_SIZE].iter().enumerate() {
            spectrum[index].re = *sample;
        }
        fft.process(&mut spectrum);

        let mut frame_chroma = [0.0_f32; 12];
        for (bin_index, value) in spectrum
            .iter()
            .take(FFT_WINDOW_SIZE / 2)
            .enumerate()
            .skip(1)
        {
            let frequency_hz = f32::from(u16::try_from(bin_index).unwrap_or(u16::MAX)) * bin_scale;
            let Some(chroma_index) = chroma_index_for_frequency(frequency_hz) else {
                continue;
            };

            frame_chroma[chroma_index] += value.norm() / frequency_hz.max(1.0);
        }

        let normalized_frame = normalize_l2(frame_chroma.to_vec());
        for (index, value) in normalized_frame.iter().enumerate() {
            chroma_accumulator[index] += *value;
        }

        frame_count = frame_count.saturating_add(1);
        start += FFT_HOP_SIZE;
    }

    if frame_count == 0 {
        return vec![0.0; 12];
    }

    normalize_l2(chroma_accumulator.to_vec())
}

fn chroma_index_for_frequency(frequency_hz: f32) -> Option<usize> {
    if !frequency_hz.is_finite() || !(MIN_FREQ_HZ..=MAX_FREQ_HZ).contains(&frequency_hz) {
        return None;
    }

    CHROMA_FREQUENCIES_HZ
        .iter()
        .enumerate()
        .min_by(|(_, left), (_, right)| {
            let left_distance = octave_distance(frequency_hz, **left);
            let right_distance = octave_distance(frequency_hz, **right);
            left_distance
                .partial_cmp(&right_distance)
                .unwrap_or(Ordering::Equal)
        })
        .map(|(index, _)| index)
}

fn octave_distance(frequency_hz: f32, base_frequency_hz: f32) -> f32 {
    [0.25_f32, 0.5, 1.0, 2.0, 4.0]
        .into_iter()
        .map(|multiplier| {
            (frequency_hz / (base_frequency_hz * multiplier))
                .log2()
                .abs()
        })
        .fold(f32::INFINITY, f32::min)
}

fn normalize_l2(values: Vec<f32>) -> Vec<f32> {
    let norm = values.iter().map(|value| value * value).sum::<f32>().sqrt();
    if norm <= 0.0 {
        return values;
    }

    values.into_iter().map(|value| value / norm).collect()
}

fn rotate_profile(profile: &[f32; 12], tonic: u8) -> Vec<f32> {
    let mut rotated = vec![0.0_f32; 12];
    for (index, value) in profile.iter().enumerate() {
        rotated[(index + usize::from(tonic)) % 12] = *value;
    }
    normalize_l2(rotated)
}

fn cosine_similarity(left: &[f32], right: &[f32]) -> f32 {
    left.iter()
        .zip(right.iter())
        .map(|(a, b)| a * b)
        .sum::<f32>()
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
) -> Result<(Vec<f32>, u32), KeyError> {
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
                return Err(KeyError::Symphonia(
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
                tracing::warn!(path = %input_path.display(), error, "packet corrupto al decodificar para key detection");
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
) -> Result<AudioReader, KeyError> {
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
        .ok_or_else(|| KeyError::MissingDefaultTrack(input_path.to_path_buf()))?;
    let track_id = track.id;
    let codec_params = track.codec_params.clone();
    let sample_rate_hz = codec_params
        .sample_rate
        .ok_or_else(|| KeyError::MissingSampleRate(input_path.to_path_buf()))?;
    let channels = codec_params
        .channels
        .map(symphonia::core::audio::Channels::count)
        .ok_or_else(|| KeyError::MissingChannels(input_path.to_path_buf()))?;
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
#[path = "key/tests.rs"]
mod tests;
