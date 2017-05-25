use chrono;
use game::Game;
use player::Player;
use {r2d2, r2d2_postgres, postgres};

pub type ConnectionPool = r2d2::Pool<r2d2_postgres::PostgresConnectionManager>;

pub fn get_connection_pool() -> Result<ConnectionPool, r2d2::InitializationError> {
    let config = r2d2::Config::default();
    let connection_manager =
        r2d2_postgres::PostgresConnectionManager::new("postgres://postgres@localhost/velo_backend",
                                                      r2d2_postgres::TlsMode::None)
                .unwrap();
    r2d2::Pool::new(config, connection_manager)
}

pub fn insert_player(conn: postgres::Connection, id: &str, name: &str) -> postgres::Result<u64> {
    conn.execute("INSERT INTO players(id, name) VALUES(?, ?)", &[&id, &name])
}

pub fn insert_game(conn: postgres::Connection, game: &Game) -> postgres::Result<u64> {
    conn.execute("INSERT INTO games(winner, loser, start_time, duration) VALUES(?, ?, ?, ?)",
                 &[&game.winner,
                   &game.loser,
                   &game.start_time,
                   &format!("INTERVAL '{} second'", game.duration.as_secs())])
}
