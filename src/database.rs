use config::Config;
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};

pub type ConnectionPool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub fn new_connection_pool(cfg: &Config) -> ConnectionPool {
    let url: String = cfg.get("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(url);
    r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.")
}
