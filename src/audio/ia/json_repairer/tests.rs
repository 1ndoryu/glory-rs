use super::*;
use serde_json::json;

#[test]
fn parses_direct_json_text() {
    let metadata = JsonRepairer::extract_metadata_from_text(
        r#"{"tipo":"loop","tags":["Dark","Punchy"],"genero":["trap"],"descripcion":"Night drums","carpeta_primaria":"Drums","carpeta_secundaria":"Kicks"}"#,
    )
    .expect("direct JSON should parse");

    assert_eq!(metadata.tipo, "loop");
    assert_eq!(metadata.tags, vec!["Dark", "Punchy"]);
    assert_eq!(metadata.genero, vec!["trap"]);
    assert_eq!(metadata.descripcion, "Night drums");
    assert_eq!(metadata.carpeta_primaria, "Drums");
    assert_eq!(metadata.carpeta_secundaria, "Kicks");
}

#[test]
fn extracts_json_from_fenced_block() {
    let metadata = JsonRepairer::extract_metadata_from_text(
        "Respuesta:\n```json\n{\n  \"tipo\": \"one shot\",\n  \"instrumentos\": [\"snare\"],\n  \"carpeta_primaria\": \"Drums\"\n}\n```",
    )
    .expect("fenced JSON should parse");

    assert_eq!(metadata.tipo, "oneshot");
    assert_eq!(metadata.instrumentos, vec!["snare"]);
    assert_eq!(metadata.carpeta_primaria, "Drums");
    assert_eq!(metadata.carpeta_secundaria, "General");
}

#[test]
fn extracts_balanced_json_from_surrounding_text() {
    let metadata = JsonRepairer::extract_metadata_from_text(
        "Te dejo el analisis final: {\"emocion\":[\"aggressive\"],\"descripcion_corta\":\"street energy\",\"carpeta_primaria\":\"Samples\",\"carpeta_secundaria\":\"Phonk\"} gracias",
    )
    .expect("embedded object should parse");

    assert_eq!(metadata.emocion, vec!["aggressive"]);
    assert_eq!(metadata.descripcion_corta, "street energy");
    assert_eq!(metadata.carpeta_primaria, "Samples");
    assert_eq!(metadata.carpeta_secundaria, "Phonk");
}

#[test]
fn accepts_json5_like_output_and_scalar_arrays() {
    let metadata = JsonRepairer::extract_metadata_from_text(
        "{tipo: 'one shot', tags: ['Dark',], genero: 'trap', artista_vibes: 'Metro Boomin', carpeta_primaria: 'Instruments', carpeta_secundaria: 'Keys'}",
    )
    .expect("json5-like output should parse");

    assert_eq!(metadata.tipo, "oneshot");
    assert_eq!(metadata.tags, vec!["Dark"]);
    assert_eq!(metadata.genero, vec!["trap"]);
    assert_eq!(metadata.artista_vibes, vec!["Metro Boomin"]);
    assert_eq!(metadata.carpeta_primaria, "Instruments");
    assert_eq!(metadata.carpeta_secundaria, "Keys");
}

#[test]
fn cleans_control_chars_inside_strings() {
    let metadata = JsonRepairer::extract_metadata_from_text(
        "{\"descripcion\":\"pads\nwith\tbloom\",\"descripcion_es\":\"linea\u{0007} rota\"}",
    )
    .expect("control chars should be sanitized");

    assert_eq!(metadata.descripcion, "pads with bloom");
    assert_eq!(metadata.descripcion_es, "linea rota");
}

#[test]
fn parses_openai_compatible_provider_response() {
    let raw = json!({
        "choices": [
            {
                "message": {
                    "content": "{\"tags\":[\"Warm\"],\"tipo\":\"loop\"}"
                }
            }
        ]
    })
    .to_string();

    let metadata = JsonRepairer::extract_metadata_from_provider_response(&raw)
        .expect("provider response should parse");

    assert_eq!(metadata.tags, vec!["Warm"]);
    assert_eq!(metadata.tipo, "loop");
}

#[test]
fn validates_lengths_and_defaults() {
    let raw = json!({
        "tipo": "desconocido",
        "tags": ["a","b","c","d","e","f","g","h","i","j","k","l","m","n","o","p"],
        "emocion": ["1","2","3","4","5","6"],
        "descripcion": "  linea   larga   ",
        "carpeta_primaria": "desconocida"
    });

    let metadata = AudioCreativeMetadata::from_object(
        raw.as_object().expect("test metadata should be object"),
    );

    assert_eq!(metadata.tipo, "oneshot");
    assert_eq!(metadata.tags.len(), 15);
    assert_eq!(metadata.emocion.len(), 5);
    assert_eq!(metadata.descripcion, "linea larga");
    assert_eq!(metadata.carpeta_primaria, "General");
    assert_eq!(metadata.carpeta_secundaria, "General");
}

#[test]
fn errors_when_provider_response_has_no_content() {
    let raw = json!({ "choices": [{ "message": {} }] }).to_string();
    let error = JsonRepairer::extract_metadata_from_provider_response(&raw)
        .expect_err("missing content should fail");

    assert_eq!(error, JsonRepairError::MissingProviderContent);
}

#[test]
fn errors_when_no_json_can_be_found() {
    let error =
        JsonRepairer::extract_metadata_from_text("sin estructura util").expect_err("should fail");

    assert_eq!(error, JsonRepairError::JsonNotFound);
}