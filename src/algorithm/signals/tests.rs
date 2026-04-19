use super::{
    behavior_signal_score, content_similarity_score, context_signal_score, novelty_signal_score,
    social_signal_score, trend_signal_score, AlgorithmSignalConfig, AlgorithmSignalInput,
    BehaviorSignalInput, ContextSignalInput, SignalParameters, SocialSignalInput, TrendNormalizers,
    TrendSignalInput,
};

fn approx_eq(left: f64, right: f64) {
    assert!((left - right).abs() < 0.0001, "left={left}, right={right}");
}

#[test]
fn legacy_current_config_matches_php_defaults() {
    let config = AlgorithmSignalConfig::legacy_current();

    approx_eq(config.senales.similitud_contenido, 0.28);
    approx_eq(config.senales.comportamiento, 0.27);
    approx_eq(config.senales.contexto, 0.15);
    approx_eq(config.senales.tendencias, 0.12);
    approx_eq(config.senales.grafo_social, 0.10);
    approx_eq(config.senales.novedad, 0.0);
    approx_eq(config.senales.total_weight(), 0.92);
}

#[test]
fn content_similarity_maps_cosine_distance_to_unit_range() {
    approx_eq(content_similarity_score(None), 0.0);
    approx_eq(content_similarity_score(Some(0.0)), 1.0);
    approx_eq(content_similarity_score(Some(1.0)), 0.5);
    approx_eq(content_similarity_score(Some(2.0)), 0.0);
    approx_eq(content_similarity_score(Some(3.5)), 0.0);
}

#[test]
fn behavior_score_applies_subweights_and_dislike_penalty() {
    let score = behavior_signal_score(
        AlgorithmSignalConfig::default().comportamiento_detalle,
        BehaviorSignalInput {
            likes_dados: 1.0,
            reproducciones: 1.0,
            tiempo_escucha: 1.0,
            descargas: 0.0,
            completadas: 0.0,
            dislike_penalty: 0.15,
        },
    );

    approx_eq(score, 0.60);
}

#[test]
fn context_score_respects_thematic_bias_over_technical_data() {
    let score = context_signal_score(
        AlgorithmSignalConfig::default().contexto_detalle,
        SignalParameters::legacy_current(),
        ContextSignalInput {
            bpm_candidato: Some(130.0),
            bpm_promedio_usuario: Some(140.0),
            key_match: Some(true),
            escala_match: Some(false),
            genero_match: 1.0,
            tipo_match: None,
            creador_afin: true,
        },
    );

    approx_eq(score, 0.871_666_666_7);
}

#[test]
fn trend_score_clamps_each_subfactor_with_absolute_normalizers() {
    let score = trend_signal_score(
        AlgorithmSignalConfig::default().tendencias_detalle,
        TrendNormalizers::legacy_current(),
        TrendSignalInput {
            likes_24h: 30.0,
            reproducciones_24h: 90.0,
            descargas_7d: 40.0,
            follows_creador_7d: 20.0,
        },
    );

    approx_eq(score, 1.0);
}

#[test]
fn social_score_caps_followed_reactions() {
    let score = social_signal_score(
        AlgorithmSignalConfig::default().grafo_social_detalle,
        SocialSignalInput {
            creador_seguido: false,
            puntos_reacciones_seguidos: 10.0,
        },
    );

    approx_eq(score, 0.4);
}

#[test]
fn novelty_score_decays_logarithmically_and_hits_zero_at_window_end() {
    approx_eq(novelty_signal_score(Some(1.0), 14.0), 1.0);
    approx_eq(novelty_signal_score(Some(7.0), 14.0), 0.262_649_535_0);
    approx_eq(novelty_signal_score(Some(14.0), 14.0), 0.0);
    approx_eq(novelty_signal_score(Some(30.0), 14.0), 0.0);
}

#[test]
fn config_score_returns_weighted_breakdown() {
    let breakdown = AlgorithmSignalConfig::default().score(AlgorithmSignalInput {
        distancia_coseno_contenido: Some(0.4),
        comportamiento: BehaviorSignalInput {
            likes_dados: 0.8,
            reproducciones: 0.4,
            tiempo_escucha: 0.9,
            descargas: 0.5,
            completadas: 0.7,
            dislike_penalty: 0.05,
        },
        contexto: ContextSignalInput {
            bpm_candidato: Some(144.0),
            bpm_promedio_usuario: Some(140.0),
            key_match: Some(true),
            escala_match: Some(true),
            genero_match: 0.75,
            tipo_match: Some(false),
            creador_afin: true,
        },
        tendencias: TrendSignalInput {
            likes_24h: 6.0,
            reproducciones_24h: 12.0,
            descargas_7d: 5.0,
            follows_creador_7d: 2.0,
        },
        grafo_social: SocialSignalInput {
            creador_seguido: true,
            puntos_reacciones_seguidos: 3.0,
        },
        dias_desde_publicacion: Some(3.0),
    });

    approx_eq(breakdown.similitud_contenido.raw, 0.8);
    approx_eq(breakdown.comportamiento.raw, 0.615);
    approx_eq(breakdown.contexto.raw, 0.833_666_666_7);
    approx_eq(breakdown.tendencias.raw, 0.35);
    approx_eq(breakdown.grafo_social.raw, 0.9);
    approx_eq(breakdown.novedad.raw, 0.583_710_336_1);
    approx_eq(breakdown.total, 0.6471);
}
