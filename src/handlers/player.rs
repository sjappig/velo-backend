use player::Player;
use super::prelude::*;

#[get("/player")]
pub fn players(pool: State<db::ConnectionPool>) -> Result<JSON<serde_json::Value>, ()> {
    let conn = pool.get()
        .map_err(|_| /* TODO: Reasonable error type */ ())?;
    Ok(JSON(json!({
                      "players": Player::get_all(&conn)
                  })))
}

#[get("/player/<id>")]
pub fn player(id: String) -> String {
    "Ismo".into()
}

#[post("/player", format = "application/json", data = "<player>")]
pub fn new_player(pool: State<db::ConnectionPool>,
                  player: rocket_contrib::JSON<Player>)
                  -> Result<content::JSON<String>, ()> {
    let conn = pool.get()
        .map_err(|_| /* TODO: Reasonable error type */ ())?;
    db::insert_player(&conn, &player)
        .map_err(|_| /* TODO: Reasonable error type */())?;
    Ok(content::JSON("OK".into()))
}
