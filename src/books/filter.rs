use serde::Deserialize;

#[derive(Deserialize)]
pub struct Filters {
    pub limit: Option<u32>,
    pub author: Option<String>,
}
