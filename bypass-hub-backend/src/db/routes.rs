use crate::models::route::Route;
use sqlx::SqlitePool;

pub async fn find_all(pool: &SqlitePool) -> sqlx::Result<Vec<Route>> {
    sqlx::query_as!(Route, "SELECT id, value, type as \"type\" FROM routes")
        .fetch_all(pool)
        .await
}

pub async fn create(pool: &SqlitePool, value: &str, route_type: &str) -> sqlx::Result<i64> {
    let id = sqlx::query!(
        "INSERT INTO routes (value, type) VALUES (?, ?)",
        value,
        route_type
    )
    .execute(pool)
    .await?
    .last_insert_rowid();

    Ok(id)
}

pub async fn create_bulk(pool: &SqlitePool, routes: &[(&str, &str)]) -> sqlx::Result<u64> {
    let mut inserted = 0u64;
    for (value, route_type) in routes {
        let result = sqlx::query!(
            "INSERT OR IGNORE INTO routes (value, type) VALUES (?, ?)",
            value,
            route_type
        )
        .execute(pool)
        .await?;
        inserted += result.rows_affected();
    }
    Ok(inserted)
}

pub struct GroupedRoutes {
    pub group_id: Option<i64>,
    pub group_name: Option<String>,
    pub routes: Vec<Route>,
}

pub async fn find_grouped(pool: &SqlitePool) -> sqlx::Result<Vec<GroupedRoutes>> {
    let rows = sqlx::query!(
        r#"
        SELECT
            r.id, r.value, r.type as "type",
            g.id as "group_id?", g.name as "group_name?"
        FROM routes r
        LEFT JOIN routes_groups rg ON rg.route_id = r.id
        LEFT JOIN "group" g ON g.id = rg.group_id
        ORDER BY g.name NULLS LAST, r.value
        "#
    )
    .fetch_all(pool)
    .await?;

    let mut map: indexmap::IndexMap<Option<i64>, GroupedRoutes> = indexmap::IndexMap::new();

    for row in rows {
        let entry = map.entry(row.group_id).or_insert_with(|| GroupedRoutes {
            group_id: row.group_id,
            group_name: row.group_name.clone(),
            routes: vec![],
        });
        entry.routes.push(Route {
            id: row.id.unwrap_or_default(),
            value: row.value,
            r#type: row.r#type,
        });
    }

    Ok(map.into_values().collect())
}

pub async fn delete(pool: &SqlitePool, id: i64) -> sqlx::Result<bool> {
    let affected = sqlx::query!("DELETE FROM routes WHERE id = ?", id)
        .execute(pool)
        .await?
        .rows_affected();

    Ok(affected > 0)
}
