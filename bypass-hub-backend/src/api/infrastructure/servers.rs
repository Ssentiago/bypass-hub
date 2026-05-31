// src/api/infrastructure/servers.rs
use crate::db::infrastructure::servers as db;
use crate::state::AppState;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{delete, get, post},
    Json, Router,
};
use serde::Deserialize;
use uuid::Uuid;

#[derive(Deserialize)]
struct CreateServerRequest {
    name: String,
    address: String,
    xui_api_key: String,
}

#[derive(Deserialize)]
struct AddInboundRequest {
    inbound_id: i64,
}

async fn list(State(state): State<AppState>) -> impl IntoResponse {
    match db::find_all(&state.pool).await {
        Ok(servers) => Json(servers).into_response(),
        Err(e) => {
            eprintln!("DB error: {e}");
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}

async fn create(
    State(state): State<AppState>,
    Json(body): Json<CreateServerRequest>,
) -> impl IntoResponse {
    if body.name.is_empty() {
        return (StatusCode::BAD_REQUEST, "name is required").into_response();
    }
    if body.address.is_empty() {
        return (StatusCode::BAD_REQUEST, "address is required").into_response();
    }
    if body.xui_api_key.is_empty() {
        return (StatusCode::BAD_REQUEST, "xui_api_key is required").into_response();
    }

    let uuid = Uuid::new_v4().to_string();

    match db::create(
        &state.pool,
        &body.name,
        &body.address,
        &body.xui_api_key,
        &uuid,
    )
    .await
    {
        Ok(id) => (
            StatusCode::CREATED,
            Json(serde_json::json!({"id": id, "uuid": uuid})),
        )
            .into_response(),
        Err(e) if e.to_string().contains("UNIQUE") => {
            (StatusCode::CONFLICT, "server already exists").into_response()
        }
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

async fn list_inbounds(State(state): State<AppState>, Path(id): Path<i64>) -> impl IntoResponse {
    match db::find_inbounds(&state.pool, id).await {
        Ok(inbounds) => Json(inbounds).into_response(),
        Err(e) => {
            eprintln!("DB error: {e}");
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}

async fn add_inbound(
    State(state): State<AppState>,
    Path(id): Path<i64>,
    Json(body): Json<AddInboundRequest>,
) -> impl IntoResponse {
    match db::add_inbound(&state.pool, id, body.inbound_id).await {
        Ok(row_id) => {
            (StatusCode::CREATED, Json(serde_json::json!({"id": row_id}))).into_response()
        }
        Err(e) if e.to_string().contains("UNIQUE") => {
            (StatusCode::CONFLICT, "inbound already registered").into_response()
        }
        Err(e) => {
            eprintln!("DB error: {e}");
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}

async fn remove_inbound(
    State(state): State<AppState>,
    Path((id, inbound_id)): Path<(i64, i64)>,
) -> impl IntoResponse {
    match db::remove_inbound(&state.pool, id, inbound_id).await {
        Ok(true) => StatusCode::NO_CONTENT.into_response(),
        Ok(false) => StatusCode::NOT_FOUND.into_response(),
        Err(e) => {
            eprintln!("DB error: {e}");
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", get(list).post(create))
        .route("/{id}", delete(remove))
        .route("/{id}/inbounds", get(list_inbounds).post(add_inbound))
        .route("/{id}/inbounds/{inbound_id}", delete(remove_inbound))
        .route("/{id}/proxy", post(super::xui_proxy::proxy))
}
