use player::Player;
use super::prelude::*;

#[get("/player")]
pub fn players(pool: State<db::ConnectionPool>) -> HandlerResult {
    let conn = pool.get().map_err(super::error_to_json)?;
    let players = Player::get_all(&conn).map_err(super::error_to_json)?;
    Ok(JSON(json!({
        "players": players,
    })))
}

#[get("/player/<id>")]
pub fn player(id: String) -> String {
    "Ismo".into()
}

#[post("/player", format = "application/json", data = "<player>")]
pub fn new_player(pool: State<db::ConnectionPool>,
                  player: rocket_contrib::JSON<Player>)
                  -> HandlerResult {
    let conn = pool.get().map_err(super::error_to_json)?;
    db::insert_player(&conn, &player)
        .map_err(super::error_to_json)?;
    Ok(JSON(json!({
        "success": true
    })))
}
