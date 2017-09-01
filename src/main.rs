#![feature(ascii_ctype, custom_attribute, custom_derive, plugin)]
#![plugin(rocket_codegen)]

#[macro_use]
extern crate lazy_static;

extern crate chrono;
#[macro_use]
extern crate derive_error_chain;
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

mod error;
mod game;
mod handlers;
mod id;
mod player;
pub mod db;

use std::env;
use std::error::Error;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::io;

fn insert_players<R>(
    conn: &postgres::Connection,
    reader: std::io::BufReader<R>,
) -> Result<(), Box<Error>>
where
    R: io::Read,
{
    for line in reader.lines() {
        let player = player::Player::parse(line?.as_str())?;
        db::insert_player(conn, &player)?;
    }
    Ok(())
}

fn insert_games<R>(
    conn: &postgres::Connection,
    reader: std::io::BufReader<R>,
) -> Result<(), Box<Error>>
where
    R: io::Read,
{
    for line in reader.lines() {
        let game = game::Game::new(line?.as_str())?;
        db::insert_game(conn, &game)?;
    }
    Ok(())
}

fn populate_database(conn: &postgres::Connection, args: &[String]) -> Result<(), Box<Error>> {
    let playerfile = File::open(&args[2])?;
    insert_players(&conn, BufReader::new(playerfile))?;

    let gamefile = File::open(&args[1])?;
    insert_games(&conn, BufReader::new(gamefile))
}

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
                if let Err(e) = populate_database(&conn, &args[..]) {
                    println!("Could not populate database: {}", e.description());
                    std::process::exit(0);
                }
            }

            rocket::ignite()
                .manage(pool)
                .mount(
                    "/",
                    routes![
                        handlers::root,
                        handlers::player::players,
                        handlers::player::player,
                        handlers::player::new_player,
                        handlers::game::games,
                        handlers::game::new_game,
                        handlers::files,
                    ],
                )
                .catch(errors![handlers::not_found])
                .launch();
        }
        Err(e) => {
            println!("Error initializing the database: {}", e);
        }
    }
}
