use super::*;

#[test]
fn generate_is_deterministic_for_same_input() {
    let input = EmbeddingInput {
        bpm: Some(120),
        music_key: Some("C#".to_owned()),
        scale: Some("major".to_owned()),
        sample_type: Some("one_shot".to_owned()),
        duration_seconds: Some(12.5),
        is_premium: true,
        tags: vec!["Synth".to_owned(), "Lofi".to_owned()],
    };

    let first = AudioEmbedding::generate(&input);
    let second = AudioEmbedding::generate(&input);

    assert_eq!(first, second);
}

#[test]
fn enharmonic_keys_share_same_slot() {
    let sharp = AudioEmbedding::generate(&EmbeddingInput {
        music_key: Some("C#".to_owned()),
        ..EmbeddingInput::default()
    });
    let flat = AudioEmbedding::generate(&EmbeddingInput {
        music_key: Some("Db".to_owned()),
        ..EmbeddingInput::default()
    });

    assert_eq!(sharp, flat);
    assert!((sharp.as_slice()[2] - 1.0).abs() < 0.0001);
}

#[test]
fn missing_scale_uses_neutral_profile() {
    let embedding = AudioEmbedding::generate(&EmbeddingInput::default());

    assert!((embedding.as_slice()[13] - 0.5).abs() < 0.0001);
    assert!((embedding.as_slice()[14] - 0.5).abs() < 0.0001);
}

#[test]
fn tags_are_hashed_case_insensitively() {
    let first = AudioEmbedding::generate(&EmbeddingInput {
        tags: vec!["  Synth  ".to_owned(), "LoFi".to_owned()],
        ..EmbeddingInput::default()
    });
    let second = AudioEmbedding::generate(&EmbeddingInput {
        tags: vec!["synth".to_owned(), "lofi".to_owned()],
        ..EmbeddingInput::default()
    });

    assert_eq!(first, second);
}

#[test]
fn weighted_profile_is_l2_normalized() {
    let first = AudioEmbedding::generate(&EmbeddingInput {
        bpm: Some(90),
        music_key: Some("A".to_owned()),
        scale: Some("minor".to_owned()),
        tags: vec!["ambient".to_owned()],
        ..EmbeddingInput::default()
    });
    let second = AudioEmbedding::generate(&EmbeddingInput {
        bpm: Some(128),
        music_key: Some("C".to_owned()),
        scale: Some("major".to_owned()),
        tags: vec!["house".to_owned(), "club".to_owned()],
        ..EmbeddingInput::default()
    });

    let profile = AudioEmbedding::build_weighted_profile(&[first, second], &[0.25, 0.75])
        .expect("weighted profile should be created");

    assert!((profile.l2_norm() - 1.0).abs() < 0.001);
}

#[test]
fn pgvector_roundtrip_preserves_values() {
    let embedding = AudioEmbedding::generate(&EmbeddingInput {
        bpm: Some(110),
        music_key: Some("F#".to_owned()),
        scale: Some("minor".to_owned()),
        duration_seconds: Some(32.0),
        tags: vec!["drums".to_owned(), "breakbeat".to_owned()],
        ..EmbeddingInput::default()
    });

    let vector = embedding.to_pgvector();
    let restored = AudioEmbedding::from_pgvector(&vector).expect("vector should roundtrip");

    assert_eq!(embedding, restored);
}

#[test]
fn from_slice_rejects_wrong_dimension() {
    let error =
        AudioEmbedding::from_slice(&[0.0_f32; 127]).expect_err("dimension mismatch should fail");
    assert_eq!(error, EmbeddingError::InvalidDimension(127));
}
