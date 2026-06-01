// src/db/infrastructure/mikrotiks.rs
use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Mikrotik {
    pub id: i64,
    pub name: String,
    pub server_id: i64,
    pub inbound_id: i64,
    pub public_key: Option<String>,
    pub assigned_ip: Option<String>,
    pub uuid: String,
    pub status: String,
    pub created_at: i64,
}

pub async fn find_all(pool: &SqlitePool) -> sqlx::Result<Vec<Mikrotik>> {
    sqlx::query_as!(
        Mikrotik,
        r#"SELECT id as "id!: i64", name, server_id as "server_id!: i64",
           inbound_id as "inbound_id!: i64", public_key, assigned_ip,
           uuid, status, created_at as "created_at!: i64"
           FROM mikrotiks ORDER BY created_at DESC"#
    )
    .fetch_all(pool)
    .await
}

pub async fn create(
    pool: &SqlitePool,
    name: &str,
    server_id: i64,
    inbound_id: i64,
    uuid: &str,
) -> sqlx::Result<i64> {
    let id = sqlx::query!(
        "INSERT INTO mikrotiks (name, server_id, inbound_id, uuid) VALUES (?, ?, ?, ?)",
        name,
        server_id,
        inbound_id,
        uuid
    )
    .execute(pool)
    .await?
    .last_insert_rowid();

    Ok(id)
}

pub async fn find_by_id(pool: &SqlitePool, id: i64) -> sqlx::Result<Option<Mikrotik>> {
    sqlx::query_as!(
        Mikrotik,
        r#"SELECT id as "id!: i64", name, server_id as "server_id!: i64",
           inbound_id as "inbound_id!: i64", public_key, assigned_ip,
           uuid, status, created_at as "created_at!: i64"
           FROM mikrotiks WHERE id = ?"#,
        id
    )
    .fetch_optional(pool)
    .await
}

pub async fn find_by_uuid(pool: &SqlitePool, uuid: &str) -> sqlx::Result<Option<Mikrotik>> {
    sqlx::query_as!(
        Mikrotik,
        r#"SELECT id as "id!: i64", name, server_id as "server_id!: i64",
           inbound_id as "inbound_id!: i64", public_key, assigned_ip,
           uuid, status, created_at as "created_at!: i64"
           FROM mikrotiks WHERE uuid = ?"#,
        uuid
    )
    .fetch_optional(pool)
    .await
}

pub async fn set_key(
    pool: &SqlitePool,
    id: i64,
    public_key: &str,
    assigned_ip: &str,
) -> sqlx::Result<bool> {
    let affected = sqlx::query!(
        "UPDATE mikrotiks SET public_key = ?, assigned_ip = ?, status = 'active' WHERE id = ?",
        public_key,
        assigned_ip,
        id
    )
    .execute(pool)
    .await?
    .rows_affected();

    Ok(affected > 0)
}

pub async fn delete(pool: &SqlitePool, id: i64) -> sqlx::Result<bool> {
    let affected = sqlx::query!("DELETE FROM mikrotiks WHERE id = ?", id)
        .execute(pool)
        .await?
        .rows_affected();

    Ok(affected > 0)
}

pub async fn save_public_key(pool: &SqlitePool, id: i64, public_key: &str) -> sqlx::Result<bool> {
    let affected = sqlx::query!(
        "UPDATE mikrotiks SET public_key = ? WHERE id = ? AND status = 'pending_key'",
        public_key,
        id
    )
    .execute(pool)
    .await?
    .rows_affected();

    Ok(affected > 0)
}
