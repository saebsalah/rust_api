use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Serialize, Deserialize, Debug, FromRow, Clone)]
pub struct Author {
    pub id: Option<i64>,
    pub name: Option<String>,
}
