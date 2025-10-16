use actix_web::web;
use crate::controller::payment_controller;

pub fn init_payment_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/payments")
            .route("/", web::post().to(payment_controller::create_payment))
            .route("/", web::get().to(payment_controller::list_payments))
            .route("/summary", web::get().to(payment_controller::payment_summary))
            .route("/{id}", web::put().to(payment_controller::update_payment_status))
            .route("/{id}", web::delete().to(payment_controller::delete_payment))
            .route("/order/{id}", web::get().to(payment_controller::get_payments_for_order))
            .route("/order/{id}/complete", web::post().to(payment_controller::complete_payment_for_order)),
    );
}
