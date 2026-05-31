// src/utils/wireguard.rs
use base64::{Engine, engine::general_purpose::STANDARD};
use x25519_dalek::{PublicKey, StaticSecret};

pub fn public_key_from_secret(secret_b64: &str) -> Result<String, String> {
    let bytes = STANDARD
        .decode(secret_b64)
        .map_err(|e| format!("base64 decode error: {e}"))?;

    let arr: [u8; 32] = bytes
        .try_into()
        .map_err(|_| "secret key must be 32 bytes".to_string())?;

    let secret = StaticSecret::from(arr);
    let public = PublicKey::from(&secret);

    Ok(STANDARD.encode(public.as_bytes()))
}

/// Следующий свободный IP в подсети из существующих allowedIPs пиров
pub fn next_peer_ip(existing_ips: &[String]) -> Option<String> {
    let mut used: std::collections::HashSet<u32> = std::collections::HashSet::new();

    for ip_cidr in existing_ips {
        let ip_str = ip_cidr.split('/').next()?;
        let octets: Vec<u8> = ip_str.split('.').filter_map(|o| o.parse().ok()).collect();
        if octets.len() == 4 {
            let n = u32::from_be_bytes([octets[0], octets[1], octets[2], octets[3]]);
            used.insert(n);
        }
    }

    // берём базу из первого IP или дефолт 10.0.0.0
    let base: u32 = 0x0A000000; // 10.0.0.0
    for i in 2u32..254 {
        let candidate = base | i;
        if !used.contains(&candidate) {
            let b = candidate.to_be_bytes();
            return Some(format!("{}.{}.{}.{}", b[0], b[1], b[2], b[3]));
        }
    }

    None
}
