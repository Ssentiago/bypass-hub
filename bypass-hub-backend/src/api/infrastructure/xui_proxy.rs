// src/api/infrastructure/xui_proxy.rs
use crate::db::infrastructure::servers as db;
use crate::state::AppState;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::post,
    Json, Router,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Deserialize)]
pub struct ProxyRequest {
    pub path: String,
    pub method: Option<String>,
    pub body: Option<Value>,
}

#[derive(Serialize)]
pub struct ProxyResponse {
    pub status: u16,
    pub body: Value,
}

pub async fn proxy(
    State(state): State<AppState>,
    Path(id): Path<i64>,
    Json(req): Json<ProxyRequest>,
) -> impl IntoResponse {
    let server = match db::find_by_id(&state.pool, id).await {
        Ok(Some(s)) => s,
        Ok(None) => return StatusCode::NOT_FOUND.into_response(),
        Err(e) => {
            eprintln!("DB error: {e}");
            return StatusCode::INTERNAL_SERVER_ERROR.into_response();
        }
    };

    let url = format!(
        "{}/{}",
        server.address.trim_end_matches('/'),
        req.path.trim_start_matches('/')
    );

    let method = req
        .method
        .unwrap_or_else(|| "GET".to_string())
        .to_uppercase();

    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(10))
        .build()
        .unwrap();

    let mut builder = match method.as_str() {
        "GET" => client.get(&url),
        "POST" => client.post(&url),
        "PUT" => client.put(&url),
        "DELETE" => client.delete(&url),
        "PATCH" => client.patch(&url),
        _ => return (StatusCode::BAD_REQUEST, "unsupported method").into_response(),
    };

    builder = builder.header("Authorization", format!("Bearer {}", server.xui_api_key));

    if let Some(body) = req.body {
        builder = builder.json(&body);
    }

    let resp = match builder.send().await {
        Ok(r) => r,
        Err(e) => {
            eprintln!("3x-ui request error: {e}");
            return (
                StatusCode::BAD_GATEWAY,
                Json(serde_json::json!({"online": false, "error": e.to_string()})),
            )
                .into_response();
        }
    };

    let status = resp.status().as_u16();
    let body: Value = resp.json().await.unwrap_or(Value::Null);

    Json(ProxyResponse { status, body }).into_response()
}

pub fn router() -> Router<AppState> {
    Router::new().route("/{id}/proxy", post(proxy))
}
