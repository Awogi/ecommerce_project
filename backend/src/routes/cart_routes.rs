use actix_web::web;
use crate::controller::cart_controller;

pub fn init_cart_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/cart")
            .route("/", web::get().to(cart_controller::list_cart))
            .route("/add", web::post().to(cart_controller::add_to_cart))
            .route("/item/{id}", web::put().to(cart_controller::update_cart_item))
            .route("/item/{id}", web::delete().to(cart_controller::remove_from_cart))
            .route("/clear", web::delete().to(cart_controller::clear_cart)),
    );
}
