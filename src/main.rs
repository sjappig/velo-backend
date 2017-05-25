#![feature(plugin)]
#![plugin(rocket_codegen)]

mod game_conversion;

#[macro_use]
extern crate log;
extern crate env_logger;

extern crate chrono;
extern crate rocket;

#[get("/players")]
fn players() -> String {
    "Hello world!".into()
}

fn main() {
    rocket::ignite().mount("/", routes![players]).launch();
}
