use actix_web::{web, HttpResponse, Result, ResponseError};
use crate::config::database::DbPool;
use crate::services::UniformCategoryService;
use crate::model::uniform_category::NewUniformCategory;

pub async fn list_categories(pool: web::Data<DbPool>) -> Result<HttpResponse> {
    match UniformCategoryService::list_categories(pool).await {
        Ok(c) => Ok(HttpResponse::Ok().json(c)),
        Err(err) => Ok(err.error_response()),
    }
}

pub async fn get_category(pool: web::Data<DbPool>, path: web::Path<i32>) -> Result<HttpResponse> {
    let id = path.into_inner();
    match UniformCategoryService::get_category(pool, id).await {
        Ok(c) => Ok(HttpResponse::Ok().json(c)),
        Err(err) => Ok(err.error_response()),
    }
}

pub async fn create_category(pool: web::Data<DbPool>, body: web::Json<NewUniformCategory>) -> Result<HttpResponse> {
    match UniformCategoryService::create_category(pool, body.into_inner()).await {
        Ok(c) => Ok(HttpResponse::Created().json(c)),
        Err(err) => Ok(err.error_response()),
    }
}

pub async fn update_category(pool: web::Data<DbPool>, path: web::Path<i32>, body: web::Json<NewUniformCategory>) -> Result<HttpResponse> {
    let id = path.into_inner();
    match UniformCategoryService::update_category(pool, id, body.into_inner()).await {
        Ok(c) => Ok(HttpResponse::Ok().json(c)),
        Err(err) => Ok(err.error_response()),
    }
}

pub async fn delete_category(pool: web::Data<DbPool>, path: web::Path<i32>) -> Result<HttpResponse> {
    let id = path.into_inner();
    match UniformCategoryService::delete_category(pool, id).await {
        Ok(_) => Ok(HttpResponse::Ok().json(crate::dto::common::GenericResponse { message: "Deleted".to_string() })),
        Err(err) => Ok(err.error_response()),
    }
}

pub async fn get_uniforms_by_category(pool: web::Data<DbPool>, path: web::Path<i32>) -> Result<HttpResponse> {
    let cid = path.into_inner();
    match crate::services::UniformService::list_by_category(pool, cid).await {
        Ok(u) => Ok(HttpResponse::Ok().json(u)),
        Err(err) => Ok(err.error_response()),
    }
}
