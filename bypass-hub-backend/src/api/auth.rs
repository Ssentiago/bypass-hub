use crate::state::AppState;
use axum::routing::get;
use axum::{
    Json, Router,
    extract::State,
    http::{HeaderValue, StatusCode, header},
    response::IntoResponse,
    routing::post,
};
use axum_extra::extract::CookieJar;
use serde::Deserialize;
use std::sync::OnceLock;
use std::time::{Duration, Instant};
use uuid::Uuid;

const SESSION_TTL: Duration = Duration::from_secs(60 * 60 * 24);

static ADMIN_USERNAME: OnceLock<String> = OnceLock::new();
static ADMIN_PASSWORD_HASH: OnceLock<String> = OnceLock::new();

fn admin_username() -> &'static str {
    ADMIN_USERNAME
        .get_or_init(|| std::env::var("ADMIN_USERNAME").unwrap_or_else(|_| "admin".to_string()))
}

fn admin_password_hash() -> &'static str {
    ADMIN_PASSWORD_HASH.get_or_init(|| {
        let password = std::env::var("ADMIN_PASSWORD").expect("ADMIN_PASSWORD must be set");
        bcrypt::hash(&password, bcrypt::DEFAULT_COST).expect("Failed to hash password")
    })
}

#[derive(Deserialize)]
struct LoginRequest {
    username: String,
    password: String,
}

async fn login(State(state): State<AppState>, Json(body): Json<LoginRequest>) -> impl IntoResponse {
    if body.username != admin_username() {
        return (StatusCode::UNAUTHORIZED, "Invalid credentials").into_response();
    }

    match bcrypt::verify(&body.password, admin_password_hash()) {
        Ok(true) => {}
        Ok(false) => return (StatusCode::UNAUTHORIZED, "Invalid credentials").into_response(),
        Err(e) => {
            eprintln!("Bcrypt error: {e}");
            return StatusCode::INTERNAL_SERVER_ERROR.into_response();
        }
    }

    let session_id = Uuid::new_v4().to_string();
    let expires_at = Instant::now() + SESSION_TTL;

    state
        .sessions
        .write()
        .await
        .insert(session_id.clone(), expires_at);

    let is_dev = std::env::var("DEV").is_ok();
    let cookie = if is_dev {
        format!("session_id={}; HttpOnly; Path=/; Max-Age=86400", session_id)
    } else {
        format!(
            "session_id={}; HttpOnly; Path=/; Max-Age=86400; Secure; SameSite=Strict",
            session_id
        )
    };

    (
        StatusCode::OK,
        [(header::SET_COOKIE, HeaderValue::from_str(&cookie).unwrap())],
    )
        .into_response()
}

async fn logout(State(state): State<AppState>, jar: CookieJar) -> impl IntoResponse {
    if let Some(cookie) = jar.get("session_id") {
        state.sessions.write().await.remove(cookie.value());
    }

    let cookie = "session_id=; HttpOnly; Path=/; Max-Age=0; Expires=Thu, 01 Jan 1970 00:00:00 GMT";

    (
        StatusCode::OK,
        [(header::SET_COOKIE, HeaderValue::from_str(cookie).unwrap())],
    )
        .into_response()
}

async fn me(State(state): State<AppState>, jar: CookieJar) -> impl IntoResponse {
    let session_id = match jar.get("session_id") {
        Some(c) => c.value().to_string(),
        None => return StatusCode::UNAUTHORIZED.into_response(),
    };

    let sessions = state.sessions.read().await;
    let expires_at = match sessions.get(&session_id) {
        Some(e) => e,
        None => return StatusCode::UNAUTHORIZED.into_response(),
    };

    if Instant::now() > *expires_at {
        return StatusCode::UNAUTHORIZED.into_response();
    }

    StatusCode::OK.into_response()
}

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/login", post(login))
        .route("/logout", post(logout))
        .route("/me", get(me))
}
