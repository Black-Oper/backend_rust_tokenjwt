use hmac::{Hmac, Mac};
use sha2::Sha256;

type HmacSha256 = Hmac<Sha256>;

pub fn compute_hmac(key: &[u8], message: &[u8]) -> Result<String, Box<dyn std::error::Error>> {
    let mut mac = HmacSha256::new_from_slice(key)?;
    mac.update(message);
    let result = mac.finalize();
    let code_bytes = result.into_bytes();
    Ok(hex::encode(code_bytes))
}

pub fn verify_hmac(key: &[u8], message: &[u8], expected: &str) -> Result<bool, Box<dyn std::error::Error>> {
    let mut mac = HmacSha256::new_from_slice(key)?;
    mac.update(message);
    let expected_bytes = hex::decode(expected)?;
    match mac.verify_slice(&expected_bytes) {
        Ok(_) => Ok(true),
        Err(_) => Ok(false),
    }
}
