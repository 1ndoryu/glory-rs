use super::*;

#[test]
fn derivative_key_keeps_parent_directory() {
    let key = build_derivative_key("samples/2026/04/77/demo.wav", "abc123", "waveform", "json");
    assert_eq!(key, "samples/2026/04/77/abc123_waveform.json");
}

#[test]
fn derivative_key_works_without_parent() {
    let key = build_derivative_key("demo.wav", "abc123", "optimizado", "mp3");
    assert_eq!(key, "abc123_optimizado.mp3");
}

#[test]
fn sample_type_normalization_preserves_legacy_one_shot() {
    assert_eq!(normalize_sample_type_for_embedding("oneshot"), "one_shot");
    assert_eq!(normalize_sample_type_for_embedding("loop"), "loop");
}

#[test]
fn music_key_labels_match_detector_indexes() {
    assert_eq!(music_key_label(0), "C");
    assert_eq!(music_key_label(10), "A#");
    assert_eq!(music_key_label(13), "C#");
}

#[test]
fn pipeline_metadata_carries_partial_progress() {
    let progress = PipelineProgress {
        stage: AudioPipelineStage::PersistOptimizedMp3,
        analysis: Some(AudioTechnicalAnalysis {
            format: "wav".to_owned(),
            duration_seconds: 12.5,
            sample_rate_hz: 44_100,
            channels: 2,
            file_size_bytes: 1_024,
            bpm: Some(120),
            bpm_confidence: Some(0.82),
            music_key: Some("C#".to_owned()),
            scale: Some("minor".to_owned()),
            key_confidence: Some(0.41),
        }),
        assets: GeneratedAudioAssets {
            original_key: "samples/demo.wav".to_owned(),
            optimized_key: Some("samples/demo_optimizado.mp3".to_owned()),
            waveform_key: Some("samples/demo_waveform.json".to_owned()),
        },
        warnings: vec!["ffprobe no disponible".to_owned()],
    };

    let metadata = build_pipeline_metadata(&progress, "optimized_ready", Some("sin preview"));

    assert_eq!(metadata["audio_pipeline"]["status"], "optimized_ready");
    assert_eq!(metadata["audio_pipeline"]["last_stage"], "persist_optimized_mp3");
    assert_eq!(metadata["audio_pipeline"]["music_key"], "C#");
    assert_eq!(metadata["audio_pipeline"]["ruta_optimizada"], "samples/demo_optimizado.mp3");
    assert_eq!(metadata["audio_pipeline"]["last_error"], "sin preview");
}