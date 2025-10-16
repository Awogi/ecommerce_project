use actix_web::{web, HttpRequest, HttpResponse, Result, ResponseError};
use crate::config::database::DbPool;
use crate::services::order_services::OrderService;
use crate::middleware::auth::AuthMiddleware;
use crate::dto::order as order_dto;

pub async fn create_order(pool: web::Data<DbPool>, req: HttpRequest, body: web::Json<order_dto::CreateOrderRequest>) -> Result<HttpResponse> {
    let user_id = match AuthMiddleware::extract_user_id(&req) { Ok(id) => id, Err(err) => return Ok(err.error_response()) };
    match OrderService::create_order(pool, user_id, body.into_inner()).await {
        Ok(order) => Ok(HttpResponse::Created().json(order)),
        Err(err) => Ok(err.error_response()),
    }
}

pub async fn get_order(pool: web::Data<DbPool>, req: HttpRequest, path: web::Path<i32>) -> Result<HttpResponse> {
    let user_id = match AuthMiddleware::extract_user_id(&req) { Ok(id) => id, Err(err) => return Ok(err.error_response()) };
    let order_id = path.into_inner();
    match OrderService::get_order(pool, user_id, order_id).await {
        Ok(order) => Ok(HttpResponse::Ok().json(order)),
        Err(err) => Ok(err.error_response()),
    }
}

pub async fn list_orders(pool: web::Data<DbPool>, req: HttpRequest) -> Result<HttpResponse> {
    let user_id = match AuthMiddleware::extract_user_id(&req) { Ok(id) => id, Err(err) => return Ok(err.error_response()) };
    // placeholder admin check: check header x-admin == "true"
    let is_admin = req.headers().get("x-admin").map(|v| v == "true").unwrap_or(false);
    match OrderService::list_orders(pool, user_id, is_admin).await {
        Ok(o) => Ok(HttpResponse::Ok().json(o)),
        Err(err) => Ok(err.error_response()),
    }
}

#[derive(serde::Deserialize)]
pub struct UpdateStatusRequest { pub status: String }

pub async fn update_order_status(pool: web::Data<DbPool>, req: HttpRequest, path: web::Path<i32>, body: web::Json<UpdateStatusRequest>) -> Result<HttpResponse> {
    let _user_id = match AuthMiddleware::extract_user_id(&req) { Ok(id) => id, Err(err) => return Ok(err.error_response()) };
    let oid = path.into_inner();
    match OrderService::update_status(pool, oid, body.status.clone()).await {
        Ok(_) => Ok(HttpResponse::Ok().json(crate::dto::common::GenericResponse { message: "Status updated".to_string() })),
        Err(err) => Ok(err.error_response()),
    }
}

pub async fn delete_order(pool: web::Data<DbPool>, req: HttpRequest, path: web::Path<i32>) -> Result<HttpResponse> {
    let user_id = match AuthMiddleware::extract_user_id(&req) { Ok(id) => id, Err(err) => return Ok(err.error_response()) };
    let oid = path.into_inner();
    let is_admin = req.headers().get("x-admin").map(|v| v == "true").unwrap_or(false);
    match OrderService::delete_order(pool, user_id, oid, is_admin).await {
        Ok(_) => Ok(HttpResponse::Ok().json(crate::dto::common::GenericResponse { message: "Deleted".to_string() })),
        Err(err) => Ok(err.error_response()),
    }
}

pub async fn list_orders_for_school(pool: web::Data<DbPool>, path: web::Path<i32>) -> Result<HttpResponse> {
    let sid = path.into_inner();
    match OrderService::list_by_school(pool, sid).await {
        Ok(o) => Ok(HttpResponse::Ok().json(o)),
        Err(err) => Ok(err.error_response()),
    }
}
