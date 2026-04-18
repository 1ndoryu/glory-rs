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

pub mod candidates;
pub mod profile;
pub mod recommender;
pub mod signals;
