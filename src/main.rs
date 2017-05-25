#![feature(custom_attribute, custom_derive, plugin)]
#![plugin(rocket_codegen)]
#![allow(unmounted_route)]

#[macro_use]
extern crate lazy_static;

extern crate chrono;
extern crate postgres;
extern crate r2d2;
extern crate r2d2_postgres;
extern crate regex;
extern crate rocket;
extern crate rocket_contrib;
#[macro_use]
extern crate serde_derive;
extern crate serde;
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
                       routes![handlers::root,
                               handlers::players,
                               handlers::player,
                               handlers::new_player,
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
