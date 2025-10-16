use crate::config::database::DbPool;
use crate::error::AppError;
use diesel::prelude::*;
use crate::schema::{orders, order_items, cart_items, uniforms};
use crate::model::{order::NewOrder, order_item::NewOrderItem};

pub struct OrderService;

impl OrderService {
    pub async fn create_order(pool: actix_web::web::Data<DbPool>, user: i32, req: crate::dto::order::CreateOrderRequest) -> Result<crate::dto::order::OrderResponse, AppError> {
        let mut conn = pool.get().map_err(|_| AppError::InternalError)?;

        // For simplicity, compute total price from provided items
        let mut total = 0.0f64;
        for it in &req.items {
            // fetch uniform price
            let u = uniforms::dsl::uniforms.find(it.uniform_id).select(crate::model::uniform::Uniform::as_select()).first::<crate::model::uniform::Uniform>(&mut conn).map_err(|_| AppError::InternalError)?;
            total += u.price * (it.quantity as f64);
        }

        let new_order = NewOrder { user_id: user, total_price: total, status: Some("pending".to_string()) };
    let inserted = diesel::insert_into(orders::table).values(&new_order).get_result::<crate::model::order::Order>(&mut conn).map_err(|_| AppError::InternalError)?;

        for it in req.items {
            // Note: price field in NewOrderItem is f64; use uniform price
            let uniform = uniforms::dsl::uniforms.find(it.uniform_id).select(crate::model::uniform::Uniform::as_select()).first::<crate::model::uniform::Uniform>(&mut conn).map_err(|_| AppError::InternalError)?;
            let new_item = NewOrderItem { order_id: inserted.id, uniform_id: it.uniform_id, quantity: it.quantity, price: uniform.price };
            diesel::insert_into(order_items::table).values(&new_item).execute(&mut conn).map_err(|_| AppError::InternalError)?;
        }

        // Optionally clear cart items for user
        diesel::delete(cart_items::dsl::cart_items.filter(cart_items::dsl::user_id.eq(user))).execute(&mut conn).ok();

        // Build response
    let items = order_items::dsl::order_items.filter(order_items::dsl::order_id.eq(inserted.id)).select(crate::model::order_item::OrderItem::as_select()).load::<crate::model::order_item::OrderItem>(&mut conn).map_err(|_| AppError::InternalError)?;
        let resp_items = items.into_iter().map(|i| crate::dto::order::OrderItemResponse { id: i.id, uniform_id: i.uniform_id, quantity: i.quantity, price: i.price }).collect();

    Ok(crate::dto::order::OrderResponse { id: inserted.id, user_id: inserted.user_id, total_price: inserted.total_price, status: inserted.status.unwrap_or_default(), created_at: inserted.created_at.map(|d| d.to_string()).unwrap_or_default(), items: resp_items })
    }

    pub async fn get_order(pool: actix_web::web::Data<DbPool>, user: i32, oid: i32) -> Result<crate::dto::order::OrderResponse, AppError> {
        let mut conn = pool.get().map_err(|_| AppError::InternalError)?;
    let ord = orders::dsl::orders.find(oid).select(crate::model::order::Order::as_select()).first::<crate::model::order::Order>(&mut conn).map_err(|_| AppError::InternalError)?;
    if ord.user_id != user { return Err(AppError::Unauthorized); }
    let items = order_items::dsl::order_items.filter(order_items::dsl::order_id.eq(ord.id)).select(crate::model::order_item::OrderItem::as_select()).load::<crate::model::order_item::OrderItem>(&mut conn).map_err(|_| AppError::InternalError)?;
    let resp_items = items.into_iter().map(|i| crate::dto::order::OrderItemResponse { id: i.id, uniform_id: i.uniform_id, quantity: i.quantity, price: i.price }).collect();
    Ok(crate::dto::order::OrderResponse { id: ord.id, user_id: ord.user_id, total_price: ord.total_price, status: ord.status.unwrap_or_default(), created_at: ord.created_at.map(|d| d.to_string()).unwrap_or_default(), items: resp_items })
    }

    pub async fn list_orders(pool: actix_web::web::Data<DbPool>, user: i32, is_admin: bool) -> Result<Vec<crate::dto::order::OrderResponse>, AppError> {
        let mut conn = pool.get().map_err(|_| AppError::InternalError)?;
        let items = if is_admin {
            orders::dsl::orders.select(crate::model::order::Order::as_select()).load::<crate::model::order::Order>(&mut conn).map_err(|_| AppError::InternalError)?
        } else {
            orders::dsl::orders.select(crate::model::order::Order::as_select()).filter(orders::dsl::user_id.eq(user)).load::<crate::model::order::Order>(&mut conn).map_err(|_| AppError::InternalError)?
        };
        let mut res = Vec::new();
        for ord in items {
            let its = order_items::dsl::order_items.filter(order_items::dsl::order_id.eq(ord.id)).select(crate::model::order_item::OrderItem::as_select()).load::<crate::model::order_item::OrderItem>(&mut conn).map_err(|_| AppError::InternalError)?;
            let resp_items = its.into_iter().map(|i| crate::dto::order::OrderItemResponse { id: i.id, uniform_id: i.uniform_id, quantity: i.quantity, price: i.price }).collect();
            res.push(crate::dto::order::OrderResponse { id: ord.id, user_id: ord.user_id, total_price: ord.total_price, status: ord.status.unwrap_or_default(), created_at: ord.created_at.map(|d| d.to_string()).unwrap_or_default(), items: resp_items });
        }
        Ok(res)
    }

    pub async fn update_status(pool: actix_web::web::Data<DbPool>, oid: i32, new_status: String) -> Result<(), AppError> {
        let mut conn = pool.get().map_err(|_| AppError::InternalError)?;
        diesel::update(orders::dsl::orders.find(oid)).set(orders::dsl::status.eq(new_status)).execute(&mut conn).map_err(|_| AppError::InternalError)?;
        Ok(())
    }

    pub async fn delete_order(pool: actix_web::web::Data<DbPool>, user: i32, oid: i32, is_admin: bool) -> Result<(), AppError> {
        let mut conn = pool.get().map_err(|_| AppError::InternalError)?;
        if is_admin {
            diesel::delete(orders::dsl::orders.find(oid)).execute(&mut conn).map_err(|_| AppError::InternalError)?;
        } else {
            diesel::delete(orders::dsl::orders.filter(orders::dsl::id.eq(oid)).filter(orders::dsl::user_id.eq(user))).execute(&mut conn).map_err(|_| AppError::InternalError)?;
        }
        Ok(())
    }

    pub async fn list_by_school(pool: actix_web::web::Data<DbPool>, sid: i32) -> Result<Vec<crate::dto::order::OrderResponse>, AppError> {
        // naive implementation: join uniforms -> order_items -> orders filtering by school_id
        let mut conn = pool.get().map_err(|_| AppError::InternalError)?;
        let joins = orders::table.inner_join(order_items::table.inner_join(uniforms::table.on(uniforms::dsl::id.eq(order_items::dsl::uniform_id))));
        // For brevity, load all orders and filter by inspecting items
        let all_orders = orders::dsl::orders.load::<crate::model::order::Order>(&mut conn).map_err(|_| AppError::InternalError)?;
        let mut res = Vec::new();
        for ord in all_orders {
            let its = order_items::dsl::order_items.filter(order_items::dsl::order_id.eq(ord.id)).load::<crate::model::order_item::OrderItem>(&mut conn).map_err(|_| AppError::InternalError)?;
            let mut include = false;
            for it in &its {
                let u = uniforms::dsl::uniforms.find(it.uniform_id).first::<crate::model::uniform::Uniform>(&mut conn).map_err(|_| AppError::InternalError)?;
                if u.school_id == sid { include = true; break; }
            }
            if include {
                let resp_items = its.into_iter().map(|i| crate::dto::order::OrderItemResponse { id: i.id, uniform_id: i.uniform_id, quantity: i.quantity, price: i.price }).collect();
                res.push(crate::dto::order::OrderResponse { id: ord.id, user_id: ord.user_id, total_price: ord.total_price, status: ord.status.unwrap_or_default(), created_at: ord.created_at.map(|d| d.to_string()).unwrap_or_default(), items: resp_items });
            }
        }
        Ok(res)
    }

    pub async fn list_items(pool: actix_web::web::Data<DbPool>, oid: i32) -> Result<Vec<crate::dto::order::OrderItemResponse>, AppError> {
        let mut conn = pool.get().map_err(|_| AppError::InternalError)?;
    let its = order_items::dsl::order_items.filter(order_items::dsl::order_id.eq(oid)).select(crate::model::order_item::OrderItem::as_select()).load::<crate::model::order_item::OrderItem>(&mut conn).map_err(|_| AppError::InternalError)?;
        Ok(its.into_iter().map(|i| crate::dto::order::OrderItemResponse { id: i.id, uniform_id: i.uniform_id, quantity: i.quantity, price: i.price }).collect())
    }

    pub async fn add_item(pool: actix_web::web::Data<DbPool>, oid: i32, req: crate::dto::order::CreateOrderItemRequest) -> Result<(), AppError> {
        let mut conn = pool.get().map_err(|_| AppError::InternalError)?;
    let uniform = uniforms::dsl::uniforms.find(req.uniform_id).select(crate::model::uniform::Uniform::as_select()).first::<crate::model::uniform::Uniform>(&mut conn).map_err(|_| AppError::InternalError)?;
        let new_item = NewOrderItem { order_id: oid, uniform_id: req.uniform_id, quantity: req.quantity, price: uniform.price };
        diesel::insert_into(order_items::table).values(&new_item).execute(&mut conn).map_err(|_| AppError::InternalError)?;
        Ok(())
    }

    pub async fn update_item(pool: actix_web::web::Data<DbPool>, item_id: i32, req: crate::dto::order::CreateOrderItemRequest) -> Result<(), AppError> {
        let mut conn = pool.get().map_err(|_| AppError::InternalError)?;
    diesel::update(order_items::dsl::order_items.find(item_id)).set((order_items::dsl::quantity.eq(req.quantity), order_items::dsl::uniform_id.eq(req.uniform_id))).execute(&mut conn).map_err(|_| AppError::InternalError)?;
        Ok(())
    }

    pub async fn delete_item(pool: actix_web::web::Data<DbPool>, item_id: i32) -> Result<(), AppError> {
        let mut conn = pool.get().map_err(|_| AppError::InternalError)?;
        diesel::delete(order_items::dsl::order_items.find(item_id)).execute(&mut conn).map_err(|_| AppError::InternalError)?;
        Ok(())
    }
}
