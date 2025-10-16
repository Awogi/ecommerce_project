use crate::config::database::DbPool;
use crate::error::AppError;
use diesel::prelude::*;
use crate::schema::{payments, orders};
use crate::model::payment::NewPayment;

pub struct PaymentService;

impl PaymentService {
    pub async fn create_payment(pool: actix_web::web::Data<DbPool>, _user: i32, req: crate::dto::payment::CreatePaymentRequest) -> Result<crate::dto::payment::PaymentResponse, AppError> {
        let mut conn = pool.get().map_err(|_| AppError::InternalError)?;
        // In real life verify order belongs to user etc. For now, assume OK
    let new = NewPayment { order_id: req.order_id, payment_method: req.payment_method, amount: req.amount, status: Some("completed".to_string()) };
    let inserted = diesel::insert_into(payments::table).values(&new).get_result::<crate::model::payment::Payment>(&mut conn).map_err(|_| AppError::InternalError)?;

        // Mark order as completed
        diesel::update(orders::table.find(req.order_id)).set(orders::dsl::status.eq("completed")).execute(&mut conn).ok();

        Ok(crate::dto::payment::PaymentResponse { id: inserted.id, order_id: inserted.order_id, payment_method: inserted.payment_method, amount: inserted.amount, status: inserted.status.clone(), paid_at: inserted.paid_at.map(|d| d.to_string()) })
    }

    pub async fn list_payments(pool: actix_web::web::Data<DbPool>) -> Result<Vec<crate::dto::payment::PaymentResponse>, AppError> {
        let mut conn = pool.get().map_err(|_| AppError::InternalError)?;
        let items = payments::dsl::payments.select(crate::model::payment::Payment::as_select()).load::<crate::model::payment::Payment>(&mut conn).map_err(|_| AppError::InternalError)?;
        let res = items.into_iter().map(|p| crate::dto::payment::PaymentResponse { id: p.id, order_id: p.order_id, payment_method: p.payment_method, amount: p.amount, status: p.status.clone(), paid_at: p.paid_at.map(|d| d.to_string()) }).collect();
        Ok(res)
    }

    pub async fn get_by_order(pool: actix_web::web::Data<DbPool>, oid: i32) -> Result<Vec<crate::dto::payment::PaymentResponse>, AppError> {
        let mut conn = pool.get().map_err(|_| AppError::InternalError)?;
        let items = payments::dsl::payments.filter(payments::dsl::order_id.eq(oid)).select(crate::model::payment::Payment::as_select()).load::<crate::model::payment::Payment>(&mut conn).map_err(|_| AppError::InternalError)?;
        let res = items.into_iter().map(|p| crate::dto::payment::PaymentResponse { id: p.id, order_id: p.order_id, payment_method: p.payment_method, amount: p.amount, status: p.status.clone(), paid_at: p.paid_at.map(|d| d.to_string()) }).collect();
        Ok(res)
    }

    pub async fn update_status(pool: actix_web::web::Data<DbPool>, pid: i32, new_status: String) -> Result<(), AppError> {
        let mut conn = pool.get().map_err(|_| AppError::InternalError)?;
        diesel::update(payments::dsl::payments.find(pid)).set(payments::dsl::status.eq(new_status)).execute(&mut conn).map_err(|_| AppError::InternalError)?;
        Ok(())
    }

    pub async fn delete_payment(pool: actix_web::web::Data<DbPool>, pid: i32) -> Result<(), AppError> {
        let mut conn = pool.get().map_err(|_| AppError::InternalError)?;
        diesel::delete(payments::dsl::payments.find(pid)).execute(&mut conn).map_err(|_| AppError::InternalError)?;
        Ok(())
    }

    pub async fn complete_payment(pool: actix_web::web::Data<DbPool>, oid: i32) -> Result<(), AppError> {
        let mut conn = pool.get().map_err(|_| AppError::InternalError)?;
        diesel::update(payments::dsl::payments.filter(payments::dsl::order_id.eq(oid))).set(payments::dsl::status.eq("completed")).execute(&mut conn).map_err(|_| AppError::InternalError)?;
        diesel::update(orders::dsl::orders.find(oid)).set(orders::dsl::status.eq("completed")).execute(&mut conn).ok();
        Ok(())
    }

    pub async fn summary(pool: actix_web::web::Data<DbPool>) -> Result<crate::dto::payment::PaymentSummaryResponse, AppError> {
        let mut conn = pool.get().map_err(|_| AppError::InternalError)?;
        use diesel::dsl::sum;
        let total_paid: Option<f64> = payments::dsl::payments.filter(payments::dsl::status.eq("completed")).select(sum(payments::dsl::amount)).first(&mut conn).map_err(|_| AppError::InternalError)?;
        let pending_count: i64 = payments::dsl::payments.filter(payments::dsl::status.eq("pending")).count().get_result(&mut conn).map_err(|_| AppError::InternalError)?;
        let failed_count: i64 = payments::dsl::payments.filter(payments::dsl::status.eq("failed")).count().get_result(&mut conn).map_err(|_| AppError::InternalError)?;
        Ok(crate::dto::payment::PaymentSummaryResponse { total_paid: total_paid.unwrap_or(0.0), pending: pending_count as i64, failed: failed_count as i64 })
    }
}
