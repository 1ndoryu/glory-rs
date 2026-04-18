use super::{
    asset_to_public_url, build_sample_detail, normalize_creator, normalize_music_key,
    normalize_sample_type, normalize_tags,
};
use crate::repositories::SampleCatalogDetailRecord;

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

#[test]
fn builds_public_urls_from_storage_keys() {
    assert_eq!(
        asset_to_public_url(Some("https://kamples.test"), Some("samples/9/demo.mp3".into())),
        Some("https://kamples.test/uploads/samples/9/demo.mp3".into())
    );
    assert_eq!(
        asset_to_public_url(None, Some("samples/9/demo.mp3".into())),
        Some("/uploads/samples/9/demo.mp3".into())
    );
    assert_eq!(
        asset_to_public_url(Some("https://kamples.test"), Some("https://cdn.test/demo.mp3".into())),
        Some("https://cdn.test/demo.mp3".into())
    );
}

#[test]
fn detail_only_exposes_private_asset_urls_to_owner() {
    let record = SampleCatalogDetailRecord {
        id: 12,
        id_corto: Some("abc123".into()),
        slug: "mi-sample-abc123".into(),
        titulo: "Mi sample".into(),
        descripcion: "detalle".into(),
        bpm: Some(140),
        music_key: Some("F#".into()),
        escala: Some("minor".into()),
        duracion: 4.2,
        formato: "wav".into(),
        tamano: 1024,
        tags: vec!["trap".into()],
        tipo: "oneshot".into(),
        estado: "procesando".into(),
        es_premium: false,
        precio: None,
        metadata: serde_json::json!({"origen": "test"}),
        ruta_preview: Some("samples/12/preview.mp3".into()),
        ruta_waveform: Some("samples/12/waveform.json".into()),
        ruta_original: Some("samples/12/original.wav".into()),
        ruta_optimizada: Some("samples/12/optimized.mp3".into()),
        permitir_descarga: true,
        licencia_libre: false,
        imagen_url: Some("samples/12/cover.jpg".into()),
        total_descargas: 1,
        total_likes: 2,
        total_reproducciones: 3,
        total_comentarios: 4,
        audio_hash: Some("hash".into()),
        verificado: false,
        mostrar_en_comunidad: true,
        publicado_at: None,
        created_at: None,
        cancion_origen_id: None,
        relacion_sampleo_id: None,
        creator_id: 77,
        creator_username: "indoryu".into(),
        creator_nombre_visible: Some("Indoryu".into()),
        creator_avatar_url: Some("avatars/77.png".into()),
        creator_verificado: true,
    };

    let owner = build_sample_detail(record.clone(), Some("https://kamples.test"), Some(77));
    assert_eq!(
        owner.ruta_original,
        Some("https://kamples.test/uploads/samples/12/original.wav".into())
    );
    assert_eq!(
        owner.ruta_optimizada,
        Some("https://kamples.test/uploads/samples/12/optimized.mp3".into())
    );

    let outsider = build_sample_detail(record, Some("https://kamples.test"), Some(99));
    assert_eq!(outsider.ruta_original, None);
    assert_eq!(outsider.ruta_optimizada, None);
}
