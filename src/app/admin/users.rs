use crate::err::ApiError;
use crate::model::{User, CreateUserMessage};
use crate::gateway::Gateways;
use actix_web::{get, post, web::{Data, Json, Path}, HttpResponse};

#[post("/users")]
async fn create(
    gateways: Data<Gateways>,
    user: Json<CreateUserMessage>,
) -> Result<HttpResponse, ApiError> {
    let conn = gateways.db.connection();
    let user = User::create(user.into_inner(), conn)?;
    Ok(HttpResponse::Ok().json(user))
}

#[get("/users/{id}")]
async fn get(
    gateways: Data<Gateways>,
    id: Path<String>,
) -> Result<HttpResponse, ApiError> {
    let conn = gateways.db.connection();
    let user = User::get(id.into_inner(), conn)?;
    Ok(HttpResponse::Ok().json(user))
}
