#[macro_use]
extern crate diesel;

use actix_web::{get, middleware, App, HttpResponse, HttpServer, Responder};
use futures::future;
use shared::*;

pub mod apps;
pub mod schema;
pub mod shared;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();
    let cfg = load_config("config/dev");

    let gateways = gateway::new(&cfg);
    let gateways2 = gateways.clone();
    let s1 = HttpServer::new(move || { 
        App::new()
            .data(gateways.clone())
            .wrap(middleware::Logger::default())
            .configure(apps::admin::init_routes)
    })
    .bind("0.0.0.0:80")?
    .run();

    let s2 = HttpServer::new(move || {
        App::new()
            .data(gateways2.clone())
            .wrap(middleware::Logger::default())
            .service(hello_world)
    })
    .bind("0.0.0.0:2048")?
    .run();

    future::try_join(s1, s2).await?;
    Ok(())
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

