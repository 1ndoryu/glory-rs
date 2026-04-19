use super::*;
use std::path::PathBuf;
use uuid::Uuid;

#[test]
fn detects_120_bpm_from_click_track() {
    let pcm = build_click_track_pcm(120, 8, TARGET_SAMPLE_RATE_HZ);
    let samples = pcm_to_f32(&pcm);

    let analysis =
        detect_bpm(&samples, TARGET_SAMPLE_RATE_HZ).expect("should detect BPM from click track");

    assert!((i64::from(analysis.bpm) - 120).abs() <= 2);
    assert!(analysis.confidence > 0.25);
    assert!(analysis.frame_count >= MIN_ONSET_FRAMES);
}

#[test]
fn returns_none_for_silence() {
    let silence = vec![0.0_f32; usize::try_from(TARGET_SAMPLE_RATE_HZ).unwrap_or(8_000)];
    assert!(detect_bpm(&silence, TARGET_SAMPLE_RATE_HZ).is_none());
}

#[test]
fn detects_90_bpm_from_wav_file() {
    let path = temp_wav_path();
    let pcm = build_click_track_pcm(90, 8, 44_100);
    write_wav_i16(&path, 44_100, &pcm);

    let analysis = detect_bpm_from_file(&path, None)
        .expect("should decode wav fixture")
        .expect("should detect BPM from wav fixture");

    let _ = std::fs::remove_file(path);
    assert!((i64::from(analysis.bpm) - 90).abs() <= 2);
    assert!(analysis.confidence > 0.20);
}

fn build_click_track_pcm(bpm: u32, beats: u32, sample_rate_hz: u32) -> Vec<i16> {
    let samples_per_beat =
        usize::try_from((u64::from(sample_rate_hz) * 60) / u64::from(bpm.max(1)))
            .expect("beat length should fit in usize");
    let total_samples =
        samples_per_beat * usize::try_from(beats).expect("beats should fit in usize");
    let click_samples = usize::try_from(sample_rate_hz / 50).unwrap_or(160).max(1);
    let mut pcm = vec![0_i16; total_samples];

    for beat in 0..usize::try_from(beats).expect("beats should fit in usize") {
        let start = beat * samples_per_beat;
        for offset in 0..click_samples {
            let index = start + offset;
            if index >= pcm.len() {
                break;
            }

            pcm[index] = if offset % 2 == 0 {
                i16::MAX / 2
            } else {
                i16::MIN / 2
            };
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
    path.push(format!("kamples-bpm-{}.wav", Uuid::new_v4()));
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
