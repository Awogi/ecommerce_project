use actix_web::http::StatusCode;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("unauthorized")]
    Unauthorized,
    #[error("not found")]
    NotFound,
    #[error("invalid credentials")]
    InvalidCredentials,
    #[error("internal error")]
    InternalError,
}

impl actix_web::ResponseError for AppError {
    fn status_code(&self) -> StatusCode {
        match self {
            AppError::Unauthorized => StatusCode::UNAUTHORIZED,
            AppError::NotFound => StatusCode::NOT_FOUND,
            AppError::InvalidCredentials => StatusCode::UNAUTHORIZED,
            AppError::InternalError => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}
