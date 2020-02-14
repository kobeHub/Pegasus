use diesel::r2d2;
use diesel::{r2d2::ConnectionManager, PgConnection};
use lazy_static::lazy_static;

use crate::errors::ApiError;

/// r2d2 postgres connection pool
type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;
pub type DbConn = r2d2::PooledConnection<ConnectionManager<PgConnection>>;

embed_migrations!();

lazy_static! {
    static ref POOL: Pool = {
        let database_url = std::env::var("DATABASE_URL").expect("Pegasus: Database url must be set");

        let manager = ConnectionManager::<PgConnection>::new(database_url);
        r2d2::Pool::builder()
            .build(manager)
            .expect("Pegasus: Failed to build connection pool")
    };
}

pub fn init() {
    info!("Initializing DB");
    lazy_static::initialize(&POOL);
    let conn = connection().expect("Pegasus: Failed to get db connection");
    embedded_migrations::run(&conn).unwrap();
}

pub fn connection() -> Result<DbConn, ApiError> {
    POOL.get()
        .map_err(|err| ApiError::new(500, format!("Failed to get db connection: {}", err)))
}
