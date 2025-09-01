use serde::Serialize;
use sqlx::FromRow;

#[derive(Debug, FromRow, Serialize)]
pub struct WebResponse<T> {
    pub data: T,
    pub message: String,
}
