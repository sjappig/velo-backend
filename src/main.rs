#![feature(plugin)]
#![plugin(rocket_codegen)]

#[macro_use] extern crate lazy_static;

extern crate chrono;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_codegen;
extern crate r2d2;
extern crate r2d2_diesel;
extern crate regex;
extern crate rocket;

mod db;
mod game;
mod handlers;
mod player;

fn main() {
    match db::get_connection_pool() {
        Ok(pool) => {
            rocket::ignite()
                .manage(pool)
                .mount("/", routes![handlers::players, handlers::files])
                .catch(errors![handlers::not_found])
                .launch();
        }
        Err(e) => {
            println!("Error initializing the database: {}", e);
        }
    }
}
