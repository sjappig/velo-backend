use db;
use game::Game;
use player::Player;
use rocket::State;
use rocket::response::{content, NamedFile};
use rocket_contrib;
use serde_json;
use std::path::{Path, PathBuf};

#[derive(Debug, Serialize, PartialEq)]
struct Games {
    games: Vec<Game>,
}

#[derive(Debug, Serialize, PartialEq)]
struct Players {
    players: Vec<Player>,
}

#[get("/player")]
pub fn players(pool: State<db::ConnectionPool>) -> content::JSON<String> {
    let conn = pool.get().unwrap();
    let ret = Players { players: Player::get_all(&conn) };
    content::JSON(serde_json::to_string(&ret).unwrap())
}

#[get("/player/<id>")]
pub fn player(id: String) -> String {
    "Ismo".into()
}

#[post("/player", format = "application/json", data = "<player>")]
pub fn new_player(pool: State<db::ConnectionPool>,
                  player: rocket_contrib::JSON<Player>)
                  -> content::JSON<String> {
    let conn = pool.get().unwrap();
    db::insert_player(&conn, &player).unwrap();
    content::JSON("jee".into())
}

#[get("/game")]
pub fn games(pool: State<db::ConnectionPool>) -> content::JSON<String> {
    let conn = pool.get().unwrap();
    let ret = Games { games: Game::get_all(&conn) };
    content::JSON(serde_json::to_string(&ret).unwrap())
}

#[get("/")]
pub fn root() -> Option<NamedFile> {
    NamedFile::open(Path::new("static/index.html")).ok()
}

#[get("/<file..>", rank=2)]
pub fn files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("static/").join(file)).ok()
}

#[error(404)]
pub fn not_found() -> content::HTML<String> {
    content::HTML(r#"<img src="404.jpg"/>"#.into())
}
