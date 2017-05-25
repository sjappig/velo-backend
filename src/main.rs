#![feature(plugin)]
#![plugin(rocket_codegen)]


#[macro_use]
extern crate log;
extern crate env_logger;

extern crate chrono;
extern crate diesel;
extern crate r2d2;
extern crate r2d2_diesel;
extern crate rocket;

mod db;
mod game_conversion;
mod handlers;

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
