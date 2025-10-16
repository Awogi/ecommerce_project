use actix_web::{web, HttpResponse, Result, ResponseError};
use crate::config::database::DbPool;
use crate::services::SchoolService;
use crate::model::school::NewSchool;

pub async fn list_schools(pool: web::Data<DbPool>) -> Result<HttpResponse> {
    match SchoolService::list_schools(pool).await {
        Ok(s) => Ok(HttpResponse::Ok().json(s)),
        Err(err) => Ok(err.error_response()),
    }
}

pub async fn get_school(pool: web::Data<DbPool>, path: web::Path<i32>) -> Result<HttpResponse> {
    let id = path.into_inner();
    match SchoolService::get_school(pool, id).await {
        Ok(s) => Ok(HttpResponse::Ok().json(s)),
        Err(err) => Ok(err.error_response()),
    }
}

pub async fn create_school(pool: web::Data<DbPool>, body: web::Json<NewSchool>) -> Result<HttpResponse> {
    match SchoolService::create_school(pool, body.into_inner()).await {
        Ok(s) => Ok(HttpResponse::Created().json(s)),
        Err(err) => Ok(err.error_response()),
    }
}

pub async fn update_school(pool: web::Data<DbPool>, path: web::Path<i32>, body: web::Json<NewSchool>) -> Result<HttpResponse> {
    let id = path.into_inner();
    match SchoolService::update_school(pool, id, body.into_inner()).await {
        Ok(s) => Ok(HttpResponse::Ok().json(s)),
        Err(err) => Ok(err.error_response()),
    }
}

pub async fn delete_school(pool: web::Data<DbPool>, path: web::Path<i32>) -> Result<HttpResponse> {
    let id = path.into_inner();
    match SchoolService::delete_school(pool, id).await {
        Ok(r) => Ok(HttpResponse::Ok().json(r)),
        Err(err) => Ok(err.error_response()),
    }
}

pub async fn get_grades_for_school(pool: web::Data<DbPool>, path: web::Path<i32>) -> Result<HttpResponse> {
    let sid = path.into_inner();
    match crate::services::GradeService::list_by_school(pool, sid).await {
        Ok(g) => Ok(HttpResponse::Ok().json(g)),
        Err(err) => Ok(err.error_response()),
    }
}

pub async fn get_uniforms_for_school(pool: web::Data<DbPool>, path: web::Path<i32>) -> Result<HttpResponse> {
    let sid = path.into_inner();
    match crate::services::UniformService::list_uniforms(pool, Some(sid)).await {
        Ok(u) => Ok(HttpResponse::Ok().json(u)),
        Err(err) => Ok(err.error_response()),
    }
}
