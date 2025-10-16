use actix_web::web;
use crate::controller::order_item_controller;

pub fn init_order_item_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/order-items")
            .route("/{id}", web::put().to(order_item_controller::update_item))
            .route("/{id}", web::delete().to(order_item_controller::delete_item)),
    );

    cfg.service(
        web::scope("/orders")
            .route("/{id}/items", web::get().to(order_item_controller::list_items))
            .route("/{id}/items", web::post().to(order_item_controller::add_item)),
    );
}
