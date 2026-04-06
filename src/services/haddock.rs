/* [064A-5] Servicio de sincronización unidireccional con Haddock POS API.
 * Envía ventas a Haddock cuando se crean o actualizan en nuestra plataforma.
 * API: https://pos-api.haddock.app (Basic Auth, POST /orders/, POST /catalog/)
 * Limitaciones: sin DELETE, sin GET, sin webhooks — solo push. */

use reqwest::Client;
use serde::Serialize;
use tracing::{info, warn};

use crate::models::{ConfiguracionRestaurante, Venta};

const HADDOCK_API_BASE: &str = "https://pos-api.haddock.app";
const MAX_RETRIES: u32 = 3;

/* Estructuras que replica el schema OpenAPI de Haddock */

#[derive(Debug, Serialize)]
struct HaddockOrdersPayload {
    orders: Vec<HaddockOrder>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct HaddockOrder {
    external_id: String,
    date: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    seats: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    channel: Option<String>,
    payments: Vec<HaddockPayment>,
    items: Vec<HaddockItem>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct HaddockPayment {
    method: String,
    base: f64,
    tax: f64,
    total: f64,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct HaddockItem {
    external_id: String,
    name: String,
    quantity: i32,
    total: f64,
    price_per_unit: f64,
}

pub struct HaddockService;

impl HaddockService {
    /// Sincroniza una venta con Haddock. Se ejecuta en background (`tokio::spawn`).
    /// No falla la operación principal — solo loguea errores.
    pub async fn sync_order(venta: &Venta, config: &ConfiguracionRestaurante) {
        if !config.haddock_sync_enabled || config.haddock_api_token.is_empty() {
            return;
        }

        let order = Self::map_venta_to_order(venta);
        let payload = HaddockOrdersPayload {
            orders: vec![order],
        };

        let token = &config.haddock_api_token;
        let url = format!("{HADDOCK_API_BASE}/orders/");

        /* Retry con backoff exponencial: 1s, 2s, 4s */
        for attempt in 0..MAX_RETRIES {
            match Self::send_request(&url, token, &payload).await {
                Ok(status) if status.is_success() => {
                    info!(
                        "[064A-5] Venta {} sincronizada con Haddock (intento {})",
                        venta.id,
                        attempt + 1
                    );
                    return;
                }
                Ok(status) => {
                    warn!(
                        "[064A-5] Haddock respondió {} para venta {} (intento {})",
                        status,
                        venta.id,
                        attempt + 1
                    );
                }
                Err(e) => {
                    warn!(
                        "[064A-5] Error de red sincronizando venta {} con Haddock (intento {}): {e}",
                        venta.id,
                        attempt + 1
                    );
                }
            }

            if attempt < MAX_RETRIES - 1 {
                let delay = std::time::Duration::from_secs(1 << attempt);
                tokio::time::sleep(delay).await;
            }
        }

        warn!(
            "[064A-5] Fallo definitivo sincronizando venta {} con Haddock después de {MAX_RETRIES} intentos",
            venta.id
        );
    }

    /// Mapea una Venta de nuestra plataforma al formato que espera Haddock
    fn map_venta_to_order(venta: &Venta) -> HaddockOrder {
        let total = Self::decimal_to_f64(&venta.importe_base) + Self::decimal_to_f64(&venta.importe_iva);

        /* Hora estimada por turno (Haddock requiere ISO 8601 con hora) */
        let hora = match venta.turno.as_str() {
            "manana" => "09:00:00",
            "mediodia" => "14:00:00",
            _ => "21:00:00",
        };
        let date = format!("{}T{}Z", venta.fecha, hora);

        HaddockOrder {
            external_id: venta.id.to_string(),
            date,
            seats: venta.comensales,
            channel: Some(Self::map_canal(&venta.canal)),
            payments: vec![HaddockPayment {
                method: Self::map_metodo_pago(&venta.metodo_pago),
                base: Self::decimal_to_f64(&venta.importe_base),
                tax: Self::decimal_to_f64(&venta.importe_iva),
                total,
            }],
            /* items es obligatorio en Haddock — enviamos un ítem genérico
             * porque nuestra plataforma no desglosa artículos individuales */
            items: vec![HaddockItem {
                external_id: format!("venta-{}", venta.id),
                name: if venta.descripcion.is_empty() {
                    "Servicio de mesa".to_string()
                } else {
                    venta.descripcion.clone()
                },
                quantity: 1,
                total,
                price_per_unit: total,
            }],
        }
    }

    /* Mapeo de canales: nuestro enum → valores que acepta Haddock */
    fn map_canal(canal: &str) -> String {
        match canal {
            "comedor" => "dining-room",
            "barra" => "bar",
            "terraza" => "terrace",
            "delivery" => "delivery",
            "just_eat" => "justeat",
            "eventos" => "events",
            _ => "any",
        }
        .to_string()
    }

    /* Mapeo de métodos de pago: nuestro enum → valores que acepta Haddock */
    fn map_metodo_pago(metodo: &str) -> String {
        match metodo {
            "efectivo" => "cash",
            "tarjeta" => "card",
            "transferencia" => "transfer",
            _ => "any",
        }
        .to_string()
    }

    fn decimal_to_f64(d: &rust_decimal::Decimal) -> f64 {
        use std::str::FromStr;
        f64::from_str(&d.to_string()).unwrap_or(0.0)
    }

    async fn send_request(
        url: &str,
        token: &str,
        payload: &HaddockOrdersPayload,
    ) -> Result<reqwest::StatusCode, reqwest::Error> {
        let client = Client::new();
        let resp = client
            .post(url)
            .header("Authorization", format!("Basic {token}"))
            .header("Content-Type", "application/json")
            .json(payload)
            .send()
            .await?;
        Ok(resp.status())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{NaiveDate, Utc};
    use rust_decimal::Decimal;
    use std::str::FromStr;
    use uuid::Uuid;

    fn mock_venta() -> Venta {
        Venta {
            id: Uuid::new_v4(),
            user_id: Uuid::new_v4(),
            fecha: NaiveDate::from_ymd_opt(2026, 4, 6).unwrap(),
            comensales: Some(4),
            descripcion: "Menú del día".to_string(),
            iva_porcentaje: Decimal::from(10),
            turno: "mediodia".to_string(),
            canal: "comedor".to_string(),
            metodo_pago: "tarjeta".to_string(),
            importe_base: Decimal::from_str("45.00").unwrap(),
            importe_iva: Decimal::from_str("4.50").unwrap(),
            reserva_id: None,
            cliente_id: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }

    #[test]
    fn test_map_venta_to_order() {
        let venta = mock_venta();
        let order = HaddockService::map_venta_to_order(&venta);

        assert_eq!(order.external_id, venta.id.to_string());
        assert!(order.date.contains("14:00:00"));
        assert_eq!(order.seats, Some(4));
        assert_eq!(order.channel.as_deref(), Some("dining-room"));
        assert_eq!(order.payments.len(), 1);
        assert_eq!(order.payments[0].method, "card");
        assert!((order.payments[0].base - 45.0).abs() < 0.01);
        assert!((order.payments[0].tax - 4.50).abs() < 0.01);
        assert!((order.payments[0].total - 49.50).abs() < 0.01);
        assert_eq!(order.items.len(), 1);
        assert_eq!(order.items[0].name, "Menú del día");
    }

    #[test]
    fn test_map_canales() {
        assert_eq!(HaddockService::map_canal("comedor"), "dining-room");
        assert_eq!(HaddockService::map_canal("barra"), "bar");
        assert_eq!(HaddockService::map_canal("terraza"), "terrace");
        assert_eq!(HaddockService::map_canal("delivery"), "delivery");
        assert_eq!(HaddockService::map_canal("just_eat"), "justeat");
        assert_eq!(HaddockService::map_canal("eventos"), "events");
        assert_eq!(HaddockService::map_canal("desconocido"), "any");
    }

    #[test]
    fn test_map_metodos_pago() {
        assert_eq!(HaddockService::map_metodo_pago("efectivo"), "cash");
        assert_eq!(HaddockService::map_metodo_pago("tarjeta"), "card");
        assert_eq!(HaddockService::map_metodo_pago("transferencia"), "transfer");
        assert_eq!(HaddockService::map_metodo_pago("otro"), "any");
    }
}
