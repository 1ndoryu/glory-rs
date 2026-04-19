use serde::{Deserialize, Serialize};
use std::str::FromStr;
use utoipa::ToSchema;

/* [174A-79] Dominio de pagos Kamples.
 * Centraliza el catálogo de planes y el reparto económico para que checkout,
 * webhook y Connect reutilicen la misma semántica del legado sin strings mágicos.
 * Pendiente: cuando 174A-82/174A-83 persistan webhooks y payouts, mover aquí los
 * estados de suscripción/transacción que hoy aún viven en SQL plano. */

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "lowercase")]
pub enum KamplesPlanId {
    Free,
    Pro,
    Premium,
}

impl KamplesPlanId {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Free => "free",
            Self::Pro => "pro",
            Self::Premium => "premium",
        }
    }
}

impl FromStr for KamplesPlanId {
    type Err = ();

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value.trim().to_ascii_lowercase().as_str() {
            "free" => Ok(Self::Free),
            "pro" => Ok(Self::Pro),
            "premium" => Ok(Self::Premium),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
pub struct KamplesPlanConfig {
    pub id: KamplesPlanId,
    pub monthly_price_cents: i64,
    pub downloads_per_day: i32,
    pub uploads_per_month: i32,
    pub max_samples: i32,
    pub transfer_gb: i32,
    pub revenue_share_bps: u16,
    pub free_trial_days: Option<u16>,
    pub trial_downloads: Option<u16>,
}

impl KamplesPlanConfig {
    pub const fn revenue_share_ratio(self) -> f64 {
        self.revenue_share_bps as f64 / 10_000.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
pub struct RevenueShareBreakdown {
    pub price_cents: i64,
    pub creator_payout_cents: i64,
    pub platform_fee_cents: i64,
    pub revenue_share_bps: u16,
}

const PRO_PLAN: KamplesPlanConfig = KamplesPlanConfig {
    id: KamplesPlanId::Pro,
    monthly_price_cents: 500,
    downloads_per_day: 50,
    uploads_per_month: -1,
    max_samples: 20_000,
    transfer_gb: 10,
    revenue_share_bps: 8_000,
    free_trial_days: None,
    trial_downloads: None,
};

const PREMIUM_PLAN: KamplesPlanConfig = KamplesPlanConfig {
    id: KamplesPlanId::Premium,
    monthly_price_cents: 1_999,
    downloads_per_day: -1,
    uploads_per_month: -1,
    max_samples: 20_000,
    transfer_gb: 50,
    revenue_share_bps: 10_000,
    free_trial_days: None,
    trial_downloads: None,
};

const FREE_PLAN: KamplesPlanConfig = KamplesPlanConfig {
    id: KamplesPlanId::Free,
    monthly_price_cents: 0,
    downloads_per_day: 5,
    uploads_per_month: -1,
    max_samples: 100,
    transfer_gb: 1,
    revenue_share_bps: 8_000,
    free_trial_days: Some(30),
    trial_downloads: Some(20),
};

const KAMPLES_PLAN_CATALOG: [KamplesPlanConfig; 3] = [PRO_PLAN, PREMIUM_PLAN, FREE_PLAN];

pub const fn kamples_plan_catalog() -> &'static [KamplesPlanConfig; 3] {
    &KAMPLES_PLAN_CATALOG
}

pub const fn kamples_plan_config(plan: KamplesPlanId) -> &'static KamplesPlanConfig {
    match plan {
        KamplesPlanId::Free => &FREE_PLAN,
        KamplesPlanId::Pro => &PRO_PLAN,
        KamplesPlanId::Premium => &PREMIUM_PLAN,
    }
}

pub fn kamples_plan_config_from_str(plan: &str) -> &'static KamplesPlanConfig {
    plan.parse::<KamplesPlanId>()
        .map(kamples_plan_config)
        .unwrap_or(&FREE_PLAN)
}

pub fn calculate_sample_revenue_share(
    price_cents: i64,
    plan: KamplesPlanId,
) -> RevenueShareBreakdown {
    let config = kamples_plan_config(plan);
    let creator_payout_cents = (price_cents * i64::from(config.revenue_share_bps) + 5_000) / 10_000;
    let platform_fee_cents = price_cents - creator_payout_cents;

    RevenueShareBreakdown {
        price_cents,
        creator_payout_cents,
        platform_fee_cents,
        revenue_share_bps: config.revenue_share_bps,
    }
}

pub fn calculate_subscription_download_revenue_share(plan: KamplesPlanId) -> RevenueShareBreakdown {
    let config = kamples_plan_config(plan);
    let amount_cents = round_div_i64(config.monthly_price_cents, 200);
    calculate_sample_revenue_share(amount_cents, plan)
}

fn round_div_i64(value: i64, divisor: i64) -> i64 {
    if divisor == 0 {
        return 0;
    }

    if value >= 0 {
        (value + (divisor / 2)) / divisor
    } else {
        (value - (divisor / 2)) / divisor
    }
}

pub fn format_price_cents(cents: i64) -> String {
    let sign = if cents < 0 { "-" } else { "" };
    let absolute = cents.abs();
    format!("{sign}{}.{:02}", absolute / 100, absolute % 100)
}

#[cfg(test)]
mod tests {
    use super::{
        calculate_sample_revenue_share, calculate_subscription_download_revenue_share,
        kamples_plan_config, kamples_plan_config_from_str, KamplesPlanId,
    };

    #[test]
    fn free_plan_is_default_for_unknown_values() {
        assert_eq!(
            kamples_plan_config_from_str("enterprise").id,
            KamplesPlanId::Free
        );
    }

    #[test]
    fn premium_receives_full_creator_share() {
        let breakdown = calculate_sample_revenue_share(1_999, KamplesPlanId::Premium);

        assert_eq!(breakdown.creator_payout_cents, 1_999);
        assert_eq!(breakdown.platform_fee_cents, 0);
    }

    #[test]
    fn pro_revenue_share_matches_legacy_ratio() {
        let breakdown = calculate_sample_revenue_share(500, KamplesPlanId::Pro);
        let plan = kamples_plan_config(KamplesPlanId::Pro);

        assert_eq!(plan.revenue_share_bps, 8_000);
        assert_eq!(breakdown.creator_payout_cents, 400);
        assert_eq!(breakdown.platform_fee_cents, 100);
    }

    #[test]
    fn subscription_download_share_rounds_like_legacy_decimal_flow() {
        let breakdown = calculate_subscription_download_revenue_share(KamplesPlanId::Pro);

        assert_eq!(breakdown.price_cents, 3);
        assert_eq!(breakdown.creator_payout_cents, 2);
        assert_eq!(breakdown.platform_fee_cents, 1);
    }

    #[test]
    fn premium_subscription_download_share_keeps_full_creator_cut() {
        let breakdown = calculate_subscription_download_revenue_share(KamplesPlanId::Premium);

        assert_eq!(breakdown.price_cents, 10);
        assert_eq!(breakdown.creator_payout_cents, 10);
        assert_eq!(breakdown.platform_fee_cents, 0);
    }
}
