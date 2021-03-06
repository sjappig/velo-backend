use id::Id;
use player::Player;
use super::prelude::*;

#[get("/player")]
pub fn players(pool: State<db::ConnectionPool>) -> HandlerResult {
    let conn = pool.get().map_err(super::error_to_json)?;
    let players = Player::get_all(&conn).map_err(super::error_to_json)?;
    Ok(Json(json!({
        "players": players,
    })))
}

#[get("/player/<id>")]
pub fn player(pool: State<db::ConnectionPool>, id: String) -> HandlerResult {
    let id = Id::new(&id[..]).map_err(super::error_to_json)?;
    let conn = pool.get().map_err(super::error_to_json)?;
    let player = Player::get(&conn, &id).map_err(super::error_to_json)?;
    Ok(Json(json!({
        "players": &[player],
    })))
}

#[post("/player", format = "application/json", data = "<player>")]
pub fn new_player(
    pool: State<db::ConnectionPool>,
    player: rocket_contrib::Json<Player>,
) -> HandlerResult {
    let conn = pool.get().map_err(super::error_to_json)?;
    db::insert_player(&conn, &player).map_err(
        super::error_to_json,
    )?;
    Ok(Json(json!({
        "success": true
    })))
}
