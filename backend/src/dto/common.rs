use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ErrorResponse {
    pub code: String,
    pub message: String,
    pub details: Option<String>,
    pub field: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GenericResponse {
    pub message: String,
}
