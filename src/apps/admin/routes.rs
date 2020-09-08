use crate::shared::*;
use crate::api_error::ApiError;
use actix_web::{get, post, web, HttpResponse};

#[post("/users")]
async fn create(
    gateways: web::Data<gateway::Gateways>,
    user: web::Json<user::UserMessage>,
) -> Result<HttpResponse, ApiError> {
    let conn = gateways.db.connection();
    let user = user::User::create(user.into_inner(), conn)?;
    Ok(HttpResponse::Ok().json(user))
}

#[get("/users/{id}")]
async fn get(
    gateways: web::Data<gateway::Gateways>,
    id: web::Path<String>,
) -> Result<HttpResponse, ApiError> {
    let conn = gateways.db.connection();
    let user = user::User::get(id.into_inner(), conn)?;
    Ok(HttpResponse::Ok().json(user))
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(get);
    cfg.service(create);
}
