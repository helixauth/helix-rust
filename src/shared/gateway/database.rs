use diesel::prelude::*;
use diesel::r2d2;

type ConnectionManager = r2d2::ConnectionManager<PgConnection>;
type ConnectionPool = r2d2::Pool<ConnectionManager>;
pub type Connection = r2d2::PooledConnection<ConnectionManager>;

#[derive(Clone)]
pub struct DB {
    pool: ConnectionPool
}

pub fn open(url: String) -> DB {
    let pool = ConnectionPool::builder()
        .build(ConnectionManager::new(url))
        .expect("Failed to create connection pool");
    DB {
        pool: pool.clone()
    }
}

impl DB {
    pub fn connection(&self) -> Connection {
        self.pool.get().expect("Failed to get connection from pool")
    }
}
