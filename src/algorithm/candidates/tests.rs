/* [174A-51] Tests del SelectorCandidatos. Las queries reales se cubren vía
 * compile-time `query!` (sqlx prepare) + tests de integración del recomendador.
 * Aquí validamos sólo lógica pura: configuración, whitelist y cache en memoria. */

use super::*;

#[test]
fn legacy_defaults_match_php_constants() {
    let cfg = CandidatesConfig::legacy_defaults();
    assert_eq!(cfg.max_trending, 300);
    assert_eq!(cfg.max_embedding, 200);
    assert_eq!(cfg.max_seguidos, 200);
    assert_eq!(cfg.max_tags, 200);
    assert_eq!(cfg.max_populares, 100);
    assert_eq!(cfg.max_nuevos, 150);
    assert_eq!(cfg.dias_trending, 14);
}

#[test]
fn safe_dias_trending_falls_back_to_14_for_unknown_value() {
    let cfg = CandidatesConfig {
        dias_trending: 21,
        ..CandidatesConfig::default()
    };
    assert_eq!(cfg.safe_dias_trending(), 14);
}

#[test]
fn safe_dias_trending_keeps_whitelisted_values() {
    for &dias in &[7, 14, 30, 60, 90] {
        let cfg = CandidatesConfig {
            dias_trending: dias,
            ..CandidatesConfig::default()
        };
        assert_eq!(cfg.safe_dias_trending(), dias);
    }
}

#[tokio::test]
async fn invalidate_count_without_redis_is_safe() {
    /* Asegura que el cleanup en memoria no panicquea cuando la entrada no existe. */
    MEMORY_COUNT_CACHE.insert(
        COUNT_CACHE_KEY.to_owned(),
        (42, Instant::now() + Duration::from_secs(60)),
    );
    CandidatesService::invalidate_count(&None)
        .await
        .expect("invalidate sin redis no debe fallar");
    assert!(MEMORY_COUNT_CACHE.get(COUNT_CACHE_KEY).is_none());
}
