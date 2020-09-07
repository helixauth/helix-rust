#[macro_use]
extern crate diesel;

use actix_web::{get, middleware, post, web, App, Error, HttpResponse, HttpServer, Responder};
use uuid::Uuid;

mod actions;
mod database;
mod models;
mod schema;

struct AppState {
    db_conn_pool: database::ConnectionPool
}

#[get("/")]
async fn hello_world() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

/// Finds user by UID.
#[get("/user/{user_id}")]
async fn get_user(
    state: web::Data<AppState>,
    user_uid: web::Path<Uuid>,
) -> Result<HttpResponse, Error> {
    let user_uid = user_uid.into_inner();
    let conn = state.db_conn_pool.get().expect("couldn't get db connection from pool");

    // Use web::block to offload blocking Diesel code without blocking server thread
    let user = web::block(move || actions::find_user_by_uid(user_uid, &conn))
        .await
        .map_err(|e| {
            eprintln!("{}", e);
            HttpResponse::InternalServerError().finish()
        })?;

    if let Some(user) = user {
        Ok(HttpResponse::Ok().json(user))
    } else {
        let res = HttpResponse::NotFound()
            .body(format!("No user found with uid: {}", user_uid));
        Ok(res)
    }
}

/// Inserts new user with name defined in form.
#[post("/user")]
async fn add_user(
    state: web::Data<AppState>,
    form: web::Json<models::NewUser>,
) -> Result<HttpResponse, Error> {
    let conn = state.db_conn_pool.get().expect("couldn't get db connection from pool");

    // Use web::block to offload blocking Diesel code without blocking server thread
    let user = web::block(move || actions::insert_new_user(&form.name, &conn))
        .await
        .map_err(|e| {
            eprintln!("{}", e);
            HttpResponse::InternalServerError().finish()
        })?;

    Ok(HttpResponse::Ok().json(user))
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    // Set up dependencies (e.g. database connection pool)
    let cfg = load_config("config/dev");
    let db_conn_pool = database::new_connection_pool(&cfg);

    // Start HTTP server
    let bind = "0.0.0.0:3000";
    println!("📡 Starting server at: {}", &bind);
    HttpServer::new(move || {
        let state = AppState{
            db_conn_pool: db_conn_pool.clone()
        };
        App::new()
            .data(state)
            .wrap(middleware::Logger::default())
            .service(get_user)
            .service(add_user)
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
