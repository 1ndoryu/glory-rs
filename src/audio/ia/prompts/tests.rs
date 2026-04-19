use super::*;

#[test]
fn analysis_prompt_uses_extraction_context_when_available() {
    let prompt = build_analysis_prompt(&AudioAnalysisPromptInput {
        original_filename: "ignored.wav".to_owned(),
        extraction_context: Some(AudioExtractionContext {
            source_title: Some("Juicy".to_owned()),
            source_artist: Some("The Notorious B.I.G.".to_owned()),
            destination_title: Some("Dreams and Nightmares".to_owned()),
            destination_artist: Some("Meek Mill".to_owned()),
            element_type: Some("piano loop".to_owned()),
            vote_count: Some(12),
        }),
        ..AudioAnalysisPromptInput::default()
    });

    assert!(prompt.contains("Sampled from \"Juicy\" by The Notorious B.I.G."));
    assert!(prompt.contains("Used in \"Dreams and Nightmares\" by Meek Mill"));
    assert!(prompt.contains("Element: piano loop"));
    assert!(prompt.contains("Confidence: 12 votes"));
    assert!(!prompt.contains("ignored.wav"));
}

#[test]
fn analysis_prompt_falls_back_to_filename_and_context_fields() {
    let prompt = build_analysis_prompt(&AudioAnalysisPromptInput {
        original_filename: "dark-loop.wav".to_owned(),
        user_description: "moody trap bells".to_owned(),
        user_tags: vec!["dark".to_owned(), "bells".to_owned()],
        bpm: Some(140.0),
        musical_key: Some("C#".to_owned()),
        scale: Some("minor".to_owned()),
        duration_seconds: Some(6.4),
        upload_origin: Some("packs/trap/night".to_owned()),
        extraction_context: None,
    });

    assert!(prompt.contains("El archivo se subio con este nombre: \"dark-loop.wav\"."));
    assert!(prompt.contains("#dark, #bells"));
    assert!(prompt.contains("BPM de 140.0"));
    assert!(prompt.contains("La tonalidad detectada es C# minor."));
    assert!(prompt.contains("Dura 6.4 segundos."));
    assert!(prompt.contains("packs/trap/night"));
    assert!(prompt.contains("carpeta_primaria"));
    assert!(prompt.contains("carpeta_secundaria"));
}

#[test]
fn transcription_context_is_truncated_and_appended() {
    let transcription = "a".repeat(3_500);
    let prompt = append_transcription_context("base", &transcription);

    assert!(prompt.starts_with("base\n\nContexto adicional obtenido por transcripcion"));
    assert!(prompt.contains(&"a".repeat(3_000)));
    assert!(!prompt.contains(&"a".repeat(3_100)));
}

#[test]
fn correction_prompt_includes_current_metadata_and_constraints() {
    let prompt = build_correction_prompt(&MetadataCorrectionPromptInput {
        current_metadata_json: "{\"tipo\":\"loop\"}".to_owned(),
        title: "Night Piano".to_owned(),
        instructions: "cambia el genero a jazz noir".to_owned(),
        bpm: Some(92.0),
        musical_key: Some("F minor".to_owned()),
    });

    assert!(prompt.contains("Night Piano"));
    assert!(prompt.contains("BPM: 92.0"));
    assert!(prompt.contains("Key: F minor"));
    assert!(prompt.contains("cambia el genero a jazz noir"));
    assert!(prompt.contains("\"nombre_archivo_base\""));
    assert!(prompt.contains("\"carpeta_primaria\""));
}
