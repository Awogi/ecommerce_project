use actix_web::{web, HttpResponse, Result, HttpRequest, ResponseError};
use crate::config::database::DbPool;
use crate::services::order_services::OrderService;
use crate::middleware::auth::AuthMiddleware;

pub async fn list_items(pool: web::Data<DbPool>, path: web::Path<i32>) -> Result<HttpResponse> {
    let oid = path.into_inner();
    match OrderService::list_items(pool, oid).await {
        Ok(it) => Ok(HttpResponse::Ok().json(it)),
        Err(err) => Ok(err.error_response()),
    }
}

pub async fn add_item(pool: web::Data<DbPool>, req: HttpRequest, path: web::Path<i32>, body: web::Json<crate::dto::order::CreateOrderItemRequest>) -> Result<HttpResponse> {
    let _user_id = match AuthMiddleware::extract_user_id(&req) { Ok(id) => id, Err(err) => return Ok(err.error_response()) };
    let oid = path.into_inner();
    match OrderService::add_item(pool, oid, body.into_inner()).await {
        Ok(_) => Ok(HttpResponse::Created().json(crate::dto::common::GenericResponse { message: "Added".to_string() })),
        Err(err) => Ok(err.error_response()),
    }
}

pub async fn update_item(pool: web::Data<DbPool>, req: HttpRequest, path: web::Path<i32>, body: web::Json<crate::dto::order::CreateOrderItemRequest>) -> Result<HttpResponse> {
    let _user_id = match AuthMiddleware::extract_user_id(&req) { Ok(id) => id, Err(err) => return Ok(err.error_response()) };
    let item_id = path.into_inner();
    match OrderService::update_item(pool, item_id, body.into_inner()).await {
        Ok(_) => Ok(HttpResponse::Ok().json(crate::dto::common::GenericResponse { message: "Updated".to_string() })),
        Err(err) => Ok(err.error_response()),
    }
}

pub async fn delete_item(pool: web::Data<DbPool>, req: HttpRequest, path: web::Path<i32>) -> Result<HttpResponse> {
    let _user_id = match AuthMiddleware::extract_user_id(&req) { Ok(id) => id, Err(err) => return Ok(err.error_response()) };
    let item_id = path.into_inner();
    match OrderService::delete_item(pool, item_id).await {
        Ok(_) => Ok(HttpResponse::Ok().json(crate::dto::common::GenericResponse { message: "Deleted".to_string() })),
        Err(err) => Ok(err.error_response()),
    }
}
