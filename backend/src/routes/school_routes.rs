use actix_web::web;
use crate::controller::school_controller;

pub fn init_school_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/schools")
            .route("/", web::get().to(school_controller::list_schools))
            .route("/", web::post().to(school_controller::create_school))
            .route("/{id}", web::get().to(school_controller::get_school))
            .route("/{id}", web::put().to(school_controller::update_school))
            .route("/{id}", web::delete().to(school_controller::delete_school))
            .route("/{id}/grades", web::get().to(school_controller::get_grades_for_school))
            .route("/{id}/uniforms", web::get().to(school_controller::get_uniforms_for_school)),
    );
}
