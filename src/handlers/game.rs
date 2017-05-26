use super::prelude::*;
use game::Game;

#[get("/game")]
pub fn games(pool: State<db::ConnectionPool>) -> JSON<serde_json::Value> {
    let conn = pool.get().unwrap();
    JSON(json!({
                   "games": Game::get_all(&conn)
               }))
}

#[post("/game", format = "application/json", data = "<game>")]
pub fn new_game(pool: State<db::ConnectionPool>, game: JSON<Game>) -> content::JSON<String> {
    let conn = pool.get().unwrap();
    db::insert_game(&conn, &game).unwrap();
    content::JSON("OK".into())
}
