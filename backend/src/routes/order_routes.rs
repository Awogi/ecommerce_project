use actix_web::web;
use crate::controller::order_controller;

pub fn init_order_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/orders")
            .route("/", web::post().to(order_controller::create_order))
            .route("/", web::get().to(order_controller::list_orders))
            .route("/{id}", web::get().to(order_controller::get_order))
            .route("/{id}", web::put().to(order_controller::update_order_status))
            .route("/{id}", web::delete().to(order_controller::delete_order)),
    );

    cfg.service(
        web::scope("/schools")
            .route("/{id}/orders", web::get().to(order_controller::list_orders_for_school)),
    );
}
