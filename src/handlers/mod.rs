use game::Game;
use player::Player;
use rocket::response::{content, NamedFile};
use serde_json;
use std::path::{Path, PathBuf};
use std::time::Duration;
use chrono::Local;

#[derive(Debug, Serialize, PartialEq)]
struct Games {
    games: Vec<Game>
}

#[derive(Debug, Serialize, PartialEq)]
struct Players {
    players: Vec<Player>
}

#[get("/player")]
pub fn players() -> content::JSON<String> {
    let ret = Players { players: vec![ Player{ name: "Ismo".into(), id: "123".into(), elo: 123 } ] };
    content::JSON(serde_json::to_string(&ret).unwrap())
}

#[get("/player/<id>")]
pub fn player(id: &str) -> String {
    "Ismo".into()
}

#[get("/game")]
pub fn games() -> content::JSON<String> {
    let ret = Games { games: vec![ Game{ start_time: Local::now(), duration: Duration::from_secs(1), winner: "123".into(), loser: "567".into() } ] };
    content::JSON(serde_json::to_string(&ret).unwrap())
}

#[get("/<file..>", rank=2)]
pub fn files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("static/").join(file)).ok()
}

#[error(404)]
pub fn not_found() -> content::HTML<String> {
    content::HTML(r#"<img src="404.jpg"/>"#.into())
}
