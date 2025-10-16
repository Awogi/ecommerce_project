use crate::config::database::DbPool;
use crate::error::AppError;
use crate::model::{grade::NewGrade, grade::Grade};
use crate::schema::grades::dsl::*;
use diesel::prelude::*;

pub struct GradeService;

impl GradeService {
    pub async fn list_grades(pool: actix_web::web::Data<DbPool>) -> Result<Vec<Grade>, AppError> {
        let mut conn = pool.get().map_err(|_| AppError::InternalError)?;
        let items = grades.load::<Grade>(&mut conn).map_err(|_| AppError::InternalError)?;
        Ok(items)
    }

    pub async fn get_grade(pool: actix_web::web::Data<DbPool>, gid: i32) -> Result<Grade, AppError> {
        let mut conn = pool.get().map_err(|_| AppError::InternalError)?;
        let g = grades.find(gid).first::<Grade>(&mut conn).map_err(|_| AppError::NotFound)?;
        Ok(g)
    }

    pub async fn list_by_school(pool: actix_web::web::Data<DbPool>, sid: i32) -> Result<Vec<Grade>, AppError> {
        let mut conn = pool.get().map_err(|_| AppError::InternalError)?;
        let items = grades.filter(school_id.eq(sid)).load::<Grade>(&mut conn).map_err(|_| AppError::InternalError)?;
        Ok(items)
    }

    pub async fn create_grade(pool: actix_web::web::Data<DbPool>, req: NewGrade) -> Result<Grade, AppError> {
        let mut conn = pool.get().map_err(|_| AppError::InternalError)?;
        let inserted = diesel::insert_into(grades).values(&req).get_result::<Grade>(&mut conn).map_err(|_| AppError::InternalError)?;
        Ok(inserted)
    }

    pub async fn update_grade(pool: actix_web::web::Data<DbPool>, gid: i32, req: NewGrade) -> Result<Grade, AppError> {
        let mut conn = pool.get().map_err(|_| AppError::InternalError)?;
        diesel::update(grades.find(gid)).set((name.eq(req.name), school_id.eq(req.school_id))).get_result::<Grade>(&mut conn).map_err(|_| AppError::InternalError)
    }

    pub async fn delete_grade(pool: actix_web::web::Data<DbPool>, gid: i32) -> Result<(), AppError> {
        let mut conn = pool.get().map_err(|_| AppError::InternalError)?;
        diesel::delete(grades.find(gid)).execute(&mut conn).map_err(|_| AppError::InternalError)?;
        Ok(())
    }
}
