/* [174A-57] AlgoTimingLogger — métricas finas del algoritmo de feed.
 *
 * Reemplazo Rust del `AlgoTimingLogger.php` legado. Diferencias clave:
 *   - Estado in-process via `RwLock<VecDeque<TimingEntry>>` (no WP options).
 *   - Marcas activas por request via `DashMap<i32, RequestState>` (un usuario
 *     puede tener varios requests en vuelo; cada uno se identifica por user_id
 *     pero el state es el último iniciado — aceptable para QA single-user).
 *   - Solo activo para `target_user_id` (default 1, configurable via env
 *     `KAMPLES_ALGO_TIMING_USER_ID`). Zero overhead para el resto.
 *   - EXPLAIN ANALYZE NO portado (requiere pgvector compatible + cost extra
 *     que no justifica scope inicial). Hook documentado para futuro.
 *
 * Uso desde `RecommenderService::feed`:
 *   ALGO_TIMING.start(user_id);
 *   ... cache lookup ...
 *   ALGO_TIMING.mark(user_id, "cache_lookup");
 *   ... compute ...
 *   ALGO_TIMING.mark(user_id, "compute");
 *   ALGO_TIMING.save(user_id, json!({"total_samples": n}));
 *
 * Endpoint admin: `GET /api/admin/algo-timing` retorna las últimas 100 medidas. */

use std::collections::VecDeque;
use std::sync::{LazyLock, RwLock};
use std::time::Instant;

use chrono::{DateTime, Utc};
use dashmap::DashMap;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use utoipa::ToSchema;

const MAX_HISTORY: usize = 100;
const ENV_TARGET_USER: &str = "KAMPLES_ALGO_TIMING_USER_ID";
const DEFAULT_TARGET_USER: i32 = 1;

/// Una medición completa de un request al feed.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct TimingEntry {
    pub ts: DateTime<Utc>,
    pub total_ms: f64,
    /// Etapas en orden de inserción con duración relativa a la marca anterior.
    pub etapas: Vec<TimingStage>,
    pub meta: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct TimingStage {
    pub name: String,
    pub ms: f64,
}

struct RequestState {
    started: Instant,
    last_mark: Instant,
    etapas: Vec<TimingStage>,
}

pub struct AlgoTimingLogger {
    target_user_id: i32,
    in_flight: DashMap<i32, RequestState>,
    history: RwLock<VecDeque<TimingEntry>>,
}

impl AlgoTimingLogger {
    fn new() -> Self {
        let target = std::env::var(ENV_TARGET_USER)
            .ok()
            .and_then(|s| s.parse::<i32>().ok())
            .unwrap_or(DEFAULT_TARGET_USER);
        Self {
            target_user_id: target,
            in_flight: DashMap::new(),
            history: RwLock::new(VecDeque::with_capacity(MAX_HISTORY)),
        }
    }

    #[inline]
    fn matches(&self, user_id: i32) -> bool {
        user_id == self.target_user_id
    }

    /// Inicia un ciclo de medición para `user_id`. No-op si no es el target.
    pub fn start(&self, user_id: i32) {
        if !self.matches(user_id) {
            return;
        }
        let now = Instant::now();
        self.in_flight.insert(
            user_id,
            RequestState {
                started: now,
                last_mark: now,
                etapas: Vec::new(),
            },
        );
    }

    /// Registra una marca con duración relativa a la anterior.
    pub fn mark(&self, user_id: i32, etapa: &str) {
        if !self.matches(user_id) {
            return;
        }
        if let Some(mut state) = self.in_flight.get_mut(&user_id) {
            let now = Instant::now();
            let delta_ms = now.duration_since(state.last_mark).as_secs_f64() * 1000.0;
            state.etapas.push(TimingStage {
                name: etapa.to_string(),
                ms: round2(delta_ms),
            });
            state.last_mark = now;
        }
    }

    /// Cierra la medición y la guarda en el historial circular.
    pub fn save(&self, user_id: i32, meta: Value) {
        if !self.matches(user_id) {
            return;
        }
        let Some((_, state)) = self.in_flight.remove(&user_id) else {
            return;
        };
        let total_ms = round2(state.started.elapsed().as_secs_f64() * 1000.0);
        let entry = TimingEntry {
            ts: Utc::now(),
            total_ms,
            etapas: state.etapas,
            meta,
        };
        if let Ok(mut hist) = self.history.write() {
            if hist.len() >= MAX_HISTORY {
                hist.pop_back();
            }
            hist.push_front(entry);
        }
    }

    /// Snapshot del historial (más reciente primero). Limit clamp a MAX_HISTORY.
    #[must_use]
    pub fn history(&self, limit: usize) -> Vec<TimingEntry> {
        let cap = limit.min(MAX_HISTORY);
        self.history.read().map_or_else(
            |_| Vec::new(),
            |hist| hist.iter().take(cap).cloned().collect(),
        )
    }

    #[must_use]
    pub const fn target_user_id(&self) -> i32 {
        self.target_user_id
    }
}

#[inline]
fn round2(value: f64) -> f64 {
    (value * 100.0).round() / 100.0
}

/// Singleton global. Se instancia 1× via `LazyLock` (lectura del env happens
/// en la primera llamada). Compartido por todos los handlers + recommender.
pub static ALGO_TIMING: LazyLock<AlgoTimingLogger> = LazyLock::new(AlgoTimingLogger::new);

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn fresh_logger(target: i32) -> AlgoTimingLogger {
        AlgoTimingLogger {
            target_user_id: target,
            in_flight: DashMap::new(),
            history: RwLock::new(VecDeque::with_capacity(MAX_HISTORY)),
        }
    }

    #[test]
    fn ignores_non_target_user() {
        let logger = fresh_logger(1);
        logger.start(99);
        logger.mark(99, "x");
        logger.save(99, json!({}));
        assert!(logger.history(10).is_empty());
    }

    #[test]
    fn records_target_user_cycle() {
        let logger = fresh_logger(7);
        logger.start(7);
        std::thread::sleep(std::time::Duration::from_millis(2));
        logger.mark(7, "step1");
        logger.save(7, json!({"k": 1}));
        let hist = logger.history(10);
        assert_eq!(hist.len(), 1);
        assert!(hist[0].total_ms >= 2.0);
        assert_eq!(hist[0].etapas.len(), 1);
        assert_eq!(hist[0].etapas[0].name, "step1");
        assert_eq!(hist[0].meta, json!({"k": 1}));
    }

    #[test]
    fn history_is_capped_and_ordered() {
        let logger = fresh_logger(7);
        for i in 0..(MAX_HISTORY + 5) {
            logger.start(7);
            logger.save(7, json!({"i": i}));
        }
        let hist = logger.history(MAX_HISTORY + 50);
        assert_eq!(hist.len(), MAX_HISTORY);
        /* La más reciente al frente. */
        assert_eq!(hist[0].meta["i"], json!(MAX_HISTORY + 4));
    }
}
