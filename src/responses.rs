use serde::Serialize;
use sqlx::Error;

#[derive(Serialize)]
pub struct CustomError {
    message: String,
}

impl CustomError {
    pub fn new(e: Error) -> Self {
        CustomError {
            message: e.to_string(),
        }
    }
}

#[derive(Serialize)]
pub struct CreateResponse {
    pub id: i64,
}
