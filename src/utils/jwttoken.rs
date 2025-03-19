use serde_json;
use crate::utils::hmac::compute_hmac;
use crate::database::structs::User;
use crate::utils::decodificar::decode_base64;

const KEY: &[u8] = b"minha_chave_secreta";

pub fn verify_jwt_token(token: &str) -> Result<User, String> {
    // Espera que o token esteja no formato "header.payload.signature"
    let parts: Vec<&str> = token.split('.').collect();
    if parts.len() != 3 {
        return Err("Token inválido".to_string());
    }
    let header_b64 = parts[0];
    let payload_b64 = parts[1];
    let signature = parts[2];

    // Recalcula a assinatura a partir do header e payload
    let token_almost = format!("{}.{}", header_b64, payload_b64);
    let computed_signature = compute_hmac(KEY, token_almost.as_bytes())
        .map_err(|e| format!("Erro ao computar HMAC: {}", e))?;

    if computed_signature != signature {
        return Err("Token inválido".to_string());
    }

    // Decodifica o payload utilizando a implementação customizada
    let payload_str = decode_base64(payload_b64);
    let user: User = serde_json::from_str(&payload_str)
        .map_err(|e| format!("Erro ao parsear JSON: {}", e))?;

    Ok(user)
}
