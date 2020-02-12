use diesel::r2d2;
use diesel::{r2d2::ConnectionManager, PgConnection};

/// r2d2 postgres connection pool
pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub fn build_pool() -> Pool {
    let database_url = std::env::var("DATABASE_URL").expect("Database url must be set");

    let manager = ConnectionManager::<PgConnection>::new(database_url);
    r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to build connection pool")
}
