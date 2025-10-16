use crate::config::database::DbPool;
use crate::error::AppError;
use crate::model::{school::NewSchool, school::School};
use crate::dto::common;
use crate::schema::schools::dsl::*;
use diesel::prelude::*;

pub struct SchoolService;

impl SchoolService {
    pub async fn list_schools(pool: actix_web::web::Data<DbPool>) -> Result<Vec<School>, AppError> {
        let mut conn = pool.get().map_err(|_| AppError::InternalError)?;
        let items = schools.load::<School>(&mut conn).map_err(|_| AppError::InternalError)?;
        Ok(items)
    }

    pub async fn get_school(pool: actix_web::web::Data<DbPool>, sid: i32) -> Result<School, AppError> {
        let mut conn = pool.get().map_err(|_| AppError::InternalError)?;
        let s = schools.find(sid).first::<School>(&mut conn).map_err(|_| AppError::NotFound)?;
        Ok(s)
    }

    pub async fn create_school(pool: actix_web::web::Data<DbPool>, req: NewSchool) -> Result<School, AppError> {
        let mut conn = pool.get().map_err(|_| AppError::InternalError)?;
        let inserted = diesel::insert_into(schools).values(&req).get_result::<School>(&mut conn).map_err(|_| AppError::InternalError)?;
        Ok(inserted)
    }

    pub async fn update_school(pool: actix_web::web::Data<DbPool>, sid: i32, req: NewSchool) -> Result<School, AppError> {
        let mut conn = pool.get().map_err(|_| AppError::InternalError)?;
        diesel::update(schools.find(sid)).set((name.eq(req.name), address.eq(req.address), contact_number.eq(req.contact_number))).get_result::<School>(&mut conn).map_err(|_| AppError::InternalError)
    }

    pub async fn delete_school(pool: actix_web::web::Data<DbPool>, sid: i32) -> Result<common::GenericResponse, AppError> {
        let mut conn = pool.get().map_err(|_| AppError::InternalError)?;
        diesel::delete(schools.find(sid)).execute(&mut conn).map_err(|_| AppError::InternalError)?;
        Ok(common::GenericResponse { message: "Deleted".to_string() })
    }
}
