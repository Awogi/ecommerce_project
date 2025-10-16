use crate::config::database::DbPool;
use crate::error::AppError;
use crate::model::{cart_item::NewCartItem, cart_item::CartItem};
use crate::schema::cart_items::dsl::*;
use diesel::prelude::*;

pub struct CartService;

impl CartService {
    pub async fn list_cart(pool: actix_web::web::Data<DbPool>, user: i32) -> Result<Vec<CartItem>, AppError> {
        let mut conn = pool.get().map_err(|_| AppError::InternalError)?;
    let items = cart_items.filter(user_id.eq(user)).select(CartItem::as_select()).load::<CartItem>(&mut conn).map_err(|_| AppError::InternalError)?;
        Ok(items)
    }

    pub async fn add_to_cart(pool: actix_web::web::Data<DbPool>, user: i32, req: crate::dto::order::CreateOrderItemRequest) -> Result<(), AppError> {
        let mut conn = pool.get().map_err(|_| AppError::InternalError)?;
        let new = NewCartItem { user_id: user, uniform_id: req.uniform_id, quantity: Some(req.quantity) };
        diesel::insert_into(cart_items).values(&new).execute(&mut conn).map_err(|_| AppError::InternalError)?;
        Ok(())
    }

    pub async fn remove_from_cart(pool: actix_web::web::Data<DbPool>, user: i32, cart_id: i32) -> Result<(), AppError> {
        let mut conn = pool.get().map_err(|_| AppError::InternalError)?;
        diesel::delete(cart_items.filter(id.eq(cart_id)).filter(user_id.eq(user))).execute(&mut conn).map_err(|_| AppError::InternalError)?;
        Ok(())
    }

    pub async fn update_cart_item(pool: actix_web::web::Data<DbPool>, user: i32, cart_id: i32, req: crate::dto::order::CreateOrderItemRequest) -> Result<(), AppError> {
        let mut conn = pool.get().map_err(|_| AppError::InternalError)?;
        diesel::update(cart_items.filter(id.eq(cart_id)).filter(user_id.eq(user))).set(quantity.eq(req.quantity)).execute(&mut conn).map_err(|_| AppError::InternalError)?;
        Ok(())
    }

    pub async fn clear_cart(pool: actix_web::web::Data<DbPool>, user: i32) -> Result<(), AppError> {
        let mut conn = pool.get().map_err(|_| AppError::InternalError)?;
        diesel::delete(cart_items.filter(user_id.eq(user))).execute(&mut conn).map_err(|_| AppError::InternalError)?;
        Ok(())
    }
}
