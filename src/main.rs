#![feature(plugin, custom_attribute, custom_derive)]
#![plugin(rocket_codegen)]
#![allow(proc_macro_derive_resolution_fallback)] // This can be removed after diesel-1.4

extern crate bcrypt;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;
extern crate dotenv;
extern crate jsonwebtoken;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;
extern crate log4rs;
extern crate rand;
extern crate rocket;
extern crate rocket_contrib;
extern crate rocket_cors;
#[macro_use]
extern crate serde_derive;
extern crate time;

mod api;
mod db;
mod models;
mod schema;

use api::auth::*;
use api::deck::*;
use api::error::*;
use api::game::*;
use api::player::*;
use db::SqlitePool;
use dotenv::dotenv;
use rocket::http::Method;
use std::env;

embed_migrations!("./migrations/");

fn main() {
    dotenv().ok();
    let log_config = env::var("LOG_CONFIG_PATH").expect("LOG_CONFIG_PATH must be set");
    let _ = log4rs::init_file(log_config, Default::default()).unwrap();
    info!("Hawthorn is starting up...");

    let pool = db::init_pool();
    match pool.get() {
        Ok(connection) => {
            info!("Running database migrations...");
            let _ = embedded_migrations::run_with_output(&connection, &mut std::io::stdout());
            setup_routes(pool);
        }
        Err(_) => {
            error!("Failed to aquire database connection");
        }
    }
}

fn setup_routes(pool: SqlitePool) {
    // You can also deserialize this
    let options = rocket_cors::Cors {
        allowed_methods: vec![Method::Get, Method::Post, Method::Put, Method::Delete]
            .into_iter()
            .map(From::from)
            .collect(),
        allow_credentials: true,
        ..Default::default()
    };

    rocket::ignite()
        .manage(pool)
        .mount("/", routes![api::index::index])
        .mount("/auth", routes![login, change_password, refresh_token])
        .mount(
            "/players",
            routes![get_players, get_player, create_player, update_player],
        )
        .mount(
            "/decks",
            routes![
                get_decks,
                get_deck,
                create_deck,
                update_deck,
                get_leaderboard,
                get_pods
            ],
        )
        .mount(
            "/games",
            routes![
                get_games_paginated,
                get_games,
                get_game,
                create_game,
                delete_game,
                update_game
            ],
        )
        .catch(catchers![handle_404, handle_401, handle_400,])
        .attach(options)
        .launch();
}
