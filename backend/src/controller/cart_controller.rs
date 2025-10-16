use actix_web::{web, HttpRequest, HttpResponse, Result, ResponseError};
use crate::config::database::DbPool;
use crate::services::cart_services::CartService;
use crate::dto::order as order_dto;
use crate::dto::uniform as uniform_dto;
use crate::middleware::auth::AuthMiddleware;

pub async fn list_cart(pool: web::Data<DbPool>, req: HttpRequest) -> Result<HttpResponse> {
    let user_id = match AuthMiddleware::extract_user_id(&req) { Ok(id) => id, Err(err) => return Ok(err.error_response()) };
    match CartService::list_cart(pool, user_id).await {
        Ok(items) => Ok(HttpResponse::Ok().json(items)),
        Err(err) => Ok(err.error_response()),
    }
}

pub async fn add_to_cart(pool: web::Data<DbPool>, req: HttpRequest, body: web::Json<order_dto::CreateOrderItemRequest>) -> Result<HttpResponse> {
    let user_id = match AuthMiddleware::extract_user_id(&req) { Ok(id) => id, Err(err) => return Ok(err.error_response()) };
    match CartService::add_to_cart(pool, user_id, body.into_inner()).await {
        Ok(_) => Ok(HttpResponse::Ok().json(crate::dto::common::GenericResponse { message: "Added to cart".to_string() })),
        Err(err) => Ok(err.error_response()),
    }
}

pub async fn remove_from_cart(pool: web::Data<DbPool>, req: HttpRequest, path: web::Path<i32>) -> Result<HttpResponse> {
    let user_id = match AuthMiddleware::extract_user_id(&req) { Ok(id) => id, Err(err) => return Ok(err.error_response()) };
    let cart_item_id = path.into_inner();
    match CartService::remove_from_cart(pool, user_id, cart_item_id).await {
        Ok(_) => Ok(HttpResponse::Ok().json(crate::dto::common::GenericResponse { message: "Removed".to_string() })),
        Err(err) => Ok(err.error_response()),
    }
}

pub async fn update_cart_item(pool: web::Data<DbPool>, req: HttpRequest, path: web::Path<i32>, body: web::Json<order_dto::CreateOrderItemRequest>) -> Result<HttpResponse> {
    let user_id = match AuthMiddleware::extract_user_id(&req) { Ok(id) => id, Err(err) => return Ok(err.error_response()) };
    let cart_item_id = path.into_inner();
    match CartService::update_cart_item(pool, user_id, cart_item_id, body.into_inner()).await {
        Ok(_) => Ok(HttpResponse::Ok().json(crate::dto::common::GenericResponse { message: "Updated".to_string() })),
        Err(err) => Ok(err.error_response()),
    }
}

pub async fn clear_cart(pool: web::Data<DbPool>, req: HttpRequest) -> Result<HttpResponse> {
    let user_id = match AuthMiddleware::extract_user_id(&req) { Ok(id) => id, Err(err) => return Ok(err.error_response()) };
    match CartService::clear_cart(pool, user_id).await {
        Ok(_) => Ok(HttpResponse::Ok().json(crate::dto::common::GenericResponse { message: "Cleared".to_string() })),
        Err(err) => Ok(err.error_response()),
    }
}
