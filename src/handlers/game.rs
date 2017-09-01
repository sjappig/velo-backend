use super::prelude::*;
use game::Game;

#[get("/game")]
pub fn games(pool: State<db::ConnectionPool>) -> HandlerResult {
    let conn = pool.get().map_err(super::error_to_json)?;
    let games = Game::get_all(&conn).map_err(super::error_to_json)?;
    Ok(Json(json!({
        "games": games,
    })))
}

#[post("/game", format = "application/json", data = "<game>")]
pub fn new_game(pool: State<db::ConnectionPool>, game: Json<Game>) -> HandlerResult {
    let conn = pool.get().map_err(super::error_to_json)?;
    db::insert_game(&conn, &game).map_err(super::error_to_json)?;
    Ok(Json(json!({
        "success": true,
    })))
}
