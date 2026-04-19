use super::*;
use std::f32::consts::PI;
use std::path::PathBuf;
use uuid::Uuid;

#[test]
fn detects_c_major_scale() {
    let pcm = build_note_cycle_pcm(
        &[262, 294, 330, 349, 392, 440, 494, 523],
        180,
        2,
        TARGET_SAMPLE_RATE_HZ,
    );
    let samples = pcm_to_f32(&pcm);

    let analysis =
        detect_key(&samples, TARGET_SAMPLE_RATE_HZ).expect("should detect key from major scale");

    assert_eq!(analysis.music_key, 0);
    assert_eq!(analysis.scale, "major");
    assert!(analysis.confidence > 0.05);
}

#[test]
fn returns_none_for_silence() {
    let silence = vec![0.0_f32; FFT_WINDOW_SIZE * 2];
    assert!(detect_key(&silence, TARGET_SAMPLE_RATE_HZ).is_none());
}

#[test]
fn detects_c_minor_scale_from_wav_file() {
    let path = temp_wav_path();
    let pcm = build_note_cycle_pcm(&[262, 294, 311, 349, 392, 415, 466, 523], 180, 2, 44_100);
    write_wav_i16(&path, 44_100, &pcm);

    let analysis = detect_key_from_file(&path, None)
        .expect("should decode wav fixture")
        .expect("should detect key from minor scale wav fixture");

    let _ = std::fs::remove_file(path);
    assert_eq!(analysis.music_key, 0);
    assert_eq!(analysis.scale, "minor");
    assert!(analysis.confidence > 0.05);
}

fn build_note_cycle_pcm(
    frequencies_hz: &[u32],
    note_duration_ms: u32,
    repetitions: u32,
    sample_rate_hz: u32,
) -> Vec<i16> {
    let note_samples =
        usize::try_from((u64::from(sample_rate_hz) * u64::from(note_duration_ms)) / 1_000)
            .expect("fixture should fit in usize");
    let total_notes =
        usize::try_from(u64::from(repetitions) * u64::try_from(frequencies_hz.len()).unwrap_or(0))
            .expect("total notes should fit in usize");
    let total_samples = note_samples * total_notes;
    let sample_rate_f32 = f32::from(u16::try_from(sample_rate_hz).unwrap_or(u16::MAX));
    let note_samples_f32 = f32::from(u16::try_from(note_samples).unwrap_or(u16::MAX));
    let mut pcm = vec![0_i16; total_samples];

    let mut cursor = 0_usize;
    for _ in 0..repetitions {
        for frequency_hz in frequencies_hz {
            let phase_step = 2.0 * PI * f32::from(u16::try_from(*frequency_hz).unwrap_or(u16::MAX))
                / sample_rate_f32;
            let mut phase = 0.0_f32;
            for offset in 0..note_samples {
                let sample = &mut pcm[cursor + offset];
                let offset_f32 = f32::from(u16::try_from(offset).unwrap_or(u16::MAX));
                let envelope = (1.0 - (offset_f32 / note_samples_f32)).clamp(0.2, 1.0);
                let value = phase.sin() * 0.75 * envelope;
                *sample = quantize_f32_to_i16(value * f32::from(i16::MAX));
                phase += phase_step;
                if phase >= 2.0 * PI {
                    phase -= 2.0 * PI;
                }
            }

            cursor += note_samples;
        }
    }

    pcm
}

fn pcm_to_f32(pcm: &[i16]) -> Vec<f32> {
    pcm.iter()
        .map(|sample| f32::from(*sample) / f32::from(i16::MAX))
        .collect()
}

fn temp_wav_path() -> PathBuf {
    let mut path = std::env::temp_dir();
    path.push(format!("kamples-key-{}.wav", Uuid::new_v4()));
    path
}

fn write_wav_i16(path: &PathBuf, sample_rate_hz: u32, pcm: &[i16]) {
    let data_len = u32::try_from(pcm.len() * 2).expect("fixture should fit in u32");
    let mut wav = Vec::with_capacity(44 + usize::try_from(data_len).unwrap_or(0));
    wav.extend_from_slice(b"RIFF");
    wav.extend_from_slice(&(36 + data_len).to_le_bytes());
    wav.extend_from_slice(b"WAVE");
    wav.extend_from_slice(b"fmt ");
    wav.extend_from_slice(&16_u32.to_le_bytes());
    wav.extend_from_slice(&1_u16.to_le_bytes());
    wav.extend_from_slice(&1_u16.to_le_bytes());
    wav.extend_from_slice(&sample_rate_hz.to_le_bytes());
    wav.extend_from_slice(&(sample_rate_hz * 2).to_le_bytes());
    wav.extend_from_slice(&2_u16.to_le_bytes());
    wav.extend_from_slice(&16_u16.to_le_bytes());
    wav.extend_from_slice(b"data");
    wav.extend_from_slice(&data_len.to_le_bytes());
    for sample in pcm {
        wav.extend_from_slice(&sample.to_le_bytes());
    }

    std::fs::write(path, wav).expect("should write wav fixture");
}

#[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
fn quantize_f32_to_i16(value: f32) -> i16 {
    value
        .round()
        .clamp(f32::from(i16::MIN), f32::from(i16::MAX)) as i16
}
