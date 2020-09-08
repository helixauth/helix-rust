use super::users;
use actix_web::web::ServiceConfig;

pub fn init_routes(cfg: &mut ServiceConfig) {
    cfg.service(users::create);
    cfg.service(users::get);
}
