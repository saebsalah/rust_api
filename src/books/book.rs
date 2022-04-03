use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Serialize, Deserialize, Debug, FromRow, Clone)]
pub struct Book {
    pub id: Option<i64>,
    pub title: Option<String>,
    pub author: Option<String>,
}
