pub mod user_routes;
pub mod uniform_routes;
pub mod cart_routes;
pub mod order_routes;
pub mod payment_routes;
pub mod school_routes;
pub mod grade_routes;
pub mod uniform_category_routes;
pub mod order_item_routes;

use actix_web::web;

pub fn init_routes(cfg: &mut web::ServiceConfig) {
	// Mount user routes and other route modules here
	cfg.service(web::scope("/").configure(user_routes::init_user_routes));
	cfg.service(web::scope("/").configure(uniform_routes::init_uniform_routes));
	cfg.service(web::scope("/").configure(cart_routes::init_cart_routes));
	cfg.service(web::scope("/").configure(order_routes::init_order_routes));
	cfg.service(web::scope("/").configure(payment_routes::init_payment_routes));
	cfg.service(web::scope("/").configure(school_routes::init_school_routes));
	cfg.service(web::scope("/").configure(grade_routes::init_grade_routes));
	cfg.service(web::scope("/").configure(uniform_category_routes::init_uniform_category_routes));
	cfg.service(web::scope("/").configure(order_item_routes::init_order_item_routes));
}
