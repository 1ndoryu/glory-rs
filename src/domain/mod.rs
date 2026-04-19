/* [174A-3] Tipos del dominio (newtypes para IDs y value objects).
 * UserId(i64), SampleId(i64), Email(String), etc. Evita primitivos desnudos. */
mod payments;

pub use payments::{
    calculate_sample_revenue_share, calculate_subscription_download_revenue_share,
    format_price_cents, kamples_plan_catalog, kamples_plan_config, kamples_plan_config_from_str,
    KamplesPlanConfig, KamplesPlanId, RevenueShareBreakdown,
};
