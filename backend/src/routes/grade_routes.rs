use actix_web::web;
use crate::controller::grade_controller;

pub fn init_grade_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/grades")
            .route("/", web::get().to(grade_controller::list_grades))
            .route("/", web::post().to(grade_controller::create_grade))
            .route("/{id}", web::get().to(grade_controller::get_grade))
            .route("/{id}", web::put().to(grade_controller::update_grade))
            .route("/{id}", web::delete().to(grade_controller::delete_grade))
            .route("/school/{school_id}", web::get().to(grade_controller::list_by_school))
            .route("/{id}/uniforms", web::get().to(grade_controller::get_uniforms_for_grade)),
    );
}
