use {diesel, r2d2, r2d2_diesel};

pub mod schema;

pub type ConnectionType = diesel::sqlite::SqliteConnection;
pub type ConnectionPool = r2d2::Pool<r2d2_diesel::ConnectionManager<ConnectionType>>;

pub fn get_connection_pool() -> Result<ConnectionPool, r2d2::InitializationError> {
    let config = r2d2::Config::default();
    let connection_manager = r2d2_diesel::ConnectionManager::<ConnectionType>::new("db.sqlite3");
    r2d2::Pool::new(config, connection_manager)
}
