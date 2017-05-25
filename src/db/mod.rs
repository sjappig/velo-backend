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

pub fn insert_player(conn: &postgres::Connection, player: &Player) -> postgres::Result<u64> {
    conn.execute("INSERT INTO players(id, name, elo) VALUES($1, $2, $3)",
                 &[&player.id, &player.name, &1200])
}

pub fn insert_game(conn: &postgres::Connection, game: &Game) -> postgres::Result<u64> {
    conn.execute("INSERT INTO games(winner, loser, start_time, duration) VALUES($1, $2, $3, $4)",
                 &[&game.winner,
                   &game.loser,
                   &game.start_time,
                   &format!("INTERVAL '{} second'", game.duration.as_secs())])
}
