// src/routes/xui/groups.rs
use crate::db;
use crate::models::group::CreateGroupRequest;
use crate::state::AppState;
use axum::{
    Json, Router,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{delete, get, post},
};
use serde::Deserialize;

async fn list(State(state): State<AppState>) -> impl IntoResponse {
    match db::xui::group::find_all(&state.pool).await {
        Ok(groups) => Json(groups).into_response(),
        Err(e) => {
            eprintln!("DB error: {e}");
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}

async fn create(
    State(state): State<AppState>,
    Json(body): Json<CreateGroupRequest>,
) -> impl IntoResponse {
    if body.name.is_empty() {
        return (StatusCode::BAD_REQUEST, "name is required").into_response();
    }

    match db::xui::group::create(&state.pool, &body.name, body.description.as_deref()).await {
        Ok(id) => (StatusCode::CREATED, Json(serde_json::json!({ "id": id }))).into_response(),
        Err(e) if e.to_string().contains("UNIQUE") => {
            (StatusCode::CONFLICT, "group already exists").into_response()
        }
        Err(e) => {
            eprintln!("DB error: {e}");
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}

async fn remove(State(state): State<AppState>, Path(id): Path<i64>) -> impl IntoResponse {
    match db::xui::group::delete(&state.pool, id).await {
        Ok(true) => StatusCode::NO_CONTENT.into_response(),
        Ok(false) => StatusCode::NOT_FOUND.into_response(),
        Err(e) => {
            eprintln!("DB error: {e}");
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}

async fn get_routes(State(state): State<AppState>, Path(id): Path<i64>) -> impl IntoResponse {
    match db::xui::group::find_routes(&state.pool, id).await {
        Ok(routes) => Json(routes).into_response(),
        Err(e) => {
            eprintln!("DB error: {e}");
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}

#[derive(Deserialize)]
struct RouteIdBody {
    route_id: i64,
}

async fn add_route(
    State(state): State<AppState>,
    Path(id): Path<i64>,
    Json(body): Json<RouteIdBody>,
) -> impl IntoResponse {
    match db::xui::group::add_route(&state.pool, id, body.route_id).await {
        Ok(_) => StatusCode::NO_CONTENT.into_response(),
        Err(e) => {
            eprintln!("DB error: {e}");
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}

async fn remove_route(
    State(state): State<AppState>,
    Path((id, route_id)): Path<(i64, i64)>,
) -> impl IntoResponse {
    match db::xui::group::remove_route(&state.pool, id, route_id).await {
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
        .route("/{id}/routes", get(get_routes).post(add_route))
        .route("/{id}/routes/{route_id}", delete(remove_route))
}
