use super::prelude::*;
use game::Game;

#[get("/game")]
pub fn games(pool: State<db::ConnectionPool>) -> HandlerResult {
    let conn = pool.get().map_err(super::error_to_json)?;
    Ok(JSON(json!({
                      "games": Game::get_all(&conn)
                  })))
}

#[post("/game", format = "application/json", data = "<game>")]
pub fn new_game(pool: State<db::ConnectionPool>, game: JSON<Game>) -> HandlerResult {
    let conn = pool.get().map_err(super::error_to_json)?;
    db::insert_game(&conn, &game).map_err(super::error_to_json)?;
    Ok(JSON(json!({
        "success": true
    })))
}
