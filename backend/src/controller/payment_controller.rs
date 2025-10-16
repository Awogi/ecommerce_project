use actix_web::{web, HttpRequest, HttpResponse, Result, ResponseError};
use crate::config::database::DbPool;
use crate::services::payment_services::PaymentService;
use crate::middleware::auth::AuthMiddleware;
use crate::dto::payment as payment_dto;

pub async fn create_payment(pool: web::Data<DbPool>, req: HttpRequest, body: web::Json<payment_dto::CreatePaymentRequest>) -> Result<HttpResponse> {
    let user_id = match AuthMiddleware::extract_user_id(&req) { Ok(id) => id, Err(err) => return Ok(err.error_response()) };
    match PaymentService::create_payment(pool, user_id, body.into_inner()).await {
        Ok(p) => Ok(HttpResponse::Created().json(p)),
        Err(err) => Ok(err.error_response()),
    }
}

pub async fn list_payments(pool: web::Data<DbPool>) -> Result<HttpResponse> {
    match PaymentService::list_payments(pool).await {
        Ok(p) => Ok(HttpResponse::Ok().json(p)),
        Err(err) => Ok(err.error_response()),
    }
}

pub async fn get_payments_for_order(pool: web::Data<DbPool>, path: web::Path<i32>) -> Result<HttpResponse> {
    let oid = path.into_inner();
    match PaymentService::get_by_order(pool, oid).await {
        Ok(p) => Ok(HttpResponse::Ok().json(p)),
        Err(err) => Ok(err.error_response()),
    }
}

#[derive(serde::Deserialize)]
pub struct UpdatePaymentStatusRequest { pub status: String }

pub async fn update_payment_status(pool: web::Data<DbPool>, path: web::Path<i32>, body: web::Json<UpdatePaymentStatusRequest>) -> Result<HttpResponse> {
    let pid = path.into_inner();
    match PaymentService::update_status(pool, pid, body.status.clone()).await {
        Ok(_) => Ok(HttpResponse::Ok().json(crate::dto::common::GenericResponse { message: "Updated".to_string() })),
        Err(err) => Ok(err.error_response()),
    }
}

pub async fn delete_payment(pool: web::Data<DbPool>, path: web::Path<i32>) -> Result<HttpResponse> {
    let pid = path.into_inner();
    match PaymentService::delete_payment(pool, pid).await {
        Ok(_) => Ok(HttpResponse::Ok().json(crate::dto::common::GenericResponse { message: "Deleted".to_string() })),
        Err(err) => Ok(err.error_response()),
    }
}

pub async fn complete_payment_for_order(pool: web::Data<DbPool>, path: web::Path<i32>) -> Result<HttpResponse> {
    let oid = path.into_inner();
    match PaymentService::complete_payment(pool, oid).await {
        Ok(_) => Ok(HttpResponse::Ok().json(crate::dto::common::GenericResponse { message: "Completed".to_string() })),
        Err(err) => Ok(err.error_response()),
    }
}

pub async fn payment_summary(pool: web::Data<DbPool>) -> Result<HttpResponse> {
    match PaymentService::summary(pool).await {
        Ok(s) => Ok(HttpResponse::Ok().json(s)),
        Err(err) => Ok(err.error_response()),
    }
}
