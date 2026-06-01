// src/api/infrastructure/mikrotiks.rs
use crate::db::infrastructure::{mikrotiks as db, servers as servers_db};
use crate::state::AppState;
use crate::utils::wireguard::{next_peer_ip, public_key_from_secret};
use axum::{
    Json, Router,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{delete, get, patch, post},
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;

#[derive(Deserialize)]
struct CreateMikrotikRequest {
    name: String,
    server_id: i64,
    inbound_id: i64,
}

#[derive(Deserialize)]
struct SetKeyRequest {
    public_key: String,
}

async fn list(State(state): State<AppState>) -> impl IntoResponse {
    match db::find_all(&state.pool).await {
        Ok(mikrotiks) => Json(mikrotiks).into_response(),
        Err(e) => {
            eprintln!("DB error: {e}");
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}

async fn create(
    State(state): State<AppState>,
    Json(body): Json<CreateMikrotikRequest>,
) -> impl IntoResponse {
    if body.name.is_empty() {
        return (StatusCode::BAD_REQUEST, "name is required").into_response();
    }

    let uuid = Uuid::new_v4().to_string();

    match db::create(
        &state.pool,
        &body.name,
        body.server_id,
        body.inbound_id,
        &uuid,
    )
    .await
    {
        Ok(id) => (
            StatusCode::CREATED,
            Json(serde_json::json!({"id": id, "uuid": uuid})),
        )
            .into_response(),
        Err(e) => {
            eprintln!("DB error: {e}");
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}

async fn remove(State(state): State<AppState>, Path(id): Path<i64>) -> impl IntoResponse {
    match db::delete(&state.pool, id).await {
        Ok(true) => StatusCode::NO_CONTENT.into_response(),
        Ok(false) => StatusCode::NOT_FOUND.into_response(),
        Err(e) => {
            eprintln!("DB error: {e}");
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}

async fn set_key(
    State(state): State<AppState>,
    Path(id): Path<i64>,
    Json(body): Json<SetKeyRequest>,
) -> impl IntoResponse {
    // сохраняем ключ сразу
    match db::save_public_key(&state.pool, id, &body.public_key).await {
        Ok(true) => {}
        Ok(false) => return StatusCode::NOT_FOUND.into_response(),
        Err(e) => {
            eprintln!("DB error: {e}");
            return StatusCode::INTERNAL_SERVER_ERROR.into_response();
        }
    }

    // пытаемся добавить peer
    match push_peer_to_xui(&state, id).await {
        Ok(assigned_ip) => Json(serde_json::json!({ "assigned_ip": assigned_ip })).into_response(),
        Err(e) => {
            eprintln!("3x-ui error: {e}");
            (
                StatusCode::BAD_GATEWAY,
                "key saved, but 3x-ui unreachable. Use retry later.",
            )
                .into_response()
        }
    }
}

async fn retry(State(state): State<AppState>, Path(id): Path<i64>) -> impl IntoResponse {
    let mikrotik = match db::find_by_id(&state.pool, id).await {
        Ok(Some(m)) => m,
        Ok(None) => return StatusCode::NOT_FOUND.into_response(),
        Err(e) => {
            eprintln!("DB error: {e}");
            return StatusCode::INTERNAL_SERVER_ERROR.into_response();
        }
    };

    if mikrotik.status != "pending_key" {
        return (StatusCode::BAD_REQUEST, "already active").into_response();
    }

    if mikrotik.public_key.is_none() {
        return (StatusCode::BAD_REQUEST, "public key not set").into_response();
    }

    match push_peer_to_xui(&state, id).await {
        Ok(assigned_ip) => Json(serde_json::json!({ "assigned_ip": assigned_ip })).into_response(),
        Err(e) => {
            eprintln!("3x-ui error: {e}");
            (StatusCode::BAD_GATEWAY, "3x-ui unreachable").into_response()
        }
    }
}

async fn push_peer_to_xui(state: &AppState, id: i64) -> Result<String, String> {
    let mikrotik = db::find_by_id(&state.pool, id)
        .await
        .map_err(|e| e.to_string())?
        .ok_or("not found")?;

    let server = servers_db::find_by_id(&state.pool, mikrotik.server_id)
        .await
        .map_err(|e| e.to_string())?
        .ok_or("server not found")?;

    let inbounds = servers_db::find_inbounds(&state.pool, server.id)
        .await
        .map_err(|e| e.to_string())?;

    let inbound = inbounds
        .iter()
        .find(|i| i.id == mikrotik.inbound_id)
        .ok_or("inbound not found")?;

    let xui_inbound_id = inbound.inbound_id;

    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(10))
        .build()
        .unwrap();

    let url = format!(
        "{}/panel/api/inbounds/get/{}",
        server.address.trim_end_matches('/'),
        xui_inbound_id
    );

    let resp: Value = client
        .get(&url)
        .header("Authorization", format!("Bearer {}", server.xui_api_key))
        .send()
        .await
        .map_err(|e| e.to_string())?
        .json()
        .await
        .map_err(|e| e.to_string())?;

    let obj = resp.get("obj").ok_or("no obj")?;

    let peers = obj["settings"]["peers"]
        .as_array()
        .cloned()
        .unwrap_or_default();

    let existing_ips: Vec<String> = peers
        .iter()
        .flat_map(|p| {
            p["allowedIPs"]
                .as_array()
                .cloned()
                .unwrap_or_default()
                .into_iter()
                .filter_map(|ip| ip.as_str().map(|s| s.to_string()))
        })
        .collect();

    let assigned_ip = next_peer_ip(&existing_ips).ok_or("IP pool exhausted")?;

    let public_key = mikrotik.public_key.ok_or("public key missing")?;

    let mut new_obj = obj.clone();
    let new_peer = serde_json::json!({
        "privateKey": "",
        "publicKey": public_key,
        "allowedIPs": [format!("{}/32", assigned_ip)],
        "keepAlive": 25
    });

    if let Some(peers_arr) = new_obj["settings"]["peers"].as_array_mut() {
        peers_arr.push(new_peer);
    }

    let update_url = format!(
        "{}/panel/api/inbounds/update/{}",
        server.address.trim_end_matches('/'),
        xui_inbound_id
    );

    let update_resp = client
        .post(&update_url)
        .header("Authorization", format!("Bearer {}", server.xui_api_key))
        .json(&new_obj)
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if !update_resp.status().is_success() {
        return Err("3x-ui update failed".to_string());
    }

    db::set_key(&state.pool, id, &public_key, &assigned_ip)
        .await
        .map_err(|e| e.to_string())?;

    Ok(assigned_ip)
}

// src/api/infrastructure/mikrotiks.rs — добавить хендлер script

async fn get_script(State(state): State<AppState>, Path(id): Path<i64>) -> impl IntoResponse {
    let mikrotik = match db::find_by_id(&state.pool, id).await {
        Ok(Some(m)) => m,
        Ok(None) => return StatusCode::NOT_FOUND.into_response(),
        Err(e) => {
            eprintln!("DB error: {e}");
            return StatusCode::INTERNAL_SERVER_ERROR.into_response();
        }
    };

    if mikrotik.status != "pending_key" && mikrotik.status != "active" {
        return (StatusCode::BAD_REQUEST, "mikrotik not ready").into_response();
    }

    let server = match servers_db::find_by_id(&state.pool, mikrotik.server_id).await {
        Ok(Some(s)) => s,
        Ok(None) => return StatusCode::NOT_FOUND.into_response(),
        Err(e) => {
            eprintln!("DB error: {e}");
            return StatusCode::INTERNAL_SERVER_ERROR.into_response();
        }
    };

    let inbounds = match servers_db::find_inbounds(&state.pool, server.id).await {
        Ok(i) => i,
        Err(e) => {
            eprintln!("DB error: {e}");
            return StatusCode::INTERNAL_SERVER_ERROR.into_response();
        }
    };

    let inbound = match inbounds.iter().find(|i| i.id == mikrotik.inbound_id) {
        Some(i) => i,
        None => return (StatusCode::BAD_REQUEST, "inbound not found").into_response(),
    };

    let xui_inbound_id = inbound.inbound_id;

    // получаем конфиг инбаунда
    let url = format!(
        "{}/panel/api/inbounds/get/{}",
        server.address.trim_end_matches('/'),
        xui_inbound_id
    );

    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(10))
        .build()
        .unwrap();

    let resp: serde_json::Value = match client
        .get(&url)
        .header("Authorization", format!("Bearer {}", server.xui_api_key))
        .send()
        .await
    {
        Ok(r) => match r.json().await {
            Ok(v) => v,
            Err(e) => {
                eprintln!("3x-ui parse error: {e}");
                return StatusCode::BAD_GATEWAY.into_response();
            }
        },
        Err(e) => {
            eprintln!("3x-ui request error: {e}");
            return StatusCode::BAD_GATEWAY.into_response();
        }
    };

    let secret_key = match resp["obj"]["settings"]["secretKey"].as_str() {
        Some(k) => k.to_string(),
        None => return (StatusCode::BAD_GATEWAY, "secretKey not found").into_response(),
    };

    let server_public_key = match crate::utils::wireguard::public_key_from_secret(&secret_key) {
        Ok(k) => k,
        Err(e) => {
            eprintln!("WG key error: {e}");
            return StatusCode::INTERNAL_SERVER_ERROR.into_response();
        }
    };

    let port = resp["obj"]["port"].as_u64().unwrap_or(51820) as u16;

    // адрес сервера — только хост
    let endpoint_address = server
        .address
        .trim_end_matches('/')
        .trim_start_matches("https://")
        .trim_start_matches("http://")
        .split('/')
        .next()
        .unwrap_or("")
        .to_string();

    let lists_base_url =
        std::env::var("BASE_URL").unwrap_or_else(|_| "http://127.0.0.1:8080".to_string());

    // assigned_ip — если active используем сохранённый, иначе вычисляем
    let assigned_ip = if let Some(ip) = &mikrotik.assigned_ip {
        ip.clone()
    } else {
        let peers = resp["obj"]["settings"]["peers"]
            .as_array()
            .cloned()
            .unwrap_or_default();

        let existing_ips: Vec<String> = peers
            .iter()
            .flat_map(|p| {
                p["allowedIPs"]
                    .as_array()
                    .cloned()
                    .unwrap_or_default()
                    .into_iter()
                    .filter_map(|ip| ip.as_str().map(|s| s.to_string()))
            })
            .collect();

        match crate::utils::wireguard::next_peer_ip(&existing_ips) {
            Some(ip) => ip,
            None => {
                return (StatusCode::INTERNAL_SERVER_ERROR, "IP pool exhausted").into_response();
            }
        }
    };

    let params = crate::utils::script::ScriptParams {
        endpoint_address,
        endpoint_port: port,
        server_public_key,
        assigned_ip,
        mikrotik_uuid: mikrotik.uuid.clone(),
        lists_base_url,
    };

    let script = match crate::utils::script::generate_init_script(&params) {
        Ok(script) => script,
        Err(_) => {
            return StatusCode::INTERNAL_SERVER_ERROR.into_response();
        }
    };

    let mut headers = axum::http::HeaderMap::new();
    headers.insert(
        axum::http::header::CONTENT_TYPE,
        axum::http::HeaderValue::from_static("text/plain; charset=utf-8"),
    );
    headers.insert(
        axum::http::header::CONTENT_DISPOSITION,
        axum::http::HeaderValue::from_static("attachment; filename=\"bypass-hub-init.rsc\""),
    );

    (headers, script).into_response()
}

async fn get_agent(State(state): State<AppState>, Path(id): Path<i64>) -> impl IntoResponse {
    let mikrotik = match db::find_by_id(&state.pool, id).await {
        Ok(Some(m)) => m,
        Ok(None) => return StatusCode::NOT_FOUND.into_response(),
        Err(e) => {
            eprintln!("DB error: {e}");
            return StatusCode::INTERNAL_SERVER_ERROR.into_response();
        }
    };

    if mikrotik.status != "active" {
        return (StatusCode::BAD_REQUEST, "mikrotik not active").into_response();
    }

    let lists_base_url =
        std::env::var("BASE_URL").unwrap_or_else(|_| "http://127.0.0.1:8080".to_string());

    let params = crate::utils::script::AgentParams {
        mikrotik_uuid: mikrotik.uuid.clone(),
        lists_base_url,
    };

    let script = match crate::utils::script::generate_agent_script(&params) {
        Ok(s) => s,
        Err(_) => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    };

    let mut headers = axum::http::HeaderMap::new();
    headers.insert(
        axum::http::header::CONTENT_TYPE,
        axum::http::HeaderValue::from_static("text/plain; charset=utf-8"),
    );
    headers.insert(
        axum::http::header::CONTENT_DISPOSITION,
        axum::http::HeaderValue::from_static("attachment; filename=\"bypass-hub-agent.rsc\""),
    );

    (headers, script).into_response()
}

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", get(list).post(create))
        .route("/{id}", delete(remove))
        .route("/{id}/key", patch(set_key))
        .route("/{id}/retry", post(retry))
        .route("/{id}/script", get(get_script))
        .route("/{id}/agent", get(get_agent))
}
