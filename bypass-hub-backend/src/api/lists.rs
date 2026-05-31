// src/api/lists.rs
use crate::db::infrastructure::servers as servers_db;
use crate::db::xui::route as routes_db;
use crate::models::route::Route;
use crate::state::AppState;
use crate::utils::geo::{GeoEntry, build_geoip, build_geosite};
use axum::{
    Router,
    extract::{Path, State},
    http::{HeaderMap, HeaderValue, StatusCode, header},
    response::IntoResponse,
    routing::get,
};

async fn geosite(State(state): State<AppState>, Path(uuid): Path<String>) -> impl IntoResponse {
    let server = match servers_db::find_by_uuid(&state.pool, &uuid).await {
        Ok(Some(s)) => s,
        Ok(None) => return StatusCode::NOT_FOUND.into_response(),
        Err(e) => {
            eprintln!("DB error: {e}");
            return StatusCode::INTERNAL_SERVER_ERROR.into_response();
        }
    };

    let routes = match routes_db::find_all(&state.pool).await {
        Ok(r) => r,
        Err(e) => {
            eprintln!("DB error: {e}");
            return StatusCode::INTERNAL_SERVER_ERROR.into_response();
        }
    };

    let entries: Vec<GeoEntry> = routes
        .iter()
        .filter(|r| r.r#type == "domain")
        .map(|r| GeoEntry::Domain(r.value.clone()))
        .collect();

    let bytes = build_geosite("bypass", &entries);

    let mut headers = HeaderMap::new();
    headers.insert(
        header::CONTENT_TYPE,
        HeaderValue::from_static("application/octet-stream"),
    );
    headers.insert(
        header::CONTENT_DISPOSITION,
        HeaderValue::from_static("attachment; filename=\"bypass-site.dat\""),
    );

    (headers, bytes).into_response()
}

async fn geoip(State(state): State<AppState>, Path(uuid): Path<String>) -> impl IntoResponse {
    let server = match servers_db::find_by_uuid(&state.pool, &uuid).await {
        Ok(Some(s)) => s,
        Ok(None) => return StatusCode::NOT_FOUND.into_response(),
        Err(e) => {
            eprintln!("DB error: {e}");
            return StatusCode::INTERNAL_SERVER_ERROR.into_response();
        }
    };

    let routes = match routes_db::find_all(&state.pool).await {
        Ok(r) => r,
        Err(e) => {
            eprintln!("DB error: {e}");
            return StatusCode::INTERNAL_SERVER_ERROR.into_response();
        }
    };

    let entries: Vec<GeoEntry> = routes
        .iter()
        .filter(|r| r.r#type == "ip")
        .map(|r| GeoEntry::Ip(r.value.clone()))
        .collect();

    let bytes = match build_geoip("bypass", &entries) {
        Ok(b) => b,
        Err(e) => {
            eprintln!("GeoIP build error: {e}");
            return StatusCode::INTERNAL_SERVER_ERROR.into_response();
        }
    };

    let mut headers = HeaderMap::new();
    headers.insert(
        header::CONTENT_TYPE,
        HeaderValue::from_static("application/octet-stream"),
    );
    headers.insert(
        header::CONTENT_DISPOSITION,
        HeaderValue::from_static("attachment; filename=\"bypass-ip.dat\""),
    );

    (headers, bytes).into_response()
}

// src/api/lists.rs — добавить хендлер mikrotik_list

async fn mikrotik_list(
    State(state): State<AppState>,
    Path(uuid): Path<String>,
) -> impl IntoResponse {
    match crate::db::infrastructure::mikrotiks::find_by_uuid(&state.pool, &uuid).await {
        Ok(Some(_)) => {}
        Ok(None) => return StatusCode::NOT_FOUND.into_response(),
        Err(e) => {
            eprintln!("DB error: {e}");
            return StatusCode::INTERNAL_SERVER_ERROR.into_response();
        }
    }

    let routes = match crate::db::mikrotik::route::find_all(&state.pool).await {
        Ok(r) => r,
        Err(e) => {
            eprintln!("DB error: {e}");
            return StatusCode::INTERNAL_SERVER_ERROR.into_response();
        }
    };

    let mut script = String::new();

    script.push_str("# bypass-hub vpn list\n");
    script.push_str("# auto-generated, do not edit\n\n");

    script.push_str("/ip firewall address-list remove [find list=to_vpn_list]\n");
    script.push_str("/ip dns static remove [find address-list=to_vpn_list]\n\n");

    let ips: Vec<_> = routes.iter().filter(|r| r.r#type == "ip").collect();
    let domains: Vec<_> = routes.iter().filter(|r| r.r#type == "domain").collect();

    if !ips.is_empty() {
        script.push_str("/ip firewall address-list\n");
        for r in &ips {
            script.push_str(&format!("add list=to_vpn_list address={}\n", r.value));
        }
        script.push('\n');
    }

    if !domains.is_empty() {
        script.push_str("/ip dns static\n");
        for r in &domains {
            script.push_str(&format!(
                "add type=FWD address-list=to_vpn_list match-subdomain=yes name={}\n",
                r.value
            ));
        }
        script.push('\n');
    }

    let mut headers = axum::http::HeaderMap::new();
    headers.insert(
        axum::http::header::CONTENT_TYPE,
        axum::http::HeaderValue::from_static("text/plain; charset=utf-8"),
    );
    headers.insert(
        axum::http::header::CONTENT_DISPOSITION,
        axum::http::HeaderValue::from_static("attachment; filename=\"vpn-list.rsc\""),
    );

    (headers, script).into_response()
}

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/{uuid}/bypass-site.dat", get(geosite))
        .route("/{uuid}/bypass-ip.dat", get(geoip))
        .route("/mikrotik/{uuid}", get(mikrotik_list))
}
