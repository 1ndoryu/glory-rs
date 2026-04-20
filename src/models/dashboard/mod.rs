use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct CreatorDashboardStats {
    pub ingresos_total: f64,
    pub ingresos_mes: f64,
    pub ingresos_anterior: f64,
    pub descargas_total: i64,
    pub descargas_mes: i64,
    pub reproducciones_total: i64,
    pub reproducciones_mes: i64,
    pub seguidores_total: i64,
    pub seguidores_nuevos_mes: i64,
    pub samples_publicados: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct CreatorDashboardSampleStat {
    pub id: i32,
    pub titulo: String,
    pub slug: String,
    pub descargas: i64,
    pub reproducciones: i64,
    pub likes: i64,
    pub ingresos: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "lowercase")]
pub enum CreatorDashboardTransactionType {
    Descarga,
    Venta,
    Suscripcion,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct CreatorDashboardTransaction {
    pub id: i32,
    pub fecha: DateTime<Utc>,
    pub tipo: CreatorDashboardTransactionType,
    pub sample: String,
    pub comprador: String,
    pub monto: f64,
    pub comision: f64,
    pub neto: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct CreatorDashboardIncomePoint {
    pub fecha: NaiveDate,
    pub monto: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "lowercase")]
pub enum CreatorDashboardIncomePeriod {
    Semana,
    Mes,
    Anio,
}

impl CreatorDashboardIncomePeriod {
    pub const fn days(self) -> i32 {
        match self {
            Self::Semana => 7,
            Self::Mes => 30,
            Self::Anio => 365,
        }
    }
}

#[derive(Debug, Clone, Deserialize, IntoParams, ToSchema)]
pub struct CreatorDashboardTransactionsQuery {
    pub page: Option<i64>,
}

impl CreatorDashboardTransactionsQuery {
    pub fn page(&self) -> i64 {
        self.page.unwrap_or(1).max(1)
    }
}

#[derive(Debug, Clone, Deserialize, IntoParams, ToSchema)]
pub struct CreatorDashboardIncomeQuery {
    pub periodo: Option<CreatorDashboardIncomePeriod>,
}

impl CreatorDashboardIncomeQuery {
    pub fn period(&self) -> CreatorDashboardIncomePeriod {
        self.periodo.unwrap_or(CreatorDashboardIncomePeriod::Mes)
    }
}