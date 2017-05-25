use rocket::response::{content, NamedFile};
use std::path::{Path, PathBuf};

#[get("/players")]
pub fn players() -> String {
    "Hello world!".into()
}

#[get("/<file..>")]
pub fn files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("static/").join(file)).ok()
}

#[error(404)]
pub fn not_found() -> content::HTML<String> {
    content::HTML(r#"<img src="404.jpg"/>"#.into())
}
