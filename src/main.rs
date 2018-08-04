#![feature(plugin, custom_attribute, custom_derive)]
#![plugin(rocket_codegen)]
#![allow(proc_macro_derive_resolution_fallback)] // This can be removed after diesel-1.4

extern crate bcrypt;
extern crate dotenv;
extern crate jsonwebtoken;
extern crate rocket;
extern crate rocket_contrib;
extern crate time;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate serde_derive;
extern crate rand;

mod api;
mod db;
mod models;
mod schema;

fn main() {
    rocket::ignite()
        .manage(db::init_pool())
        .mount("/", routes![api::index::index])
        .mount("/auth", routes![api::auth::login])
        .mount(
            "/players",
            routes![
                api::player::get_players,
                api::player::get_player,
                api::player::create_player,
                api::player::update_player
            ],
        )
        .mount(
            "/decks",
            routes![
                api::deck::get_decks,
                api::deck::get_deck,
                api::deck::create_deck,
                api::deck::update_deck,
                api::deck::get_leaderboard,
                api::deck::get_pods
            ],
        )
        .mount(
            "/games",
            routes![
                api::game::get_games,
                api::game::get_game,
                api::game::create_game,
                api::game::delete_game
            ],
        )
        .launch();
}
