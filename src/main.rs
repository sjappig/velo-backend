#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;

#[get("/players")]
fn players() -> String {
    "Hello world!".into()
}

fn main() {
    rocket::ignite().mount("/", routes![players]).launch();
}
