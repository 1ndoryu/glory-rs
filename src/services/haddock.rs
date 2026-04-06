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
        let url = format!("{HADDOCK_API_BASE}/orders/");
        Self::sync_order_to_url(venta, config, &url).await;
    }

    /* [064A-5] Núcleo de sincronización con URL inyectable para testing.
     * En producción se llama vía sync_order() con la URL real de Haddock.
     * En tests se llama directamente con la URL del mock server. */
    async fn sync_order_to_url(venta: &Venta, config: &ConfiguracionRestaurante, url: &str) {
        if !config.haddock_sync_enabled || config.haddock_api_token.is_empty() {
            return;
        }

        let order = Self::map_venta_to_order(venta);
        let payload = HaddockOrdersPayload {
            orders: vec![order],
        };

        let token = &config.haddock_api_token;

        /* Retry con backoff exponencial: 1s, 2s, 4s */
        for attempt in 0..MAX_RETRIES {
            match Self::send_request(url, token, &payload).await {
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
    use chrono::{NaiveDate, NaiveTime, Utc};
    use rust_decimal::Decimal;
    use std::str::FromStr;
    use uuid::Uuid;
    use wiremock::{
        matchers::{body_json, header, method, path},
        Mock, MockServer, ResponseTemplate,
    };

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

    fn mock_config(token: &str, enabled: bool) -> ConfiguracionRestaurante {
        ConfiguracionRestaurante {
            id: Uuid::new_v4(),
            user_id: Uuid::new_v4(),
            reserva_email_obligatorio: false,
            reserva_telefono_obligatorio: true,
            reserva_nombre_obligatorio: true,
            reserva_apellidos_obligatorio: false,
            iva_por_defecto: Decimal::from(10),
            nombre_restaurante: "Test".to_string(),
            groq_api_key: None,
            auto_venta_reserva: false,
            hora_desayuno_inicio: NaiveTime::from_hms_opt(0, 0, 0).unwrap(),
            hora_desayuno_fin: NaiveTime::from_hms_opt(12, 0, 0).unwrap(),
            hora_comida_inicio: NaiveTime::from_hms_opt(12, 0, 0).unwrap(),
            hora_comida_fin: NaiveTime::from_hms_opt(18, 0, 0).unwrap(),
            hora_cena_inicio: NaiveTime::from_hms_opt(18, 0, 0).unwrap(),
            hora_cena_fin: NaiveTime::from_hms_opt(23, 59, 59).unwrap(),
            url_haddock: String::new(),
            haddock_api_token: token.to_string(),
            haddock_sync_enabled: enabled,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }

    /* ── Tests de mapeo de datos (unitarios) ────────────────────── */

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

    #[test]
    fn test_empty_description_fallback() {
        let mut venta = mock_venta();
        venta.descripcion = String::new();
        let order = HaddockService::map_venta_to_order(&venta);
        assert_eq!(order.items[0].name, "Servicio de mesa");
    }

    #[test]
    fn test_zero_amounts_auto_venta() {
        let mut venta = mock_venta();
        venta.importe_base = Decimal::ZERO;
        venta.importe_iva = Decimal::ZERO;
        let order = HaddockService::map_venta_to_order(&venta);
        assert!((order.payments[0].total).abs() < 0.001);
        assert!((order.items[0].total).abs() < 0.001);
    }

    #[test]
    fn test_turno_hora_mapping() {
        let mut venta = mock_venta();
        venta.turno = "manana".to_string();
        let order = HaddockService::map_venta_to_order(&venta);
        assert!(order.date.contains("09:00:00"));

        venta.turno = "noche".to_string();
        let order = HaddockService::map_venta_to_order(&venta);
        assert!(order.date.contains("21:00:00"));

        /* Turno desconocido → fallback a noche */
        venta.turno = "inventado".to_string();
        let order = HaddockService::map_venta_to_order(&venta);
        assert!(order.date.contains("21:00:00"));
    }

    #[test]
    fn test_date_format_iso8601() {
        let venta = mock_venta();
        let order = HaddockService::map_venta_to_order(&venta);
        /* Formato esperado: YYYY-MM-DDTHH:MM:SSZ */
        assert_eq!(order.date, "2026-04-06T14:00:00Z");
    }

    #[test]
    fn test_external_id_is_uuid() {
        let venta = mock_venta();
        let order = HaddockService::map_venta_to_order(&venta);
        /* Verificar que external_id es un UUID válido */
        assert!(Uuid::parse_str(&order.external_id).is_ok());
    }

    /* ── Tests de integración con mock server HTTP ──────────────── */

    #[tokio::test]
    async fn test_sync_disabled_no_request() {
        let server = MockServer::start().await;
        /* No registramos ningún mock — si se hace request, el test falla */
        let venta = mock_venta();
        let config = mock_config("dG9rZW46c2VjcmV0", false);
        let url = format!("{}/orders/", server.uri());

        HaddockService::sync_order_to_url(&venta, &config, &url).await;

        /* Verificar que NO se hizo ningún request */
        let received = server.received_requests().await.unwrap();
        assert!(received.is_empty(), "No debería enviar requests con sync desactivado");
    }

    #[tokio::test]
    async fn test_empty_token_no_request() {
        let server = MockServer::start().await;
        let venta = mock_venta();
        let config = mock_config("", true);
        let url = format!("{}/orders/", server.uri());

        HaddockService::sync_order_to_url(&venta, &config, &url).await;

        let received = server.received_requests().await.unwrap();
        assert!(received.is_empty(), "No debería enviar requests con token vacío");
    }

    #[tokio::test]
    async fn test_both_disabled_and_empty_no_request() {
        let server = MockServer::start().await;
        let venta = mock_venta();
        let config = mock_config("", false);
        let url = format!("{}/orders/", server.uri());

        HaddockService::sync_order_to_url(&venta, &config, &url).await;

        let received = server.received_requests().await.unwrap();
        assert!(received.is_empty(), "No debería enviar requests con ambos desactivados");
    }

    #[tokio::test]
    async fn test_sync_success_200() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/orders/"))
            .respond_with(ResponseTemplate::new(200).set_body_string("{}"))
            .expect(1)
            .mount(&server)
            .await;

        let venta = mock_venta();
        let config = mock_config("dG9rZW46c2VjcmV0", true);
        let url = format!("{}/orders/", server.uri());

        HaddockService::sync_order_to_url(&venta, &config, &url).await;
        /* wiremock verifica automáticamente que se recibió exactamente 1 request */
    }

    #[tokio::test]
    async fn test_authorization_header_format() {
        let token = "dG9rZW46c2VjcmV0";
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/orders/"))
            .and(header("Authorization", &format!("Basic {token}")))
            .respond_with(ResponseTemplate::new(200))
            .expect(1)
            .mount(&server)
            .await;

        let venta = mock_venta();
        let config = mock_config(token, true);
        let url = format!("{}/orders/", server.uri());

        HaddockService::sync_order_to_url(&venta, &config, &url).await;
    }

    #[tokio::test]
    async fn test_content_type_json() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/orders/"))
            .and(header("Content-Type", "application/json"))
            .respond_with(ResponseTemplate::new(200))
            .expect(1)
            .mount(&server)
            .await;

        let venta = mock_venta();
        let config = mock_config("dG9rZW46c2VjcmV0", true);
        let url = format!("{}/orders/", server.uri());

        HaddockService::sync_order_to_url(&venta, &config, &url).await;
    }

    #[tokio::test]
    async fn test_401_unauthorized_retries_3_times() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/orders/"))
            .respond_with(ResponseTemplate::new(401).set_body_string(r#"{"error":"Invalid credentials"}"#))
            .expect(3)
            .mount(&server)
            .await;

        let venta = mock_venta();
        let config = mock_config("token-malo", true);
        let url = format!("{}/orders/", server.uri());

        HaddockService::sync_order_to_url(&venta, &config, &url).await;
        /* wiremock verifica que hubo exactamente 3 intentos */
    }

    #[tokio::test]
    async fn test_500_server_error_retries_3_times() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/orders/"))
            .respond_with(ResponseTemplate::new(500).set_body_string("Internal Server Error"))
            .expect(3)
            .mount(&server)
            .await;

        let venta = mock_venta();
        let config = mock_config("dG9rZW46c2VjcmV0", true);
        let url = format!("{}/orders/", server.uri());

        HaddockService::sync_order_to_url(&venta, &config, &url).await;
    }

    #[tokio::test]
    async fn test_success_on_second_retry() {
        let server = MockServer::start().await;
        /* Primer intento falla con 500, segundo tiene éxito */
        Mock::given(method("POST"))
            .and(path("/orders/"))
            .respond_with(ResponseTemplate::new(500))
            .up_to_n_times(1)
            .expect(1)
            .mount(&server)
            .await;

        Mock::given(method("POST"))
            .and(path("/orders/"))
            .respond_with(ResponseTemplate::new(200))
            .expect(1)
            .mount(&server)
            .await;

        let venta = mock_venta();
        let config = mock_config("dG9rZW46c2VjcmV0", true);
        let url = format!("{}/orders/", server.uri());

        HaddockService::sync_order_to_url(&venta, &config, &url).await;
        /* Total: 2 requests (1 fallo + 1 éxito) */
    }

    #[tokio::test]
    async fn test_network_error_no_panic() {
        /* Apuntar a un puerto cerrado — genera error de conexión */
        let venta = mock_venta();
        let config = mock_config("dG9rZW46c2VjcmV0", true);
        let url = "http://127.0.0.1:1/orders/";

        /* Esto NO debe hacer panic — solo loguear el error */
        HaddockService::sync_order_to_url(&venta, &config, url).await;
    }

    #[tokio::test]
    async fn test_payload_json_structure() {
        let server = MockServer::start().await;
        let venta = mock_venta();
        let order = HaddockService::map_venta_to_order(&venta);
        let expected_payload = HaddockOrdersPayload {
            orders: vec![order],
        };

        Mock::given(method("POST"))
            .and(path("/orders/"))
            .and(body_json(&expected_payload))
            .respond_with(ResponseTemplate::new(200))
            .expect(1)
            .mount(&server)
            .await;

        let config = mock_config("dG9rZW46c2VjcmV0", true);
        let url = format!("{}/orders/", server.uri());

        HaddockService::sync_order_to_url(&venta, &config, &url).await;
    }

    #[tokio::test]
    async fn test_json_has_camel_case_keys() {
        let venta = mock_venta();
        let order = HaddockService::map_venta_to_order(&venta);
        let payload = HaddockOrdersPayload {
            orders: vec![order],
        };
        let json = serde_json::to_string(&payload).unwrap();

        /* Haddock espera camelCase, no snake_case */
        assert!(json.contains("externalId"), "Debe usar camelCase: externalId");
        assert!(json.contains("pricePerUnit"), "Debe usar camelCase: pricePerUnit");
        assert!(!json.contains("external_id"), "No debe usar snake_case");
        assert!(!json.contains("price_per_unit"), "No debe usar snake_case");
    }

    #[tokio::test]
    async fn test_venta_sin_comensales() {
        let mut venta = mock_venta();
        venta.comensales = None;
        let order = HaddockService::map_venta_to_order(&venta);
        assert!(order.seats.is_none());

        /* Verificar que seats no aparece en JSON (skip_serializing_if) */
        let payload = HaddockOrdersPayload {
            orders: vec![order],
        };
        let json = serde_json::to_string(&payload).unwrap();
        assert!(!json.contains("seats"), "seats=null no debería serializarse");
    }

    #[tokio::test]
    async fn test_large_amounts() {
        let mut venta = mock_venta();
        venta.importe_base = Decimal::from_str("99999.99").unwrap();
        venta.importe_iva = Decimal::from_str("21000.00").unwrap();
        let order = HaddockService::map_venta_to_order(&venta);
        assert!((order.payments[0].base - 99_999.99).abs() < 0.01);
        assert!((order.payments[0].total - 120_999.99).abs() < 0.01);
    }
}
