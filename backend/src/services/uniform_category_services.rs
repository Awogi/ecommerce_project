use crate::config::database::DbPool;
use crate::error::AppError;
use crate::model::{uniform_category::NewUniformCategory, uniform_category::UniformCategory};
use crate::schema::uniform_categories::dsl::*;
use diesel::prelude::*;

pub struct UniformCategoryService;

impl UniformCategoryService {
    pub async fn list_categories(pool: actix_web::web::Data<DbPool>) -> Result<Vec<UniformCategory>, AppError> {
        let mut conn = pool.get().map_err(|_| AppError::InternalError)?;
        let items = uniform_categories.load::<UniformCategory>(&mut conn).map_err(|_| AppError::InternalError)?;
        Ok(items)
    }

    pub async fn get_category(pool: actix_web::web::Data<DbPool>, cid: i32) -> Result<UniformCategory, AppError> {
        let mut conn = pool.get().map_err(|_| AppError::InternalError)?;
        let c = uniform_categories.find(cid).first::<UniformCategory>(&mut conn).map_err(|_| AppError::NotFound)?;
        Ok(c)
    }

    pub async fn create_category(pool: actix_web::web::Data<DbPool>, req: NewUniformCategory) -> Result<UniformCategory, AppError> {
        let mut conn = pool.get().map_err(|_| AppError::InternalError)?;
        let inserted = diesel::insert_into(uniform_categories).values(&req).get_result::<UniformCategory>(&mut conn).map_err(|_| AppError::InternalError)?;
        Ok(inserted)
    }

    pub async fn update_category(pool: actix_web::web::Data<DbPool>, cid: i32, req: NewUniformCategory) -> Result<UniformCategory, AppError> {
        let mut conn = pool.get().map_err(|_| AppError::InternalError)?;
        diesel::update(uniform_categories.find(cid)).set(name.eq(req.name)).get_result::<UniformCategory>(&mut conn).map_err(|_| AppError::InternalError)
    }

    pub async fn delete_category(pool: actix_web::web::Data<DbPool>, cid: i32) -> Result<(), AppError> {
        let mut conn = pool.get().map_err(|_| AppError::InternalError)?;
        diesel::delete(uniform_categories.find(cid)).execute(&mut conn).map_err(|_| AppError::InternalError)?;
        Ok(())
    }
}
