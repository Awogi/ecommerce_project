use actix_web::{HttpRequest, Result};
use crate::error::AppError;
use crate::utils::jwt::{verify_access_token, Claims};

pub struct AuthMiddleware;

impl AuthMiddleware {
    // Method expected by the controller
    pub async fn _verify_token(&self, token: &str) -> Result<Claims, AppError> {
        verify_access_token(token)
            .map_err(|_| AppError::InvalidCredentials)
    }

    pub fn extract_user_id(req: &HttpRequest) -> Result<i32, AppError> {
        let auth_header = req
            .headers()
            .get("Authorization")
            .ok_or(AppError::Unauthorized)?
            .to_str()
            .map_err(|_| AppError::Unauthorized)?;

        if !auth_header.starts_with("Bearer ") {
            return Err(AppError::Unauthorized);
        }

        let token = &auth_header[7..];
        let claims = verify_access_token(token)
            .map_err(|_| AppError::InvalidCredentials)?;

        claims.sub.parse::<i32>()
            .map_err(|_| AppError::InvalidCredentials)
    }

    pub fn extract_claims(req: &HttpRequest) -> Result<Claims, AppError> {
        let auth_header = req
            .headers()
            .get("Authorization")
            .ok_or(AppError::Unauthorized)?
            .to_str()
            .map_err(|_| AppError::Unauthorized)?;

        if !auth_header.starts_with("Bearer ") {
            return Err(AppError::Unauthorized);
        }

        let token = &auth_header[7..];
        verify_access_token(token)
            .map_err(|_| AppError::InvalidCredentials)
    }

    // Alias for extract_claims for consistency
    pub fn validate_token(req: &HttpRequest) -> Result<Claims, AppError> {
        Self::extract_claims(req)
    }
}
