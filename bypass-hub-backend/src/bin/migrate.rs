use sqlx::sqlite::SqliteConnectOptions;
use sqlx::sqlite::SqlitePoolOptions;
use std::env;
use std::str::FromStr;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();

    let db_url = env::var("DATABASE_URL").unwrap_or_else(|_| "sqlite://bypass-hub.db".to_string());

    let opts = SqliteConnectOptions::from_str(&db_url)?.create_if_missing(true);
    let pool = SqlitePoolOptions::new().connect_with(opts).await?;

    println!("Running database migrations...");
    sqlx::migrate!("./migrations").run(&pool).await?;
    println!("Migrations completed successfully!");

    Ok(())
}
