use actix_web::{web, HttpResponse, Result, ResponseError};
use crate::config::database::DbPool;
use crate::dto::uniform as uniform_dto;
use crate::services::uniform_services::UniformService;

use serde::Deserialize;

#[derive(Deserialize)]
pub struct UniformQuery {
    pub school_id: Option<i32>,
}

pub async fn list_uniforms(pool: web::Data<DbPool>, query: web::Query<UniformQuery>) -> Result<HttpResponse> {
    match UniformService::list_uniforms(pool, query.school_id).await {
        Ok(uniforms) => Ok(HttpResponse::Ok().json(uniforms)),
        Err(err) => Ok(err.error_response()),
    }
}

pub async fn get_uniform(pool: web::Data<DbPool>, path: web::Path<i32>) -> Result<HttpResponse> {
    let id = path.into_inner();
    match UniformService::get_uniform(pool, id).await {
        Ok(u) => Ok(HttpResponse::Ok().json(u)),
        Err(err) => Ok(err.error_response()),
    }
}

pub async fn create_uniform(pool: web::Data<DbPool>, body: web::Json<uniform_dto::CreateUniformRequest>) -> Result<HttpResponse> {
    match UniformService::create_uniform(pool, body.into_inner()).await {
        Ok(u) => Ok(HttpResponse::Created().json(u)),
        Err(err) => Ok(err.error_response()),
    }
}

pub async fn list_by_grade(pool: web::Data<DbPool>, path: web::Path<i32>) -> Result<HttpResponse> {
    let gid = path.into_inner();
    match UniformService::list_by_grade(pool, gid).await {
        Ok(u) => Ok(HttpResponse::Ok().json(u)),
        Err(err) => Ok(err.error_response()),
    }
}

pub async fn list_by_category(pool: web::Data<DbPool>, path: web::Path<i32>) -> Result<HttpResponse> {
    let cid = path.into_inner();
    match UniformService::list_by_category(pool, cid).await {
        Ok(u) => Ok(HttpResponse::Ok().json(u)),
        Err(err) => Ok(err.error_response()),
    }
}

#[derive(serde::Deserialize)]
pub struct UpdateStockRequest { pub stock_quantity: i32 }

pub async fn update_stock(pool: web::Data<DbPool>, path: web::Path<i32>, body: web::Json<UpdateStockRequest>) -> Result<HttpResponse> {
    let uid = path.into_inner();
    match UniformService::update_stock(pool, uid, body.stock_quantity).await {
        Ok(u) => Ok(HttpResponse::Ok().json(u)),
        Err(err) => Ok(err.error_response()),
    }
}
