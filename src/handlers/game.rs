use super::prelude::*;
use game::Game;

#[get("/game")]
pub fn games(pool: State<db::ConnectionPool>) -> Result<JSON<serde_json::Value>, ()> {
    let conn = pool.get()
        .map_err(|_| /* TODO: Reasonable error type */ ())?;
    Ok(JSON(json!({
                      "games": Game::get_all(&conn)
                  })))
}

#[post("/game", format = "application/json", data = "<game>")]
pub fn new_game(pool: State<db::ConnectionPool>,
                game: JSON<Game>)
                -> Result<content::JSON<String>, ()> {
    let conn = pool.get()
        .map_err(|_| /* TODO: Reasonable error type */ ())?;
    db::insert_game(&conn, &game)
        .map_err(|_| /* TODO: Reasonable error type */ ())?;
    Ok(content::JSON("OK".into()))
}
