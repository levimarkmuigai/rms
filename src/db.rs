use postgres::NoTls;
use r2d2::Pool;
use r2d2_postgres::PostgresConnectionManager;

pub type PgPool = Pool<PostgresConnectionManager<NoTls>>;

pub fn build_pool(database_url: &str) -> PgPool {
    let manager =
        PostgresConnectionManager::new(database_url.parse().expect("invlaid database url"), NoTls);

    Pool::builder()
        .max_size(8)
        .build(manager)
        .expect("failed to build connection pool")
}
