use actix_web::{web, HttpResponse, Result, ResponseError};
use crate::config::database::DbPool;
use crate::services::GradeService;
use crate::model::grade::NewGrade;

pub async fn list_grades(pool: web::Data<DbPool>) -> Result<HttpResponse> {
    match GradeService::list_grades(pool).await {
        Ok(g) => Ok(HttpResponse::Ok().json(g)),
        Err(err) => Ok(err.error_response()),
    }
}

pub async fn get_grade(pool: web::Data<DbPool>, path: web::Path<i32>) -> Result<HttpResponse> {
    let id = path.into_inner();
    match GradeService::get_grade(pool, id).await {
        Ok(g) => Ok(HttpResponse::Ok().json(g)),
        Err(err) => Ok(err.error_response()),
    }
}

pub async fn list_by_school(pool: web::Data<DbPool>, path: web::Path<i32>) -> Result<HttpResponse> {
    let sid = path.into_inner();
    match GradeService::list_by_school(pool, sid).await {
        Ok(g) => Ok(HttpResponse::Ok().json(g)),
        Err(err) => Ok(err.error_response()),
    }
}

pub async fn create_grade(pool: web::Data<DbPool>, body: web::Json<NewGrade>) -> Result<HttpResponse> {
    match GradeService::create_grade(pool, body.into_inner()).await {
        Ok(g) => Ok(HttpResponse::Created().json(g)),
        Err(err) => Ok(err.error_response()),
    }
}

pub async fn update_grade(pool: web::Data<DbPool>, path: web::Path<i32>, body: web::Json<NewGrade>) -> Result<HttpResponse> {
    let id = path.into_inner();
    match GradeService::update_grade(pool, id, body.into_inner()).await {
        Ok(g) => Ok(HttpResponse::Ok().json(g)),
        Err(err) => Ok(err.error_response()),
    }
}

pub async fn delete_grade(pool: web::Data<DbPool>, path: web::Path<i32>) -> Result<HttpResponse> {
    let id = path.into_inner();
    match GradeService::delete_grade(pool, id).await {
        Ok(_) => Ok(HttpResponse::Ok().json(crate::dto::common::GenericResponse { message: "Deleted".to_string() })),
        Err(err) => Ok(err.error_response()),
    }
}

pub async fn get_uniforms_for_grade(pool: web::Data<DbPool>, path: web::Path<i32>) -> Result<HttpResponse> {
    let gid = path.into_inner();
    match crate::services::UniformService::list_by_grade(pool, gid).await {
        Ok(u) => Ok(HttpResponse::Ok().json(u)),
        Err(err) => Ok(err.error_response()),
    }
}
