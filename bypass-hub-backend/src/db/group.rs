use crate::models::group::Group;
use sqlx::SqlitePool;

pub async fn find_all(pool: &SqlitePool) -> sqlx::Result<Vec<Group>> {
    sqlx::query_as!(Group, "SELECT id, name, description FROM \"group\"")
        .fetch_all(pool)
        .await
}

pub async fn create(pool: &SqlitePool, name: &str, description: Option<&str>) -> sqlx::Result<i64> {
    let id = sqlx::query!(
        "INSERT INTO \"group\" (name, description) VALUES (?, ?)",
        name,
        description
    )
    .execute(pool)
    .await?
    .last_insert_rowid();

    Ok(id)
}

pub async fn delete(pool: &SqlitePool, id: i64) -> sqlx::Result<bool> {
    let affected = sqlx::query!("DELETE FROM \"group\" WHERE id = ?", id)
        .execute(pool)
        .await?
        .rows_affected();

    Ok(affected > 0)
}

// Роуты в группе
pub async fn find_routes(
    pool: &SqlitePool,
    group_id: i64,
) -> sqlx::Result<Vec<crate::models::route::Route>> {
    sqlx::query_as!(
        crate::models::route::Route,
        r#"SELECT r.id, r.value, r.type as "type"
           FROM routes r
           JOIN routes_groups rg ON rg.route_id = r.id
           WHERE rg.group_id = ?"#,
        group_id
    )
    .fetch_all(pool)
    .await
}

pub async fn add_route(pool: &SqlitePool, group_id: i64, route_id: i64) -> sqlx::Result<()> {
    sqlx::query!(
        "INSERT OR IGNORE INTO routes_groups (route_id, group_id) VALUES (?, ?)",
        route_id,
        group_id
    )
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn remove_route(pool: &SqlitePool, group_id: i64, route_id: i64) -> sqlx::Result<bool> {
    let affected = sqlx::query!(
        "DELETE FROM routes_groups WHERE group_id = ? AND route_id = ?",
        group_id,
        route_id
    )
    .execute(pool)
    .await?
    .rows_affected();

    Ok(affected > 0)
}
