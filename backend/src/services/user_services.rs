use actix_web::web;
use diesel::prelude::*;
use bcrypt::{hash, verify, DEFAULT_COST};
use chrono::Utc;

use crate::config::database::DbPool;
use crate::model::user::{User, NewUser, UpdateUser};
use crate::schema::users;
use crate::dto::user as user_dto;
use crate::dto::common::ErrorResponse;
use crate::error::AppError;

pub struct UserService;

impl UserService {
    pub async fn register_user_json(
        pool: web::Data<DbPool>,
        request: user_dto::RegisterUserRequest,
    ) -> Result<user_dto::RegisterUserResponse, AppError> {
        // Basic validation — assume validation helpers exist in crate::utils::validation
    if request.full_name.len() < 2 || request.full_name.len() > 120 {
            return Ok(user_dto::RegisterUserResponse::Error(ErrorResponse {
                code: "VALIDATION_ERROR".to_string(),
                message: "Name must be 2-120 characters".to_string(),
                details: None,
                field: Some("name".to_string()),
            }));
        }

    if request.password.len() < 8 {
            return Ok(user_dto::RegisterUserResponse::Error(ErrorResponse {
                code: "VALIDATION_ERROR".to_string(),
                message: "Password must be at least 8 characters".to_string(),
                details: None,
                field: Some("password".to_string()),
            }));
        }

    let mut conn = pool.get().map_err(|_| AppError::InternalError)?;

        // duplicate email check
        let existing = users::table
            .filter(users::email.eq(&request.email))
            .first::<User>(&mut conn)
            .optional()
            .map_err(|_| AppError::InternalError)?;

        if existing.is_some() {
            return Ok(user_dto::RegisterUserResponse::Error(ErrorResponse {
                code: "DUPLICATE_EMAIL".to_string(),
                message: "Email already exists".to_string(),
                details: None,
                field: Some("email".to_string()),
            }));
        }

        let hashed = hash(&request.password, DEFAULT_COST).map_err(|_| AppError::InternalError)?;

        let new_user = NewUser {
            full_name: request.full_name,
            email: request.email,
            password_hash: hashed,
            role: request.role,
        };

        let inserted = diesel::insert_into(users::table)
            .values(&new_user)
            .get_result::<User>(&mut conn)
            .map_err(|_| AppError::InternalError)?;

        let profile = user_dto::UserProfile {
            id: inserted.id,
            full_name: inserted.full_name,
            email: inserted.email,
            role: inserted.role,
            created_at: inserted.created_at.to_string(),
        };

        Ok(user_dto::RegisterUserResponse::Success(profile))
    }

    pub async fn login_user_json(
        pool: web::Data<DbPool>,
        request: user_dto::LoginRequest,
    ) -> Result<user_dto::LoginResponse, AppError> {
    if request.identifier.is_empty() || request.password.is_empty() {
            return Ok(user_dto::LoginResponse::Error(ErrorResponse {
                code: "VALIDATION_ERROR".to_string(),
                message: "Identifier and password are required".to_string(),
                details: None,
                field: None,
            }));
        }

        let mut conn = pool.get().map_err(|_| AppError::InternalError)?;

        let user = users::table
            .filter(users::email.eq(&request.identifier))
            .first::<User>(&mut conn)
            .map_err(|_| AppError::InternalError)?;

        if !verify(&request.password, &user.password_hash).map_err(|_| AppError::InternalError)? {
            return Ok(user_dto::LoginResponse::Error(ErrorResponse {
                code: "INVALID_CREDENTIALS".to_string(),
                message: "Invalid email or password".to_string(),
                details: None,
                field: None,
            }));
        }

        // For simplicity don't generate tokens here — return success with profile only
        let profile = user_dto::UserProfile {
            id: user.id,
            full_name: user.full_name,
            email: user.email,
            role: user.role,
            created_at: user.created_at.to_string(),
        };

    Ok(user_dto::LoginResponse::Success(user_dto::LoginSuccess { user: profile, access_token: "".to_string(), refresh_token: "".to_string(), expires_in: 0 }))
    }

    pub async fn get_user_profile_json(
        pool: web::Data<DbPool>,
        user_id: i32,
    ) -> Result<user_dto::GetMeResponse, AppError> {
        let mut conn = pool.get().map_err(|_| AppError::InternalError)?;

        let user = users::table.find(user_id).first::<User>(&mut conn).map_err(|_| AppError::InternalError)?;

        let profile = user_dto::UserProfile {
            id: user.id,
            full_name: user.full_name,
            email: user.email,
            role: user.role,
            created_at: user.created_at.to_string(),
        };

        Ok(user_dto::GetMeResponse::Success { user: profile })
    }

    pub async fn update_user_profile_json(
        pool: web::Data<DbPool>,
        user_id: i32,
        request: user_dto::UpdateUserRequest,
    ) -> Result<user_dto::UpdateUserResponse, AppError> {
        let mut conn = pool.get().map_err(|_| AppError::InternalError)?;

        let mut update = UpdateUser {
            full_name: request.full_name,
            email: request.email,
            password_hash: None,
            role: request.role,
        };

        if let Some(pwd) = request.password {
                // include school_id in profile if needed (may be shown in frontend)
            update.password_hash = Some(hash(&pwd, DEFAULT_COST).map_err(|_| AppError::InternalError)?);
        }


        let updated = diesel::update(users::table.find(user_id)).set(&update).get_result::<User>(&mut conn).map_err(|_| AppError::InternalError)?;

        let profile = user_dto::UserProfile {
            id: updated.id,
            full_name: updated.full_name,
            email: updated.email,
            role: updated.role,
            created_at: updated.created_at.to_string(),
        };

        Ok(user_dto::UpdateUserResponse::Success { user: profile })
    }

    pub async fn refresh_token_json(
        _pool: web::Data<DbPool>,
        _request: user_dto::RefreshTokenRequest,
    ) -> Result<user_dto::RefreshTokenResponse, AppError> {
        Ok(user_dto::RefreshTokenResponse::Error(ErrorResponse { code: "NOT_IMPLEMENTED".to_string(), message: "Not implemented".to_string(), details: None, field: None }))
    }

    // email verification stub
    pub async fn verify_email_json(
        _pool: web::Data<DbPool>,
        _request: user_dto::VerifyEmailRequest,
    ) -> Result<user_dto::VerifyEmailResponse, AppError> {
        Ok(user_dto::VerifyEmailResponse::Error(crate::dto::common::ErrorResponse { code: "NOT_IMPLEMENTED".to_string(), message: "Not implemented".to_string(), details: None, field: None }))
    }

    pub async fn forgot_password(
        _pool: web::Data<DbPool>,
        _request: user_dto::ForgotPasswordRequest,
    ) -> Result<(), AppError> {
        // For now, just pretend we sent an email
        Ok(())
    }

    pub async fn reset_password(
        _pool: web::Data<DbPool>,
        _request: user_dto::ResetPasswordRequest,
    ) -> Result<(), AppError> {
        // Not implemented — would verify otp and update password
        Ok(())
    }

    pub async fn change_password(
        pool: web::Data<DbPool>,
        user_id: i32,
        request: user_dto::ChangePasswordRequest,
    ) -> Result<(), AppError> {
        let mut conn = pool.get().map_err(|_| AppError::InternalError)?;

        let user = users::table.find(user_id).first::<User>(&mut conn).map_err(|_| AppError::InternalError)?;

        if !verify(&request.old_password, &user.password_hash).map_err(|_| AppError::InternalError)? {
            return Err(AppError::InvalidCredentials);
        }

        let hashed = hash(&request.new_password, DEFAULT_COST).map_err(|_| AppError::InternalError)?;

        let mut update = UpdateUser {
            full_name: None,
            email: None,
            password_hash: Some(hashed),
            role: None,
        };

        diesel::update(users::table.find(user_id)).set(&update).execute(&mut conn).map_err(|_| AppError::InternalError)?;

        Ok(())
    }
}
