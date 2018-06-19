#![feature(plugin, custom_attribute)]
#![plugin(rocket_codegen)]

extern crate dotenv;
extern crate rocket;

#[macro_use]
extern crate diesel;
extern crate rocket_contrib;

#[macro_use]
extern crate serde_derive;

mod db;
mod models;
mod schema;

use db::DbConn;
use models::{NewPlayer, Player};
use rocket_contrib::Json;

use diesel::result::Error;
use rocket::http::Status;
use rocket::response::Failure;

#[get("/")]
fn index() -> &'static str {
    "Hello World"
}

#[get("/")]
fn get_players(conn: DbConn) -> Result<Json<Vec<Player>>, Failure> {
    Player::all_players(&conn)
        .map(|players| Json(players))
        .map_err(|error| error_status(error))
}

#[get("/<id>")]
fn get_player(id: i32, conn: DbConn) -> Result<Json<Player>, Failure> {
    Player::find_player(id, &conn)
        .map(|player| Json(player))
        .map_err(|error| error_status(error))
}

#[post("/", format = "application/json", data = "<new_player>")]
fn create_player(new_player: Json<NewPlayer>, conn: DbConn) -> Result<Json<Player>, Failure> {
    Player::insert_player(new_player.into_inner(), &conn)
        .map(|player| Json(player))
        .map_err(|error| error_status(error))
}

fn error_status(error: Error) -> Failure {
    Failure(match error {
        Error::NotFound => Status::NotFound,
        _ => Status::InternalServerError,
    })
}

fn main() {
    rocket::ignite()
        .manage(db::init_pool())
        .mount("/", routes![index])
        .mount("/player", routes![get_player, create_player])
        .mount("/players", routes![get_players])
        .launch();
}
