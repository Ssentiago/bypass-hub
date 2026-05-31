// src/db/infrastructure/servers.rs
use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Server {
    pub id: i64,
    pub name: String,
    pub address: String,
    pub xui_api_key: String,
    pub uuid: String,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct ServerInbound {
    pub id: i64,
    pub server_id: i64,
    pub inbound_id: i64,
}

pub async fn find_all(pool: &SqlitePool) -> sqlx::Result<Vec<Server>> {
    sqlx::query_as!(
        Server,
        "SELECT id, name, address, xui_api_key, uuid FROM servers"
    )
    .fetch_all(pool)
    .await
}

pub async fn create(
    pool: &SqlitePool,
    name: &str,
    address: &str,
    xui_api_key: &str,
    uuid: &str,
) -> sqlx::Result<i64> {
    let id = sqlx::query!(
        "INSERT INTO servers (name, address, xui_api_key, uuid) VALUES (?, ?, ?, ?)",
        name,
        address,
        xui_api_key,
        uuid
    )
    .execute(pool)
    .await?
    .last_insert_rowid();

    Ok(id)
}

pub async fn delete(pool: &SqlitePool, id: i64) -> sqlx::Result<bool> {
    let affected = sqlx::query!("DELETE FROM servers WHERE id = ?", id)
        .execute(pool)
        .await?
        .rows_affected();

    Ok(affected > 0)
}

pub async fn find_by_uuid(pool: &SqlitePool, uuid: &str) -> sqlx::Result<Option<Server>> {
    sqlx::query_as!(
        Server,
        r#"SELECT id as "id!: i64", name, address, xui_api_key, uuid FROM servers WHERE uuid = ?"#,
        uuid
    )
    .fetch_optional(pool)
    .await
}

pub async fn find_inbounds(pool: &SqlitePool, server_id: i64) -> sqlx::Result<Vec<ServerInbound>> {
    sqlx::query_as!(
        ServerInbound,
        r#"SELECT id as "id!: i64", server_id as "server_id!: i64", inbound_id as "inbound_id!: i64"
           FROM server_inbounds WHERE server_id = ?"#,
        server_id
    )
    .fetch_all(pool)
    .await
}

pub async fn add_inbound(pool: &SqlitePool, server_id: i64, inbound_id: i64) -> sqlx::Result<i64> {
    let id = sqlx::query!(
        "INSERT INTO server_inbounds (server_id, inbound_id) VALUES (?, ?)",
        server_id,
        inbound_id
    )
    .execute(pool)
    .await?
    .last_insert_rowid();

    Ok(id)
}

pub async fn remove_inbound(
    pool: &SqlitePool,
    server_id: i64,
    inbound_id: i64,
) -> sqlx::Result<bool> {
    let affected = sqlx::query!(
        "DELETE FROM server_inbounds WHERE server_id = ? AND inbound_id = ?",
        server_id,
        inbound_id
    )
    .execute(pool)
    .await?
    .rows_affected();

    Ok(affected > 0)
}

pub async fn find_by_id(pool: &SqlitePool, id: i64) -> sqlx::Result<Option<Server>> {
    sqlx::query_as!(
        Server,
        "SELECT id, name, address, xui_api_key, uuid FROM servers WHERE id = ?",
        id
    )
    .fetch_optional(pool)
    .await
}
