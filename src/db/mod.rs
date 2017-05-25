use {r2d2, r2d2_postgres};

pub type ConnectionPool = r2d2::Pool<r2d2_postgres::PostgresConnectionManager>;

pub fn get_connection_pool() -> Result<ConnectionPool, r2d2::InitializationError> {
    let config = r2d2::Config::default();
    let connection_manager =
        r2d2_postgres::PostgresConnectionManager::new("postgres://postgres@localhost/velo_backend",
                                                      r2d2_postgres::TlsMode::None)
                .unwrap();
    r2d2::Pool::new(config, connection_manager)
}
