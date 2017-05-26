use super::prelude::*;
use game::Game;

#[get("/game")]
pub fn games(pool: State<db::ConnectionPool>) -> content::JSON<String> {
    let conn = pool.get().unwrap();
    content::JSON(serde_json::to_string(&Game::get_all(&conn)).unwrap())
}

#[post("/game", format = "application/json", data = "<game>")]
pub fn new_game(pool: State<db::ConnectionPool>,
                game: rocket_contrib::JSON<Game>)
                -> content::JSON<String> {
    let conn = pool.get().unwrap();
    db::insert_game(&conn, &game).unwrap();
    content::JSON("OK".into())
}
