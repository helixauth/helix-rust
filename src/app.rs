use crate::database;

pub struct State {
    pub db: database::DB
}

pub fn build_state(cfg: &config::Config) -> State {
    let database_url: String = cfg.get("DATABASE_URL").expect("DATABASE_URL not set");
    State {
        db: database::open(database_url)
    }
}
