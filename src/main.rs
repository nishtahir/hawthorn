#![feature(plugin, custom_attribute)]
#![plugin(rocket_codegen)]
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
        .mount("/login", routes![api::login::login])
        .mount(
            "/player",
            routes![api::player::get_player, api::player::create_player],
        )
        .mount("/players", routes![api::player::get_players])
        .mount(
            "/deck",
            routes![api::deck::get_deck, api::deck::create_deck],
        )
        .mount("/decks", routes![api::deck::get_decks])
        .mount("/games", routes![api::game::get_games])
        .mount(
            "/game",
            routes![api::game::get_game, api::game::create_game],
        )
        .launch();
}
