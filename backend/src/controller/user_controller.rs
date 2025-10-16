use actix_web::{web, HttpRequest, HttpResponse, Result, ResponseError};

use crate::config::database::DbPool;
use crate::services::UserService;
use crate::dto::user as user_dto;
use crate::middleware::auth::AuthMiddleware;

pub async fn register_user(
	pool: web::Data<DbPool>,
	body: web::Json<user_dto::RegisterUserRequest>,
) -> Result<HttpResponse> {
	let request = body.into_inner();

	match UserService::register_user_json(pool, request).await {
		Ok(response) => {
			match response {
				user_dto::RegisterUserResponse::Success(_) => Ok(HttpResponse::Created().json(response)),
				user_dto::RegisterUserResponse::EmailVerificationRequired(_) => Ok(HttpResponse::Accepted().json(response)),
				user_dto::RegisterUserResponse::Error(_) => Ok(HttpResponse::BadRequest().json(response)),
			}
		},
		Err(err) => Ok(err.error_response()),
	}
}

pub async fn verify_email(
	pool: web::Data<DbPool>,
	body: web::Json<user_dto::VerifyEmailRequest>,
) -> Result<HttpResponse> {
	let request = body.into_inner();

	match UserService::verify_email_json(pool, request).await {
		Ok(response) => {
			match response {
				user_dto::VerifyEmailResponse::Success(_) => Ok(HttpResponse::Ok().json(response)),
				user_dto::VerifyEmailResponse::Error(_) => Ok(HttpResponse::BadRequest().json(response)),
			}
		},
		Err(err) => Ok(err.error_response()),
	}
}

pub async fn login_user(
	pool: web::Data<DbPool>,
	body: web::Json<user_dto::LoginRequest>,
) -> Result<HttpResponse> {
	let request = body.into_inner();

	match UserService::login_user_json(pool, request).await {
		Ok(response) => {
			match response {
				user_dto::LoginResponse::Success { .. } => Ok(HttpResponse::Ok().json(response)),
				user_dto::LoginResponse::Error(_) => Ok(HttpResponse::Unauthorized().json(response)),
			}
		},
		Err(err) => Ok(err.error_response()),
	}
}

pub async fn get_me(
	pool: web::Data<DbPool>,
	req: HttpRequest,
) -> Result<HttpResponse> {
	let user_id = match AuthMiddleware::extract_user_id(&req) {
		Ok(id) => id,
		Err(err) => return Ok(err.error_response()),
	};

	match UserService::get_user_profile_json(pool, user_id).await {
		Ok(response) => Ok(HttpResponse::Ok().json(response)),
		Err(err) => Ok(err.error_response()),
	}
}

pub async fn update_me(
	pool: web::Data<DbPool>,
	req: HttpRequest,
	body: web::Json<user_dto::UpdateUserRequest>,
) -> Result<HttpResponse> {
	let user_id = match AuthMiddleware::extract_user_id(&req) {
		Ok(id) => id,
		Err(err) => return Ok(err.error_response()),
	};

	let request = body.into_inner();

	match UserService::update_user_profile_json(pool, user_id, request).await {
		Ok(response) => Ok(HttpResponse::Ok().json(response)),
		Err(err) => Ok(err.error_response()),
	}
}

pub async fn refresh_token(
	pool: web::Data<DbPool>,
	body: web::Json<user_dto::RefreshTokenRequest>,
) -> Result<HttpResponse> {
	let request = body.into_inner();

	match UserService::refresh_token_json(pool, request).await {
		Ok(response) => Ok(HttpResponse::Ok().json(response)),
		Err(err) => Ok(err.error_response()),
	}
}


// POST /users/forgot-password
pub async fn forgot_password(
	pool: web::Data<DbPool>,
	body: web::Json<user_dto::ForgotPasswordRequest>,
) -> Result<HttpResponse> {
	match UserService::forgot_password(pool, body.into_inner()).await {
		Ok(_) => Ok(HttpResponse::Ok().json(user_dto::GenericResponse { message: "Password reset link sent if email exists.".to_string() })),
		Err(err) => Ok(err.error_response()),
	}
}

// POST /users/reset-password
pub async fn reset_password(
	pool: web::Data<DbPool>,
	body: web::Json<user_dto::ResetPasswordRequest>,
) -> Result<HttpResponse> {
	match UserService::reset_password(pool, body.into_inner()).await {
		Ok(_) => Ok(HttpResponse::Ok().json(user_dto::GenericResponse { message: "Password reset successful.".to_string() })),
		Err(err) => Ok(err.error_response()),
	}
}

// POST /users/change-password
pub async fn change_password(
	pool: web::Data<DbPool>,
	req: HttpRequest,
	body: web::Json<user_dto::ChangePasswordRequest>,
) -> Result<HttpResponse> {
	let user_id = match AuthMiddleware::extract_user_id(&req) {
		Ok(id) => id,
		Err(err) => return Ok(err.error_response()),
	};
	match UserService::change_password(pool, user_id, body.into_inner()).await {
		Ok(_) => Ok(HttpResponse::Ok().json(user_dto::GenericResponse { message: "Password changed successfully.".to_string() })),
		Err(err) => Ok(err.error_response()),
	}
}

