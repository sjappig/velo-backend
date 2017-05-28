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
    pub use rocket_contrib::JSON;
    pub use rocket_contrib;
    pub use serde_json;
    pub use std::error::Error;
    pub use super::HandlerResult;
}

use self::prelude::*;

/// Result type that contains JSON values for both success and failure
pub type HandlerResult = Result<JSON<serde_json::Value>, JSON<serde_json::Value>>;

/// Helper function to convert types implementing `std::error::Error` trait to JSON
fn error_to_json<E: Error>(e: E) -> JSON<serde_json::Value> {
    JSON(json!({
        "success": false,
        "error": e.description()
    }))
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
