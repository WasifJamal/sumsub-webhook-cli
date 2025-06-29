//! HMAC-SHA256 signature generation for webhook payloads
use hmac::{Hmac, Mac};
use sha2::Sha256;
use std::collections::HashMap;

type HmacSha256 = Hmac<Sha256>;

/// Signs the payload using the provided secret and returns HTTP headers
pub fn sign_payload(secret: &str, payload: &str) -> HashMap<String, String> {
    let mut mac =
        HmacSha256::new_from_slice(secret.as_bytes()).expect("HMAC can take key of any size");
    mac.update(payload.as_bytes());
    let digest = hex::encode(mac.finalize().into_bytes());

    let mut headers = HashMap::new();
    headers.insert("x-payload-digest-alg".into(), "HMAC_SHA256_HEX".into());
    headers.insert("x-payload-digest".into(), digest);
    headers.insert("content-type".into(), "application/json".into());
    headers
}
