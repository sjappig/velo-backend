#![feature(custom_attribute, plugin)]
#![plugin(rocket_codegen)]

#[macro_use]
extern crate lazy_static;

extern crate chrono;
extern crate postgres;
extern crate r2d2;
extern crate r2d2_postgres;
extern crate regex;
extern crate rocket;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

mod db;
mod game;
mod handlers;
mod player;

fn main() {
    match db::get_connection_pool() {
        Ok(pool) => {
            rocket::ignite()
                .manage(pool)
                .mount("/",
                       routes![handlers::players,
                               handlers::player,
                               handlers::games,
                               handlers::files])
                .catch(errors![handlers::not_found])
                .launch();
        }
        Err(e) => {
            println!("Error initializing the database: {}", e);
        }
    }
}
