use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Route {
    pub id: i64,
    pub value: String,
    pub r#type: String,
}
