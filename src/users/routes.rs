use crate::users::{User, UserMessage};
use crate::app;
use crate::api_error::ApiError;
use actix_web::{get, post, web, HttpResponse};

#[post("/users")]
async fn create(
    state: web::Data<app::State>,
    user: web::Json<UserMessage>,
) -> Result<HttpResponse, ApiError> {
    let conn = state.db.connection();
    let user = User::create(user.into_inner(), conn)?;
    Ok(HttpResponse::Ok().json(user))
}

#[get("/users/{id}")]
async fn get(
    state: web::Data<app::State>,
    id: web::Path<String>,
) -> Result<HttpResponse, ApiError> {
    let conn = state.db.connection();
    let user = User::get(id.into_inner(), conn)?;
    Ok(HttpResponse::Ok().json(user))
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(get);
    cfg.service(create);
}
