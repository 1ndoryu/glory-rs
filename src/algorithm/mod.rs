/* [174A-49] Algoritmo de descubrimiento del feed.
 * La fase actual arranca por `signals.rs`: pesos, breakdown y fórmulas puras de
 * las 6 señales del legado vigente. Perfil, candidatos y recomendador llegan en
 * las tareas siguientes para evitar acoplar el módulo demasiado pronto. */

/* [174A-50] Perfil de usuario para recomendación: BPM/key/scale/tipo/creadores
 * favoritos + géneros declarados, con cache TTL 30min (Redis o memoria). */

/* [174A-51] Selector de candidatos: pre-filtra ~1000 IDs vía 6 fuentes
 * (trending, embedding ANN, seguidos, top-tags, populares, no reproducidos)
 * con filtro bidireccional de bloqueos. */

/* [174A-52] MotorRecomendacion: orquesta perfil + candidatos + scoring
 * (signals) + diversidad + cache stale-while-revalidate + warm async +
 * `similar_to_sample`. */

/* [174A-53] PrecomputeService: vista materializada `mv_trending_samples`
 * que pre-calcula likes/repro/descargas/follows recientes para alimentar
 * `signals.tendencias` en O(1) por sample. */

/* [174A-54] TagAffinityService: pre-calcula afinidad tag↔usuario en
 * `user_tag_scores` para que el scoring lea con un JOIN indexado en lugar
 * de hacer UNNEST + 7 JOINs por request. */

pub mod candidates;
pub mod precompute;
pub mod profile;
pub mod recommender;
pub mod signals;
pub mod tag_affinity;
