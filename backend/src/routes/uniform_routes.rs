use actix_web::web;
use crate::controller::uniform_controller;

pub fn init_uniform_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/uniforms")
            .route("/", web::get().to(uniform_controller::list_uniforms))
            .route("/{id}", web::get().to(uniform_controller::get_uniform))
            .route("/", web::post().to(uniform_controller::create_uniform)),
    );

    cfg.service(
        web::scope("/uniforms")
            .route("/grade/{id}", web::get().to(uniform_controller::list_by_grade))
            .route("/category/{id}", web::get().to(uniform_controller::list_by_category))
            .route("/{id}/stock", web::patch().to(uniform_controller::update_stock)),
    );
}
