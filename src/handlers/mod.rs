use game;
use player;
use rocket::response::{content, NamedFile};
use serde_json;
use std::path::{Path, PathBuf};
use std::time::Duration;
use chrono::Local;

#[derive(Debug, Serialize, PartialEq)]
struct Games {
    games: Vec<game::Game>
}

#[derive(Debug, Serialize, PartialEq)]
struct Players {
    players: Vec<player::Player>
}

#[get("/player")]
pub fn players() -> content::JSON<String> {
    let ret = Players { players: player::get_all() };
    content::JSON(serde_json::to_string(&ret).unwrap())
}

#[get("/player/<id>")]
pub fn player(id: &str) -> String {
    "Ismo".into()
}

#[get("/game")]
pub fn games() -> content::JSON<String> {
    let ret = Games { games: game::get_all() };
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
