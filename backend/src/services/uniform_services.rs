use crate::config::database::DbPool;
use crate::dto::uniform as uniform_dto;
use crate::error::AppError;
use crate::model::uniform::{Uniform, NewUniform};
use crate::schema::uniforms::dsl::*;
use diesel::prelude::*;

pub struct UniformService;

impl UniformService {
    pub async fn list_uniforms(pool: actix_web::web::Data<DbPool>, school: Option<i32>) -> Result<Vec<uniform_dto::UniformResponse>, AppError> {
        let mut conn = pool.get().map_err(|_| AppError::InternalError)?;
        // Use as_select to ensure selectable projection matches the model
        let items = match school {
            Some(sid) => uniforms.filter(school_id.eq(sid)).select(Uniform::as_select()).load::<Uniform>(&mut conn).map_err(|_| AppError::InternalError)?,
            None => uniforms.select(Uniform::as_select()).load::<Uniform>(&mut conn).map_err(|_| AppError::InternalError)?,
        };
        let res = items.into_iter().map(|u| uniform_dto::UniformResponse {
            id: u.id,
            name: u.name,
            school_id: u.school_id,
            grade_id: u.grade_id,
            category_id: u.category_id,
            size: u.size,
            price: u.price,
            stock_quantity: u.stock_quantity.unwrap_or(0),
            image_url: u.image_url,
            created_at: u.created_at.map(|d| d.to_string()).unwrap_or_default(),
        }).collect();
        Ok(res)
    }

    pub async fn get_uniform(pool: actix_web::web::Data<DbPool>, uid: i32) -> Result<uniform_dto::UniformResponse, AppError> {
        let mut conn = pool.get().map_err(|_| AppError::InternalError)?;
    let u = uniforms.find(uid).select(Uniform::as_select()).first::<Uniform>(&mut conn).map_err(|_| AppError::InternalError)?;
        Ok(uniform_dto::UniformResponse {
            id: u.id,
            name: u.name,
            school_id: u.school_id,
            grade_id: u.grade_id,
            category_id: u.category_id,
            size: u.size,
            price: u.price,
            stock_quantity: u.stock_quantity.unwrap_or(0),
            image_url: u.image_url,
            created_at: u.created_at.map(|d| d.to_string()).unwrap_or_default(),
        })
    }

    pub async fn create_uniform(pool: actix_web::web::Data<DbPool>, req: uniform_dto::CreateUniformRequest) -> Result<uniform_dto::UniformResponse, AppError> {
        let mut conn = pool.get().map_err(|_| AppError::InternalError)?;
        let new = NewUniform {
            name: req.name,
            school_id: req.school_id,
            grade_id: req.grade_id,
            category_id: req.category_id,
            size: req.size,
            price: req.price,
            stock_quantity: req.stock_quantity.or(Some(0)),
            image_url: req.image_url,
        };

    let inserted = diesel::insert_into(uniforms).values(&new).get_result::<Uniform>(&mut conn).map_err(|_| AppError::InternalError)?;
        Ok(uniform_dto::UniformResponse {
            id: inserted.id,
            name: inserted.name,
            school_id: inserted.school_id,
            grade_id: inserted.grade_id,
            category_id: inserted.category_id,
            size: inserted.size,
            price: inserted.price,
            stock_quantity: inserted.stock_quantity.unwrap_or(0),
            image_url: inserted.image_url,
            created_at: inserted.created_at.map(|d| d.to_string()).unwrap_or_default(),
        })
    }

    pub async fn list_by_grade(pool: actix_web::web::Data<DbPool>, gid: i32) -> Result<Vec<uniform_dto::UniformResponse>, AppError> {
        let mut conn = pool.get().map_err(|_| AppError::InternalError)?;
    let items = uniforms.filter(grade_id.eq(gid)).select(Uniform::as_select()).load::<Uniform>(&mut conn).map_err(|_| AppError::InternalError)?;
        let res = items.into_iter().map(|u| uniform_dto::UniformResponse {
            id: u.id,
            name: u.name,
            school_id: u.school_id,
            grade_id: u.grade_id,
            category_id: u.category_id,
            size: u.size,
            price: u.price,
            stock_quantity: u.stock_quantity.unwrap_or(0),
            image_url: u.image_url,
            created_at: u.created_at.map(|d| d.to_string()).unwrap_or_default(),
        }).collect();
        Ok(res)
    }

    pub async fn list_by_category(pool: actix_web::web::Data<DbPool>, cid: i32) -> Result<Vec<uniform_dto::UniformResponse>, AppError> {
        let mut conn = pool.get().map_err(|_| AppError::InternalError)?;
    let items = uniforms.filter(category_id.eq(cid)).select(Uniform::as_select()).load::<Uniform>(&mut conn).map_err(|_| AppError::InternalError)?;
        let res = items.into_iter().map(|u| uniform_dto::UniformResponse {
            id: u.id,
            name: u.name,
            school_id: u.school_id,
            grade_id: u.grade_id,
            category_id: u.category_id,
            size: u.size,
            price: u.price,
            stock_quantity: u.stock_quantity.unwrap_or(0),
            image_url: u.image_url,
            created_at: u.created_at.map(|d| d.to_string()).unwrap_or_default(),
        }).collect();
        Ok(res)
    }

    pub async fn update_stock(pool: actix_web::web::Data<DbPool>, uid: i32, new_stock: i32) -> Result<uniform_dto::UniformResponse, AppError> {
        let mut conn = pool.get().map_err(|_| AppError::InternalError)?;
        diesel::update(uniforms.find(uid)).set(stock_quantity.eq(new_stock)).get_result::<Uniform>(&mut conn).map_err(|_| AppError::InternalError).map(|u| uniform_dto::UniformResponse {
            id: u.id,
            name: u.name,
            school_id: u.school_id,
            grade_id: u.grade_id,
            category_id: u.category_id,
            size: u.size,
            price: u.price,
            stock_quantity: u.stock_quantity.unwrap_or(0),
            image_url: u.image_url,
            created_at: u.created_at.map(|d| d.to_string()).unwrap_or_default(),
        })
    }
}
