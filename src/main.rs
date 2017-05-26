#![feature(ascii_ctype, custom_attribute, custom_derive, plugin)]
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
#[macro_use]
pub extern crate rocket_contrib;
#[macro_use]
extern crate serde_derive;
extern crate serde;
pub extern crate serde_json;

pub mod db;
mod game;
mod handlers;
mod player;

use std::env;
use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

fn main() {
    match db::get_connection_pool() {
        Ok(pool) => {
            let args: Vec<_> = env::args().collect();
            if args.len() > 1 {
                if args.len() != 3 {
                    println!("Usage: {} <gamefile> <playerfile>", args[0]);
                    return;
                }

                let conn = pool.get().unwrap();

                let playerfile = File::open(&args[2]).unwrap();
                let playerfilereader = BufReader::new(&playerfile);
                for line in playerfilereader.lines() {
                    db::insert_player(&conn,
                                      &player::Player::parse(line.unwrap().as_str()).unwrap())
                            .unwrap();
                }

                let gamefile = File::open(&args[1]).unwrap();
                let gamefilereader = BufReader::new(&gamefile);
                for line in gamefilereader.lines() {
                    db::insert_game(&conn, &game::Game::new(line.unwrap().as_str()).unwrap())
                        .unwrap();
                }
            }

            rocket::ignite()
                .manage(pool)
                .mount("/",
                       routes![handlers::root,
                               handlers::player::players,
                               handlers::player::player,
                               handlers::player::new_player,
                               handlers::game::games,
                               handlers::game::new_game,
                               handlers::files])
                .catch(errors![handlers::not_found])
                .launch();
        }
        Err(e) => {
            println!("Error initializing the database: {}", e);
        }
    }
}
