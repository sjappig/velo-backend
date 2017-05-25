use {diesel, r2d2, r2d2_diesel};

pub mod schema;

pub type ConnectionType = diesel::sqlite::SqliteConnection;
pub type ConnectionPool = r2d2::Pool<r2d2_diesel::ConnectionManager<ConnectionType>>;

use diesel::connection::SimpleConnection;

pub fn get_connection_pool() -> Result<ConnectionPool, r2d2::InitializationError> {
    let config = r2d2::Config::default();
    let connection_manager = r2d2_diesel::ConnectionManager::<ConnectionType>::new("db.sqlite3");
    r2d2::Pool::new(config, connection_manager).and_then(|pool| {
        let conn = pool.get().unwrap(); // TODO
        conn.batch_execute("PRAGMA foreign_keys = ON").unwrap(); // TODO
        Ok(pool)
    })
}
