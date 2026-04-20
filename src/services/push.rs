use atomic_web_push::engine::general_purpose::URL_SAFE_NO_PAD as ATOMIC_URL_SAFE_NO_PAD;
use atomic_web_push::{
    ContentEncoding, PartialVapidSignatureBuilder, ReqwestWebPushClient, SubscriptionInfo, Urgency,
    VapidSignatureBuilder, WebPushClient, WebPushError, WebPushMessageBuilder,
};
use base64::Engine as _;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use std::time::Duration;
use utoipa::ToSchema;

use crate::config::AppConfig;
use crate::errors::AppError;
use crate::repositories::{
    PushSubscriptionRecord, PushSubscriptionRepository, RegisterPushSubscriptionRecord,
};
use crate::AppState;

const DEFAULT_PUSH_TTL_SECONDS: u32 = 300;
const FALLBACK_VAPID_SUBJECT: &str = "https://kamples.com";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, ToSchema, Default)]
#[serde(rename_all = "lowercase")]
pub enum PushSubscriptionPlatform {
    #[default]
    Web,
    Android,
    Desktop,
}

impl PushSubscriptionPlatform {
    pub const fn as_db_str(self) -> &'static str {
        match self {
            Self::Web => "web",
            Self::Android => "android",
            Self::Desktop => "desktop",
        }
    }
}

#[derive(Debug, Clone)]
pub struct PushNotificationPayload {
    pub title: String,
    pub body: String,
    pub data: serde_json::Value,
    pub tag: Option<String>,
    pub icon_url: Option<String>,
    pub badge_url: Option<String>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct PushSendSummary {
    pub attempted: usize,
    pub delivered: usize,
    pub deactivated: usize,
}

#[derive(Clone)]
pub struct PushDeliveryRuntime {
    client: ReqwestWebPushClient,
    vapid_builder: PartialVapidSignatureBuilder,
    public_key: String,
    subject: String,
}

#[derive(Debug, thiserror::Error)]
pub enum PushDeliveryRuntimeError {
    #[error("Configuración VAPID incompleta: define ambas variables o ninguna ({0})")]
    PartialConfig(&'static str),
    #[error("La clave privada VAPID es inválida: {0}")]
    InvalidPrivateKey(String),
    #[error("La clave pública VAPID no coincide con la derivada desde la privada")]
    PublicKeyMismatch,
    #[error("No se pudo inicializar el cliente Web Push: {0}")]
    ClientInitialization(String),
}

enum PushSendFailure {
    ExpiredEndpoint,
    Other(String),
}

pub struct PushNotificationService;

/* [174A-75] Runtime y servicio Web Push VAPID.
 * - La inicialización deriva/valida la clave pública a partir de la privada.
 * - El registro conserva el upsert por endpoint del legado.
 * - El envío invalida endpoints 404/410 equivalentes (`EndpointNotFound/NotValid`).
 * - El fanout notify(user, event) queda para 174A-78, pero la pieza de canal
 *   ya queda lista y reusable desde mensajes, likes o notificaciones sistema. */

impl PushDeliveryRuntime {
    pub fn from_config(config: &AppConfig) -> Result<Option<Self>, PushDeliveryRuntimeError> {
        let private_key = if let Some(value) = config.vapid_private_key.as_ref() {
            value.trim()
        } else {
            if config.vapid_public_key.is_some() || config.vapid_subject.is_some() {
                return Err(PushDeliveryRuntimeError::PartialConfig(
                    "VAPID_PRIVATE_KEY/KAMPLES_VAPID_PRIVATE_KEY",
                ));
            }
            return Ok(None);
        };

        let vapid_builder =
            VapidSignatureBuilder::from_base64_no_sub(private_key, ATOMIC_URL_SAFE_NO_PAD)
                .map_err(|error| PushDeliveryRuntimeError::InvalidPrivateKey(error.to_string()))?;
        let derived_public_key =
            base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(vapid_builder.get_public_key());

        if let Some(configured_public_key) = config.vapid_public_key.as_ref() {
            if configured_public_key.trim() != derived_public_key {
                return Err(PushDeliveryRuntimeError::PublicKeyMismatch);
            }
        }

        let client = reqwest::Client::builder()
            .timeout(Duration::from_secs(5))
            .build()
            .map(ReqwestWebPushClient::from)
            .map_err(|error| PushDeliveryRuntimeError::ClientInitialization(error.to_string()))?;

        Ok(Some(Self {
            client,
            vapid_builder,
            public_key: derived_public_key,
            subject: config
                .vapid_subject
                .clone()
                .or_else(|| config.public_base_url.clone())
                .unwrap_or_else(|| FALLBACK_VAPID_SUBJECT.to_string()),
        }))
    }

    pub fn public_key(&self) -> &str {
        &self.public_key
    }

    pub fn subject(&self) -> &str {
        &self.subject
    }
}

impl PushNotificationService {
    pub async fn subscribe(
        pool: &PgPool,
        user_id: i32,
        endpoint: &str,
        p256dh: &str,
        auth: &str,
        platform: PushSubscriptionPlatform,
    ) -> Result<(), AppError> {
        validate_subscription(endpoint, p256dh, auth)?;

        PushSubscriptionRepository::upsert(
            pool,
            RegisterPushSubscriptionRecord {
                user_id,
                endpoint: endpoint.trim(),
                p256dh: p256dh.trim(),
                auth: auth.trim(),
                platform: platform.as_db_str(),
            },
        )
        .await
    }

    pub async fn unsubscribe(
        pool: &PgPool,
        user_id: i32,
        endpoint: &str,
    ) -> Result<bool, AppError> {
        validate_endpoint(endpoint)?;
        PushSubscriptionRepository::delete_for_user_by_endpoint(pool, user_id, endpoint.trim())
            .await
    }

    pub async fn send_to_user(
        state: &AppState,
        user_id: i32,
        payload: PushNotificationPayload,
    ) -> Result<PushSendSummary, AppError> {
        let Some(runtime) = state.push_runtime.as_ref() else {
            tracing::debug!(user_id, "web push deshabilitado por configuración ausente");
            return Ok(PushSendSummary::default());
        };

        let subscriptions =
            PushSubscriptionRepository::list_active_by_user(&state.pool, user_id).await?;
        if subscriptions.is_empty() {
            return Ok(PushSendSummary::default());
        }

        let payload_bytes =
            serde_json::to_vec(&build_delivery_payload(payload)).map_err(|error| {
                AppError::Internal(format!("No se pudo serializar el payload push: {error}"))
            })?;

        let mut summary = PushSendSummary::default();
        for subscription in subscriptions {
            summary.attempted += 1;
            let endpoint = subscription.endpoint.clone();
            let platform = subscription.platform.clone();

            match send_to_subscription(runtime, &subscription, &payload_bytes).await {
                Ok(()) => {
                    summary.delivered += 1;
                }
                Err(PushSendFailure::ExpiredEndpoint) => {
                    PushSubscriptionRepository::mark_inactive(&state.pool, &endpoint).await?;
                    summary.deactivated += 1;
                    tracing::info!(
                        user_id,
                        platform,
                        endpoint = %truncate_endpoint(&endpoint),
                        "endpoint push expirado y desactivado"
                    );
                }
                Err(PushSendFailure::Other(error)) => {
                    tracing::warn!(
                        user_id,
                        platform,
                        endpoint = %truncate_endpoint(&endpoint),
                        error = %error,
                        "falló el envío web push"
                    );
                }
            }
        }

        Ok(summary)
    }
}

async fn send_to_subscription(
    runtime: &PushDeliveryRuntime,
    subscription: &PushSubscriptionRecord,
    payload_bytes: &[u8],
) -> Result<(), PushSendFailure> {
    let subscription_info = SubscriptionInfo::new(
        subscription.endpoint.clone(),
        subscription.p256dh.clone(),
        subscription.auth.clone(),
    );

    let mut message_builder = WebPushMessageBuilder::new(&subscription_info);
    message_builder.set_payload(ContentEncoding::Aes128Gcm, payload_bytes);
    message_builder.set_ttl(DEFAULT_PUSH_TTL_SECONDS);
    message_builder.set_urgency(Urgency::Normal);

    let mut signature_builder = runtime
        .vapid_builder
        .clone()
        .add_sub_info(&subscription_info);
    signature_builder.add_claim("sub", runtime.subject().to_string());
    let signature = signature_builder
        .build()
        .map_err(|error| PushSendFailure::Other(error.to_string()))?;
    message_builder.set_vapid_signature(signature);

    let message = message_builder
        .build()
        .map_err(|error| PushSendFailure::Other(error.to_string()))?;

    runtime
        .client
        .send(message)
        .await
        .map_err(map_web_push_error)
}

fn build_delivery_payload(payload: PushNotificationPayload) -> serde_json::Value {
    let mut body = serde_json::Map::new();
    body.insert("title".into(), serde_json::Value::String(payload.title));
    body.insert("body".into(), serde_json::Value::String(payload.body));
    body.insert("data".into(), normalize_payload_data(payload.data));

    if let Some(tag) = payload
        .tag
        .map(|value| value.trim().to_string())
        .filter(|value| !value.is_empty())
    {
        body.insert("tag".into(), serde_json::Value::String(tag));
    }
    if let Some(icon_url) = payload
        .icon_url
        .map(|value| value.trim().to_string())
        .filter(|value| !value.is_empty())
    {
        body.insert("icon".into(), serde_json::Value::String(icon_url));
    }
    if let Some(badge_url) = payload
        .badge_url
        .map(|value| value.trim().to_string())
        .filter(|value| !value.is_empty())
    {
        body.insert("badge".into(), serde_json::Value::String(badge_url));
    }

    serde_json::Value::Object(body)
}

fn normalize_payload_data(data: serde_json::Value) -> serde_json::Value {
    if data.is_null() {
        serde_json::json!({})
    } else {
        data
    }
}

fn validate_subscription(endpoint: &str, p256dh: &str, auth: &str) -> Result<(), AppError> {
    if endpoint.trim().is_empty() || p256dh.trim().is_empty() || auth.trim().is_empty() {
        return Err(AppError::BadRequest(
            "Campos endpoint, keys.p256dh y keys.auth son obligatorios".into(),
        ));
    }

    validate_endpoint(endpoint)
}

fn validate_endpoint(endpoint: &str) -> Result<(), AppError> {
    let endpoint = endpoint.trim();
    if endpoint.is_empty() {
        return Err(AppError::BadRequest("Campo endpoint es obligatorio".into()));
    }

    let parsed = url::Url::parse(endpoint)
        .map_err(|_| AppError::BadRequest("Endpoint debe ser una URL HTTPS válida".into()))?;
    if parsed.scheme() != "https" {
        return Err(AppError::BadRequest(
            "Endpoint debe ser una URL HTTPS válida".into(),
        ));
    }

    Ok(())
}

fn map_web_push_error(error: WebPushError) -> PushSendFailure {
    match error {
        WebPushError::EndpointNotFound(_) | WebPushError::EndpointNotValid(_) => {
            PushSendFailure::ExpiredEndpoint
        }
        other => PushSendFailure::Other(other.to_string()),
    }
}

fn truncate_endpoint(endpoint: &str) -> String {
    endpoint.chars().take(80).collect()
}

#[cfg(test)]
mod tests {
    use super::{normalize_payload_data, validate_endpoint, PushDeliveryRuntime};
    use crate::config::AppConfig;

    #[test]
    fn null_payload_becomes_object() {
        assert_eq!(
            normalize_payload_data(serde_json::Value::Null),
            serde_json::json!({})
        );
    }

    #[test]
    fn endpoint_requires_https() {
        assert!(validate_endpoint("https://push.example.test/subscription").is_ok());
        assert!(validate_endpoint("http://push.example.test/subscription").is_err());
        assert!(validate_endpoint("notaurl").is_err());
    }

    #[test]
    fn runtime_derives_public_key_from_private_key() {
        let config = AppConfig {
            database_url: "postgres://unused".into(),
            redis_url: None,
            jwt_secret: "secret".into(),
            host: "127.0.0.1".into(),
            port: 3000,
            db_max_connections: 10,
            db_min_connections: 2,
            google_client_ids: Vec::new(),
            storage_root: "./uploads".into(),
            storage_backend: "local".into(),
            s3_bucket: None,
            s3_endpoint_url: None,
            public_base_url: Some("https://kamples.com".into()),
            ws_secret: "secret".into(),
            ws_public_url: None,
            ws_ticket_ttl_secs: 60,
            vapid_public_key: None,
            vapid_private_key: Some("IQ9Ur0ykXoHS9gzfYX0aBjy9lvdrjx_PFUXmie9YRcY".into()),
            vapid_subject: None,
            fcm_service_account_json: None,
            smtp: None,
            scraper_secret: None,
            stripe: crate::config::StripeConfig::default(),
        };

        let runtime = PushDeliveryRuntime::from_config(&config)
            .expect("runtime válido")
            .expect("runtime habilitado");

        assert!(!runtime.public_key().is_empty());
        assert_eq!(runtime.subject(), "https://kamples.com");
    }
}
