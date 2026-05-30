use crate::state::AppState;
use axum::{
    extract::{Request, State},
    http::StatusCode,
    middleware::Next,
    response::Response,
};
use axum_extra::extract::CookieJar;
use std::time::Instant;

pub async fn auth_session(
    State(state): State<AppState>,
    jar: CookieJar,
    request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    let session_id = jar
        .get("session_id")
        .map(|c| c.value().to_string())
        .ok_or(StatusCode::UNAUTHORIZED)?;

    let sessions = state.sessions.read().await;
    let expires_at = sessions.get(&session_id).ok_or(StatusCode::UNAUTHORIZED)?;

    if Instant::now() > *expires_at {
        return Err(StatusCode::UNAUTHORIZED);
    }

    Ok(next.run(request).await)
}
