pub mod database;

#[derive(Clone)]
pub struct Gateways {
    pub db: database::DB
}

pub fn new(cfg: &config::Config) -> Gateways {
    let database_url: String = cfg.get("DATABASE_URL").expect("DATABASE_URL not set");
    Gateways {
        db: database::open(database_url)
    }
}
