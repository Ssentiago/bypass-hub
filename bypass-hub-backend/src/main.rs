mod api;
pub mod db;
pub mod middleware;
pub mod models;
pub mod state;
pub mod utils;

use crate::state::AppState;
use axum::Router;
use axum::middleware as axum_middleware;
use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
use std::collections::HashMap;
use std::env;
use std::net::SocketAddr;
use std::str::FromStr;
use std::sync::Arc;
use tokio::sync::RwLock;
use tower_http::cors::CorsLayer;
use tower_http::services::{ServeDir, ServeFile};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();

    tracing_subscriber::fmt::init();

    let is_dev = env::var("DEV").is_ok();
    let dev_frontend_dir = "../bypass-hub-frontend/dist";

    let db_url = env::var("DATABASE_URL").unwrap_or_else(|_| "sqlite://bypass-hub.db".to_string());

    let opts = SqliteConnectOptions::from_str(&db_url)?.create_if_missing(true);
    let pool = SqlitePoolOptions::new().connect_with(opts).await?;

    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Cannot invoke migrations");

    let state = AppState {
        pool,
        sessions: Arc::new(RwLock::new(HashMap::new())),
    };

    let backend_port = env::var("BACKEND_PORT").unwrap_or_else(|_| "3000".to_string());

    let protected = Router::new()
        .nest("/api/xui/routes", api::xui::route::router())
        .nest("/api/xui/groups", api::xui::group::router())
        .nest("/api/mikrotik/routes", api::mikrotik::route::router())
        .nest("/api/mikrotik/groups", api::mikrotik::group::router())
        .nest(
            "/api/infrastructure/servers",
            api::infrastructure::servers::router(),
        )
        .nest(
            "/api/infrastructure/mikrotiks",
            api::infrastructure::mikrotiks::router(),
        )
        .layer(axum_middleware::from_fn_with_state(
            state.clone(),
            middleware::session::auth_session,
        ));

    let public = Router::new()
        .nest("/api/auth", api::auth::router())
        .nest("/lists", api::lists::router());

    let mut app = Router::new()
        .merge(protected)
        .merge(public)
        .with_state(state);

    if is_dev {
        println!("LAUNCHING IN DEVELOPMENT ENVIRONMENT. USING ASSETS FROM: {dev_frontend_dir}");
        app = app
            .fallback_service(
                ServeDir::new(dev_frontend_dir)
                    .fallback(ServeFile::new(format!("{dev_frontend_dir}/index.html"))),
            )
            .layer(CorsLayer::very_permissive());
    } else {
        println!("RUNNING IN PRODUCTION ENVIRONMENT. USE NGINX TO SERVE ASSETS...");
    }

    app = app.layer(tower_http::trace::TraceLayer::new_for_http());

    let listener = tokio::net::TcpListener::bind(format!("127.0.0.1:{backend_port}"))
        .await
        .expect("Cannot launch server");

    println!("Listening on 127.0.0.1:{backend_port} over HTTP...");

    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await?;

    Ok(())
}
