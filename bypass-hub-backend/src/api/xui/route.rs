// src/routes/xui/routes.rs
use crate::db;
use crate::state::AppState;
use axum::extract::Query;
use axum::{
    Json, Router,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{delete, get, post},
};
use serde::{Deserialize, Serialize};

async fn list(State(state): State<AppState>) -> impl IntoResponse {
    match db::xui::route::find_all(&state.pool).await {
        Ok(routes) => Json(routes).into_response(),
        Err(e) => {
            eprintln!("DB error: {e}");
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}

#[derive(Deserialize)]
struct CreateRouteRequest {
    value: String,
    r#type: String,
    group_ids: Option<Vec<i64>>,
}

async fn create(
    State(state): State<AppState>,
    Json(body): Json<CreateRouteRequest>,
) -> impl IntoResponse {
    if body.value.is_empty() {
        return (StatusCode::BAD_REQUEST, "value is required").into_response();
    }
    if !matches!(body.r#type.as_str(), "domain" | "ip") {
        return (StatusCode::BAD_REQUEST, "type must be 'domain' or 'ip'").into_response();
    }

    let id = match db::xui::route::create(&state.pool, &body.value, &body.r#type).await {
        Ok(id) => id,
        Err(e) if e.to_string().contains("UNIQUE") => {
            return (StatusCode::CONFLICT, "route already exists").into_response();
        }
        Err(e) => {
            eprintln!("DB error: {e}");
            return StatusCode::INTERNAL_SERVER_ERROR.into_response();
        }
    };

    if let Some(group_ids) = body.group_ids {
        for group_id in group_ids {
            let _ = db::xui::group::add_route(&state.pool, group_id, id).await;
        }
    }

    (StatusCode::CREATED, Json(serde_json::json!({"id": id}))).into_response()
}

async fn remove(State(state): State<AppState>, Path(id): Path<i64>) -> impl IntoResponse {
    match db::xui::route::delete(&state.pool, id).await {
        Ok(true) => StatusCode::NO_CONTENT.into_response(),
        Ok(false) => StatusCode::NOT_FOUND.into_response(),
        Err(e) => {
            eprintln!("DB error: {e}");
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}

#[derive(Deserialize)]
struct BulkRouteItem {
    value: String,
    r#type: String,
}

#[derive(Deserialize)]
struct BulkCreateRequest {
    routes: Vec<BulkRouteItem>,
    group_ids: Option<Vec<i64>>,
}

async fn create_bulk(
    State(state): State<AppState>,
    Json(body): Json<BulkCreateRequest>,
) -> impl IntoResponse {
    if body.routes.is_empty() {
        return (StatusCode::BAD_REQUEST, "empty list").into_response();
    }

    for item in &body.routes {
        if !matches!(item.r#type.as_str(), "domain" | "ip") {
            return (
                StatusCode::BAD_REQUEST,
                format!("invalid type: {}", item.r#type),
            )
                .into_response();
        }
    }

    let routes: Vec<(&str, &str)> = body
        .routes
        .iter()
        .map(|r| (r.value.as_str(), r.r#type.as_str()))
        .collect();

    let inserted = match db::xui::route::create_bulk(&state.pool, &routes).await {
        Ok(n) => n,
        Err(e) => {
            eprintln!("DB error: {e}");
            return StatusCode::INTERNAL_SERVER_ERROR.into_response();
        }
    };

    if let Some(group_ids) = body.group_ids {
        let all_routes = db::xui::route::find_all(&state.pool)
            .await
            .unwrap_or_default();
        let values: std::collections::HashSet<&str> =
            body.routes.iter().map(|r| r.value.as_str()).collect();
        for route in all_routes
            .iter()
            .filter(|r| values.contains(r.value.as_str()))
        {
            for &group_id in &group_ids {
                let _ = db::xui::group::add_route(&state.pool, group_id, route.id).await;
            }
        }
    }

    (
        StatusCode::CREATED,
        Json(serde_json::json!({"inserted": inserted})),
    )
        .into_response()
}

#[derive(Serialize)]
struct GroupInfo {
    id: i64,
    name: String,
}

#[derive(Serialize)]
struct GroupedRoutesResponse {
    group: Option<GroupInfo>,
    routes: Vec<crate::models::route::Route>,
}

async fn list_grouped(State(state): State<AppState>) -> impl IntoResponse {
    match db::xui::route::find_grouped(&state.pool).await {
        Ok(grouped) => {
            let response: Vec<GroupedRoutesResponse> = grouped
                .into_iter()
                .map(|g| GroupedRoutesResponse {
                    group: g.group_id.map(|id| GroupInfo {
                        id,
                        name: g.group_name.unwrap_or_default(),
                    }),
                    routes: g.routes,
                })
                .collect();
            Json(response).into_response()
        }
        Err(e) => {
            eprintln!("DB error: {e}");
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", get(list).post(create))
        .route("/grouped", get(list_grouped))
        .route("/bulk", post(create_bulk))
        .route("/{id}", delete(remove))
}
