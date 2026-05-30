use prost::Message;

#[derive(Clone, prost::Message)]
struct GeoSiteList {
    #[prost(message, repeated, tag = "1")]
    entry: Vec<GeoSite>,
}

#[derive(Clone, prost::Message)]
struct GeoSite {
    #[prost(string, tag = "1")]
    country_code: String,
    #[prost(message, repeated, tag = "2")]
    domain: Vec<Domain>,
}

#[derive(Clone, prost::Message)]
struct Domain {
    #[prost(int32, tag = "1")]
    r#type: i32,
    #[prost(string, tag = "2")]
    value: String,
}

#[derive(Clone, prost::Message)]
struct GeoIPList {
    #[prost(message, repeated, tag = "1")]
    entry: Vec<GeoIP>,
}

#[derive(Clone, prost::Message)]
struct GeoIP {
    #[prost(string, tag = "1")]
    country_code: String,
    #[prost(message, repeated, tag = "2")]
    cidr: Vec<Cidr>,
}

#[derive(Clone, prost::Message)]
struct Cidr {
    #[prost(bytes = "vec", tag = "1")]
    ip: Vec<u8>,
    #[prost(uint32, tag = "2")]
    prefix: u32,
}

// --- публичный интерфейс ---

pub enum GeoEntry {
    Domain(String), // "example.com"
    Ip(String),     // "1.2.3.4/24" или "1.2.3.4"
}

pub enum GeoError {
    InvalidIp(String),
}

impl std::fmt::Display for GeoError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            GeoError::InvalidIp(s) => write!(f, "invalid IP: {}", s),
        }
    }
}

/// Генерирует geosite.dat из списка доменов
pub fn build_geosite(tag: &str, entries: &[GeoEntry]) -> Vec<u8> {
    let domains = entries
        .iter()
        .filter_map(|e| match e {
            GeoEntry::Domain(d) => Some(Domain {
                r#type: 2, // Domain — домен + поддомены
                value: d.clone(),
            }),
            _ => None,
        })
        .collect();

    GeoSiteList {
        entry: vec![GeoSite {
            country_code: tag.to_uppercase(),
            domain: domains,
        }],
    }
    .encode_to_vec()
}

/// Генерирует geoip.dat из списка IP/CIDR
pub fn build_geoip(tag: &str, entries: &[GeoEntry]) -> Result<Vec<u8>, GeoError> {
    let mut cidrs = Vec::new();

    for e in entries {
        let GeoEntry::Ip(raw) = e else { continue };

        let (ip_str, prefix) = if let Some((ip, prefix)) = raw.split_once('/') {
            let p = prefix
                .parse::<u32>()
                .map_err(|_| GeoError::InvalidIp(raw.clone()))?;
            (ip, p)
        } else {
            let p = if raw.contains(':') { 128 } else { 32 };
            (raw.as_str(), p)
        };

        let ip_bytes = parse_ip(ip_str).ok_or_else(|| GeoError::InvalidIp(raw.clone()))?;

        cidrs.push(Cidr {
            ip: ip_bytes,
            prefix,
        });
    }

    Ok(GeoIPList {
        entry: vec![GeoIP {
            country_code: tag.to_uppercase(),
            cidr: cidrs,
        }],
    }
    .encode_to_vec())
}

fn parse_ip(s: &str) -> Option<Vec<u8>> {
    if let Ok(addr) = s.parse::<std::net::Ipv4Addr>() {
        return Some(addr.octets().to_vec());
    }
    if let Ok(addr) = s.parse::<std::net::Ipv6Addr>() {
        return Some(addr.octets().to_vec());
    }
    None
}
