/* [174A-63] Tokens HMAC firmados temporales para descargas/streaming.
 *
 * Formato (URL-safe base64): "{sample_id}:{user_id}:{exp}:{hex(hmac_sha256)}"
 * - sample_id, user_id: i32
 * - exp: epoch seconds (i64)
 * - firma: HMAC-SHA256 sobre "sample_id:user_id:exp" con jwt_secret
 *
 * El handler stream acepta un solo parámetro `?token=...` y verifica firma + exp.
 * Equivalente a `DescargasStreamController::generarFirmaDescarga` del legado PHP.
 */

use crate::errors::AppError;
use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine};
use hmac::{Hmac, Mac};
use sha2::Sha256;
use std::time::{SystemTime, UNIX_EPOCH};

type HmacSha256 = Hmac<Sha256>;

const DEFAULT_TTL_SECS: i64 = 300; // 5 min

fn now_unix() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| i64::try_from(d.as_secs()).unwrap_or(i64::MAX))
        .unwrap_or(0)
}

fn sign(payload: &str, secret: &str) -> String {
    let mut mac = HmacSha256::new_from_slice(secret.as_bytes())
        .expect("HMAC acepta cualquier longitud de key");
    mac.update(payload.as_bytes());
    hex::encode(mac.finalize().into_bytes())
}

/// Genera un token firmado válido por `ttl_secs` segundos (5 min por defecto si <=0).
pub fn generate(sample_id: i32, user_id: i32, ttl_secs: i64, secret: &str) -> String {
    let ttl = if ttl_secs > 0 { ttl_secs } else { DEFAULT_TTL_SECS };
    let exp = now_unix().saturating_add(ttl);
    let payload = format!("{sample_id}:{user_id}:{exp}");
    let sig = sign(&payload, secret);
    let raw = format!("{payload}:{sig}");
    URL_SAFE_NO_PAD.encode(raw.as_bytes())
}

/// Verifica un token y devuelve `(sample_id, user_id)` si es válido y no expiró.
pub fn verify(token: &str, secret: &str) -> Result<(i32, i32), AppError> {
    let raw = URL_SAFE_NO_PAD
        .decode(token.as_bytes())
        .map_err(|_| AppError::Unauthorized)?;
    let raw = String::from_utf8(raw).map_err(|_| AppError::Unauthorized)?;
    let parts: Vec<&str> = raw.split(':').collect();
    if parts.len() != 4 {
        return Err(AppError::Unauthorized);
    }
    let sample_id: i32 = parts[0].parse().map_err(|_| AppError::Unauthorized)?;
    let user_id: i32 = parts[1].parse().map_err(|_| AppError::Unauthorized)?;
    let exp: i64 = parts[2].parse().map_err(|_| AppError::Unauthorized)?;
    if now_unix() > exp {
        return Err(AppError::Forbidden("token de descarga expirado".into()));
    }
    let payload = format!("{sample_id}:{user_id}:{exp}");
    let expected = sign(&payload, secret);
    /* Comparación constante para evitar timing attacks */
    if !constant_time_eq(expected.as_bytes(), parts[3].as_bytes()) {
        return Err(AppError::Unauthorized);
    }
    Ok((sample_id, user_id))
}

fn constant_time_eq(a: &[u8], b: &[u8]) -> bool {
    if a.len() != b.len() {
        return false;
    }
    let mut diff: u8 = 0;
    for (x, y) in a.iter().zip(b.iter()) {
        diff |= x ^ y;
    }
    diff == 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generate_and_verify_round_trip() {
        let token = generate(42, 7, 60, "secret-key");
        let (sid, uid) = verify(&token, "secret-key").expect("valido");
        assert_eq!(sid, 42);
        assert_eq!(uid, 7);
    }

    #[test]
    fn verify_rejects_wrong_secret() {
        let token = generate(1, 2, 60, "secret-A");
        assert!(verify(&token, "secret-B").is_err());
    }

    #[test]
    fn verify_rejects_expired() {
        let token = generate(1, 2, -10, "k"); // ttl<=0 → fallback 300s, no se puede simular fácil
        // Construyo manualmente uno expirado:
        let payload = format!("1:2:{}", now_unix() - 10);
        let sig = sign(&payload, "k");
        let raw = format!("{payload}:{sig}");
        let token_exp = URL_SAFE_NO_PAD.encode(raw.as_bytes());
        assert!(verify(&token_exp, "k").is_err());
        // El generado con ttl<=0 debe ser válido (fallback 300s)
        assert!(verify(&token, "k").is_ok());
    }

    #[test]
    fn verify_rejects_garbage() {
        assert!(verify("not-base64!!", "k").is_err());
        assert!(verify("YWJjZGVm", "k").is_err()); // base64 válido pero formato malo
    }
}
