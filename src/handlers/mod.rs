use rocket::response::{content, NamedFile};
use std::path::{Path, PathBuf};

pub mod game;
pub mod player;

/// Use handlers::prelude::* to bring commonly used
/// handler stuff to a module
mod prelude {
    pub use db;
    pub use rocket::State;
    pub use rocket::response::content;
    pub use rocket_contrib;
    pub use rocket_contrib::JSON;
    pub use serde_json;
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
