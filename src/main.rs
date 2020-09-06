use actix_web::{get, App, HttpServer, HttpResponse, Responder};
use log::info;
use env_logger;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    info!("📡 Starting server on port 3000");
    HttpServer::new(|| {
        App::new()
            .service(hello_world)
    })
    .bind("0.0.0.0:3000")?
    .run()
    .await
}

#[get("/")]
pub async fn hello_world() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}
