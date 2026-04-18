use super::{normalize_creator, normalize_music_key, normalize_sample_type, normalize_tags};

#[test]
fn normalizes_sample_type_aliases() {
    assert!(matches!(
        normalize_sample_type(Some("one-shot".into())),
        Ok(Some(value)) if value == "oneshot"
    ));
    assert!(matches!(
        normalize_sample_type(Some("vocals".into())),
        Ok(Some(value)) if value == "vocal"
    ));
    assert!(matches!(
        normalize_sample_type(Some("other".into())),
        Ok(Some(value)) if value == "otro"
    ));
}

#[test]
fn rejects_invalid_sample_type() {
    assert!(normalize_sample_type(Some("drumkit".into())).is_err());
}

#[test]
fn normalizes_key_variants() {
    assert!(matches!(
        normalize_music_key(Some("bb".into())),
        Ok(Some(value)) if value == "Bb"
    ));
    assert!(matches!(
        normalize_music_key(Some(" f# ".into())),
        Ok(Some(value)) if value == "F#"
    ));
}

#[test]
fn normalizes_tags_and_dedupes() {
    let tags = normalize_tags(Some(" Trap,drill,trap, boom bap ".into())).unwrap_or_default();
    assert_eq!(tags, vec!["trap", "drill", "boom bap"]);
}

#[test]
fn trims_creator_and_rejects_too_long_values() {
    assert!(matches!(
        normalize_creator(Some("  indoryu ".into())),
        Ok(Some(value)) if value == "indoryu"
    ));
    assert!(normalize_creator(Some("x".repeat(51))).is_err());
}
