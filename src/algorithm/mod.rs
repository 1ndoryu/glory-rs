/* [174A-49] Algoritmo de descubrimiento del feed.
 * La fase actual arranca por `signals.rs`: pesos, breakdown y fórmulas puras de
 * las 6 señales del legado vigente. Perfil, candidatos y recomendador llegan en
 * las tareas siguientes para evitar acoplar el módulo demasiado pronto. */

pub mod signals;
