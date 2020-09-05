use actix_web::{get, App, HttpServer, HttpResponse, Responder};

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello_world)
    })
    .bind("localhost:3000")?
    .run()
    .await
}

#[get("/")]
pub async fn hello_world() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}
