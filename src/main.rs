#![feature(plugin)]
#![plugin(rocket_codegen)]

mod game_conversion;

#[macro_use]
extern crate log;
extern crate env_logger;

extern crate chrono;
extern crate rocket;

mod handlers;

fn main() {
    rocket::ignite()
        .mount("/", routes![handlers::players, handlers::files])
        .catch(errors![handlers::not_found])
        .launch();
}
