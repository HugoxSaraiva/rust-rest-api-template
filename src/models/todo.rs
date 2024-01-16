use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Serialize, Deserialize, ToSchema)]
pub enum TodoError {
    /// Todo already exists conflict.
    #[schema(example = "Todo already exists")]
    Conflict(String),
    /// Todo not found by id.
    #[schema(example = "id = 1")]
    NotFound(String),
    /// Todo operation unauthorized
    #[schema(example = "missing api key")]
    Unauthorized(String),
}

/// Item to do.
#[derive(Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct Todo {
    pub id: Uuid,
    pub value: String,
    pub is_done: bool,
}
