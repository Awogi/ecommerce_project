use actix_web::web;
use crate::controller::uniform_category_controller;

pub fn init_uniform_category_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/uniform-categories")
            .route("/", web::get().to(uniform_category_controller::list_categories))
            .route("/", web::post().to(uniform_category_controller::create_category))
            .route("/{id}", web::get().to(uniform_category_controller::get_category))
            .route("/{id}", web::put().to(uniform_category_controller::update_category))
            .route("/{id}", web::delete().to(uniform_category_controller::delete_category))
            .route("/category/{id}/uniforms", web::get().to(uniform_category_controller::get_uniforms_by_category))
            .route("/{id}/uniforms", web::get().to(uniform_category_controller::get_uniforms_by_category)),
    );
}
