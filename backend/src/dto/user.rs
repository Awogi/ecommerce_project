use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RegisterUserRequest {
    pub full_name: String,
    pub email: String,
    pub password: String,
    pub phone: String,
    pub address: String,
    pub role: Option<String>,
    pub school_id: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserProfile {
    pub id: i32,
    pub full_name: String,
    pub email: String,
    pub role: String,
    pub created_at: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum RegisterUserResponse {
    Success(UserProfile),
    EmailVerificationRequired(EmailVerificationRequiredResponse),
    Error(crate::dto::common::ErrorResponse),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EmailVerificationRequiredResponse {
    pub message: String,
    pub email: String,
    pub expires_in_minutes: i64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct VerifyEmailRequest {
    pub email: String,
    pub otp: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum VerifyEmailResponse {
    Success(UserProfile),
    Error(crate::dto::common::ErrorResponse),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LoginRequest {
    pub identifier: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LoginSuccess {
    pub user: UserProfile,
    pub access_token: String,
    pub refresh_token: String,
    pub expires_in: i64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum LoginResponse {
    Success(LoginSuccess),
    Error(crate::dto::common::ErrorResponse),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum GetMeResponse {
    Success { user: UserProfile },
    Error(crate::dto::common::ErrorResponse),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UpdateUserRequest {
    pub full_name: Option<String>,
    pub email: Option<String>,
    pub password: Option<String>,
    pub role: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum UpdateUserResponse {
    Success { user: UserProfile },
    Error(crate::dto::common::ErrorResponse),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RefreshTokenRequest {
    pub refresh_token: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum RefreshTokenResponse {
    Success { access_token: String, refresh_token: String, expires_in: i64 },
    Error(crate::dto::common::ErrorResponse),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ForgotPasswordRequest {
    pub email: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ResetPasswordRequest {
    pub email: String,
    pub otp: String,
    pub new_password: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ChangePasswordRequest {
    pub old_password: String,
    pub new_password: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum RefreshTokenResponseWrapper {
    Success { access_token: String, refresh_token: String, expires_in: i64 },
    Error(crate::dto::common::ErrorResponse),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GenericResponse {
    pub message: String,
}
