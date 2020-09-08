#[macro_use]
extern crate diesel;

use actix_web::{get, middleware, App, HttpResponse, HttpServer, Responder};

mod app;
mod api_error;
mod database;
mod schema;
mod users;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    // Set up dependencies (e.g. database connection pool)
    let cfg = load_config("config/dev");

    // Start HTTP server
    let host: String = cfg.get("HOST").expect("Host not set");
    let port: String = cfg.get("PORT").expect("Port not set");
    let bind = format!("{}:{}", host, port);
    println!("📡 Starting server at: {}", &bind);
    HttpServer::new(move || {
        let state = app::build_state(&cfg);
        App::new()
            .data(state)
            .wrap(middleware::Logger::default())
            .configure(users::init_routes)
            .service(hello_world)
    })
    .bind(&bind)?
    .run()
    .await
}

fn load_config(filename: &str) -> config::Config {
    let mut cfg = config::Config::default();
    cfg.merge(config::File::with_name(filename)).unwrap();
    cfg
}

#[get("/")]
async fn hello_world() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

