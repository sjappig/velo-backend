use rocket::response::{content, NamedFile};
use std::path::{Path, PathBuf};

pub mod game;
pub mod player;

/// Use handlers::prelude::* to bring commonly used
/// handler stuff to a module
mod prelude {
    pub use db;
    pub use rocket::State;
    pub use rocket::http::Status;
    pub use rocket::response::content;
    pub use rocket::response::status;
    pub use rocket_contrib::Json;
    pub use rocket_contrib;
    pub use serde_json;
    pub use std::error::Error;
    pub use super::HandlerResult;
}

use self::prelude::*;

/// Result type that contains JSON values for both success and failure
pub type HandlerResult = Result<Json<serde_json::Value>, status::Custom<Json<serde_json::Value>>>;

/// Helper function to convert types implementing `std::error::Error` trait to JSON
fn error_to_json<E: Error>(e: E) -> status::Custom<Json<serde_json::Value>> {
    status::Custom(
        Status::InternalServerError,
        Json(json!({
        "success": false,
        "error": e.description()
    })),
    )
}

#[get("/")]
pub fn root() -> Option<NamedFile> {
    NamedFile::open(Path::new("static/index.html")).ok()
}

#[get("/<file..>", rank = 2)]
pub fn files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("static/").join(file)).ok()
}

#[error(404)]
pub fn not_found() -> content::Html<String> {
    content::Html(r#"<img src="404.jpg"/>"#.into())
}
