/* [174A-49] Núcleo puro de las 6 señales del algoritmo de descubrimiento.
 * Esta capa no conoce SQL ni repositorios: solo fija pesos, inputs y fórmulas
 * para que `profile`, `candidates` y `recommender` puedan reutilizar la misma
 * lógica sin volver a hardcodear constantes del legado. */

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct WeightedSignalScore {
    pub raw: f64,
    pub weighted: f64,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SignalScoreBreakdown {
    pub similitud_contenido: WeightedSignalScore,
    pub comportamiento: WeightedSignalScore,
    pub contexto: WeightedSignalScore,
    pub tendencias: WeightedSignalScore,
    pub grafo_social: WeightedSignalScore,
    pub novedad: WeightedSignalScore,
    pub total: f64,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct AlgorithmSignalWeights {
    pub similitud_contenido: f64,
    pub comportamiento: f64,
    pub contexto: f64,
    pub tendencias: f64,
    pub grafo_social: f64,
    pub novedad: f64,
}

impl AlgorithmSignalWeights {
    pub const fn legacy_current() -> Self {
        Self {
            similitud_contenido: 0.28,
            comportamiento: 0.27,
            contexto: 0.15,
            tendencias: 0.12,
            grafo_social: 0.10,
            novedad: 0.0,
        }
    }

    pub fn total_weight(self) -> f64 {
        self.similitud_contenido
            + self.comportamiento
            + self.contexto
            + self.tendencias
            + self.grafo_social
            + self.novedad
    }
}

impl Default for AlgorithmSignalWeights {
    fn default() -> Self {
        Self::legacy_current()
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BehaviorSignalWeights {
    pub likes_dados: f64,
    pub reproducciones: f64,
    pub tiempo_escucha: f64,
    pub descargas: f64,
    pub completadas: f64,
}

impl BehaviorSignalWeights {
    pub const fn legacy_current() -> Self {
        Self {
            likes_dados: 0.30,
            reproducciones: 0.25,
            tiempo_escucha: 0.20,
            descargas: 0.15,
            completadas: 0.10,
        }
    }
}

impl Default for BehaviorSignalWeights {
    fn default() -> Self {
        Self::legacy_current()
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ContextSignalWeights {
    pub bpm_proximidad: f64,
    pub key_match: f64,
    pub escala_match: f64,
    pub genero_match: f64,
    pub tipo_match: f64,
    pub creador_afin: f64,
}

impl ContextSignalWeights {
    pub const fn legacy_current() -> Self {
        Self {
            bpm_proximidad: 0.08,
            key_match: 0.06,
            escala_match: 0.04,
            genero_match: 0.30,
            tipo_match: 0.07,
            creador_afin: 0.45,
        }
    }
}

impl Default for ContextSignalWeights {
    fn default() -> Self {
        Self::legacy_current()
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TrendSignalWeights {
    pub likes_24h: f64,
    pub reproducciones_24h: f64,
    pub descargas_7d: f64,
    pub follows_creador_7d: f64,
}

impl TrendSignalWeights {
    pub const fn legacy_current() -> Self {
        Self {
            likes_24h: 0.40,
            reproducciones_24h: 0.30,
            descargas_7d: 0.20,
            follows_creador_7d: 0.10,
        }
    }
}

impl Default for TrendSignalWeights {
    fn default() -> Self {
        Self::legacy_current()
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TrendNormalizers {
    pub max_likes_ventana_corta: f64,
    pub max_repro_ventana_corta: f64,
    pub max_descargas_ventana_media: f64,
    pub max_follows_ventana_media: f64,
}

impl TrendNormalizers {
    pub const fn legacy_current() -> Self {
        Self {
            max_likes_ventana_corta: 15.0,
            max_repro_ventana_corta: 30.0,
            max_descargas_ventana_media: 20.0,
            max_follows_ventana_media: 10.0,
        }
    }
}

impl Default for TrendNormalizers {
    fn default() -> Self {
        Self::legacy_current()
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SocialSignalWeights {
    pub creador_seguido: f64,
    pub likeado_por_seguidos: f64,
}

impl SocialSignalWeights {
    pub const fn legacy_current() -> Self {
        Self {
            creador_seguido: 0.60,
            likeado_por_seguidos: 0.40,
        }
    }
}

impl Default for SocialSignalWeights {
    fn default() -> Self {
        Self::legacy_current()
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SignalParameters {
    pub bpm_tolerancia: f64,
    pub novedad_dias_boost: f64,
}

impl SignalParameters {
    pub const fn legacy_current() -> Self {
        Self {
            bpm_tolerancia: 15.0,
            novedad_dias_boost: 14.0,
        }
    }
}

impl Default for SignalParameters {
    fn default() -> Self {
        Self::legacy_current()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct BehaviorSignalInput {
    pub likes_dados: f64,
    pub reproducciones: f64,
    pub tiempo_escucha: f64,
    pub descargas: f64,
    pub completadas: f64,
    pub dislike_penalty: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct ContextSignalInput {
    pub bpm_candidato: Option<f64>,
    pub bpm_promedio_usuario: Option<f64>,
    pub key_match: Option<bool>,
    pub escala_match: Option<bool>,
    pub genero_match: f64,
    pub tipo_match: Option<bool>,
    pub creador_afin: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct TrendSignalInput {
    pub likes_24h: f64,
    pub reproducciones_24h: f64,
    pub descargas_7d: f64,
    pub follows_creador_7d: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct SocialSignalInput {
    pub creador_seguido: bool,
    pub puntos_reacciones_seguidos: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct AlgorithmSignalInput {
    pub distancia_coseno_contenido: Option<f64>,
    pub comportamiento: BehaviorSignalInput,
    pub contexto: ContextSignalInput,
    pub tendencias: TrendSignalInput,
    pub grafo_social: SocialSignalInput,
    pub dias_desde_publicacion: Option<f64>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct AlgorithmSignalConfig {
    pub senales: AlgorithmSignalWeights,
    pub comportamiento_detalle: BehaviorSignalWeights,
    pub contexto_detalle: ContextSignalWeights,
    pub tendencias_detalle: TrendSignalWeights,
    pub tendencias_normalizadores: TrendNormalizers,
    pub grafo_social_detalle: SocialSignalWeights,
    pub parametros: SignalParameters,
}

impl AlgorithmSignalConfig {
    pub const fn legacy_current() -> Self {
        Self {
            senales: AlgorithmSignalWeights::legacy_current(),
            comportamiento_detalle: BehaviorSignalWeights::legacy_current(),
            contexto_detalle: ContextSignalWeights::legacy_current(),
            tendencias_detalle: TrendSignalWeights::legacy_current(),
            tendencias_normalizadores: TrendNormalizers::legacy_current(),
            grafo_social_detalle: SocialSignalWeights::legacy_current(),
            parametros: SignalParameters::legacy_current(),
        }
    }

    pub fn score(self, input: AlgorithmSignalInput) -> SignalScoreBreakdown {
        let similitud_contenido = content_similarity_score(input.distancia_coseno_contenido);
        let comportamiento = behavior_signal_score(self.comportamiento_detalle, input.comportamiento);
        let contexto = context_signal_score(self.contexto_detalle, self.parametros, input.contexto);
        let tendencias = trend_signal_score(
            self.tendencias_detalle,
            self.tendencias_normalizadores,
            input.tendencias,
        );
        let grafo_social = social_signal_score(self.grafo_social_detalle, input.grafo_social);
        let novedad = novelty_signal_score(
            input.dias_desde_publicacion,
            self.parametros.novedad_dias_boost,
        );

        let similitud_contenido = weighted_score(self.senales.similitud_contenido, similitud_contenido);
        let comportamiento = weighted_score(self.senales.comportamiento, comportamiento);
        let contexto = weighted_score(self.senales.contexto, contexto);
        let tendencias = weighted_score(self.senales.tendencias, tendencias);
        let grafo_social = weighted_score(self.senales.grafo_social, grafo_social);
        let novedad = weighted_score(self.senales.novedad, novedad);
        let total = similitud_contenido.weighted
            + comportamiento.weighted
            + contexto.weighted
            + tendencias.weighted
            + grafo_social.weighted
            + novedad.weighted;

        SignalScoreBreakdown {
            similitud_contenido,
            comportamiento,
            contexto,
            tendencias,
            grafo_social,
            novedad,
            total,
        }
    }
}

impl Default for AlgorithmSignalConfig {
    fn default() -> Self {
        Self::legacy_current()
    }
}

pub fn content_similarity_score(distancia_coseno: Option<f64>) -> f64 {
    let Some(distancia_coseno) = distancia_coseno else {
        return 0.0;
    };

    clamp_unit(1.0 - distancia_coseno / 2.0)
}

pub fn behavior_signal_score(weights: BehaviorSignalWeights, input: BehaviorSignalInput) -> f64 {
    let positive = weights.likes_dados * clamp_unit(input.likes_dados)
        + weights.reproducciones * clamp_unit(input.reproducciones)
        + weights.tiempo_escucha * clamp_unit(input.tiempo_escucha)
        + weights.descargas * clamp_unit(input.descargas)
        + weights.completadas * clamp_unit(input.completadas);

    clamp_unit((positive - input.dislike_penalty).max(0.0))
}

pub fn context_signal_score(
    weights: ContextSignalWeights,
    parameters: SignalParameters,
    input: ContextSignalInput,
) -> f64 {
    let bpm = match (input.bpm_promedio_usuario, input.bpm_candidato) {
        (Some(bpm_usuario), bpm_candidato) => {
            let bpm_candidato = bpm_candidato.unwrap_or(0.0);
            clamp_unit(
                (parameters.bpm_tolerancia - (bpm_candidato - bpm_usuario).abs())
                    / parameters.bpm_tolerancia,
            )
        }
        (None, _) => 0.5,
    };

    let key = tri_state_match_score(input.key_match);
    let escala = tri_state_match_score(input.escala_match);
    let genero = clamp_unit(input.genero_match);
    let tipo = tri_state_match_score(input.tipo_match);
    let creador = if input.creador_afin { 1.0 } else { 0.0 };

    clamp_unit(
        weights.bpm_proximidad * bpm
            + weights.key_match * key
            + weights.escala_match * escala
            + weights.genero_match * genero
            + weights.tipo_match * tipo
            + weights.creador_afin * creador,
    )
}

pub fn trend_signal_score(
    weights: TrendSignalWeights,
    normalizers: TrendNormalizers,
    input: TrendSignalInput,
) -> f64 {
    let likes = clamp_unit(input.likes_24h.max(0.0) / normalizers.max_likes_ventana_corta);
    let reproducciones =
        clamp_unit(input.reproducciones_24h / normalizers.max_repro_ventana_corta);
    let descargas =
        clamp_unit(input.descargas_7d / normalizers.max_descargas_ventana_media);
    let follows =
        clamp_unit(input.follows_creador_7d / normalizers.max_follows_ventana_media);

    clamp_unit(
        weights.likes_24h * likes
            + weights.reproducciones_24h * reproducciones
            + weights.descargas_7d * descargas
            + weights.follows_creador_7d * follows,
    )
}

pub fn social_signal_score(weights: SocialSignalWeights, input: SocialSignalInput) -> f64 {
    let creador = if input.creador_seguido { 1.0 } else { 0.0 };
    let likeado = clamp_unit(input.puntos_reacciones_seguidos / 4.0);

    clamp_unit(weights.creador_seguido * creador + weights.likeado_por_seguidos * likeado)
}

pub fn novelty_signal_score(dias_desde_publicacion: Option<f64>, dias_boost: f64) -> f64 {
    let Some(dias_desde_publicacion) = dias_desde_publicacion else {
        return 0.0;
    };

    let dias = dias_desde_publicacion.max(1.0);
    clamp_unit(1.0 - dias.ln() / dias_boost.ln())
}

fn weighted_score(weight: f64, raw: f64) -> WeightedSignalScore {
    let raw = clamp_unit(raw);
    WeightedSignalScore {
        raw,
        weighted: weight * raw,
    }
}

fn tri_state_match_score(value: Option<bool>) -> f64 {
    match value {
        Some(true) => 1.0,
        Some(false) => 0.0,
        None => 0.5,
    }
}

fn clamp_unit(value: f64) -> f64 {
    value.clamp(0.0, 1.0)
}

#[cfg(test)]
#[path = "signals/tests.rs"]
mod tests;