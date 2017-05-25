#![feature(plugin)]
#![plugin(rocket_codegen)]

mod game_conversion;

#[macro_use]
extern crate log;
extern crate env_logger;

extern crate chrono;
extern crate rocket;

use rocket::response::{content, NamedFile};
use std::path::{Path, PathBuf};

#[get("/players")]
fn players() -> String {
    "Hello world!".into()
}

#[get("/<file..>")]
fn files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("static/").join(file)).ok()
}

#[error(404)]
fn not_found() -> content::HTML<String> {
    content::HTML(r#"<img src="404.jpg"/>"#.into())
}

fn main() {
    rocket::ignite().mount("/", routes![players, files]).catch(errors![not_found]).launch();
}
