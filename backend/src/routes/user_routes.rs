use actix_web::web;
use crate::controller::user_controller;

pub fn init_user_routes(cfg: &mut web::ServiceConfig) {
	cfg.service(
		web::scope("/users")
			.route("/register", web::post().to(user_controller::register_user))
			.route("/verify-email", web::post().to(user_controller::verify_email))
			.route("/login", web::post().to(user_controller::login_user))
			.route("/refresh", web::post().to(user_controller::refresh_token))
			.route("/me", web::get().to(user_controller::get_me))
			.route("/me", web::put().to(user_controller::update_me))
			.route("/forgot-password", web::post().to(user_controller::forgot_password))
			.route("/reset-password", web::post().to(user_controller::reset_password))
			.route("/change-password", web::post().to(user_controller::change_password)),
	);
}

