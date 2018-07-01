#![feature(plugin, custom_attribute)]
#![plugin(rocket_codegen)]

extern crate dotenv;
extern crate rocket;
extern crate time;

#[macro_use]
extern crate diesel;

#[macro_use]
extern crate rocket_contrib;

#[macro_use]
extern crate serde_derive;

mod db;
mod elo;
mod models;
mod schema;

use db::DbConn;

use models::{
    Deck, Game, Insertable, NewDeck, NewGame, NewParticipant, NewPlayer, NewRanking, Participant,
    ParticipantRequest, Player, Ranking, Retrievable,
};

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
    Player::all(&conn)
        .map(|players| Json(players))
        .map_err(|error| error_status(error))
}

#[get("/<id>")]
fn get_player(id: i32, conn: DbConn) -> Result<Json, Failure> {
    match Player::find(id, &conn) {
        Ok(player) => match Deck::find_by_player(&player, &conn) {
            Ok(decks) => Ok(Json(
                json!({ "id": player.id, "alias": player.alias, "decks": decks }),
            )),
            Err(error) => Err(error_status(error)),
        },
        Err(error) => Err(error_status(error)),
    }
}

#[post("/", format = "application/json", data = "<new_player>")]
fn create_player(new_player: Json<NewPlayer>, conn: DbConn) -> Result<Json<Player>, Failure> {
    NewPlayer::insert(new_player.into_inner(), &conn)
        .map(|player| Json(player))
        .map_err(|error| error_status(error))
}

#[get("/")]
fn get_decks(conn: DbConn) -> Result<Json<Vec<Deck>>, Failure> {
    Deck::all(&conn)
        .map(|deck| Json(deck))
        .map_err(|error| error_status(error))
}

#[get("/<id>")]
fn get_deck(id: i32, conn: DbConn) -> Result<Json, Failure> {
    match Deck::find(id, &conn) {
        Ok(deck) => match Participant::find_by_deck(&deck, &conn) {
            Ok(participants) => Ok(Json(json!({
                        "id": deck.id,
                        "alias": deck.alias,
                        "commander": deck.commander,
                        "player_id": deck.player_id,
                        "games": participants.len(),
                        "wins": participants.iter().filter(|&p| p.win == true).count()
                    }))),
            Err(error) => Err(error_status(error)),
        },
        Err(error) => Err(error_status(error)),
    }
}

#[post("/", format = "application/json", data = "<new_deck>")]
fn create_deck(new_deck: Json<NewDeck>, conn: DbConn) -> Result<Json<Deck>, Failure> {
    NewDeck::insert(new_deck.into_inner(), &conn)
        .map(|deck| Json(deck))
        .map_err(|error| error_status(error))
}

#[get("/")]
fn get_games(conn: DbConn) -> Result<Json<Vec<Game>>, Failure> {
    Game::all(&conn)
        .map(|game| Json(game))
        .map_err(|error| error_status(error))
}

#[get("/<id>")]
fn get_game(id: i32, conn: DbConn) -> Result<Json, Failure> {
    match Game::find(id, &conn) {
        Ok(game) => match Participant::find_by_game(&game, &conn) {
            Ok(participant) => Ok(Json(
                json!({ "id": game.id, "time_stamp": game.time_stamp ,"participants": participant }),
            )),
            Err(error) => Err(error_status(error)),
        },
        Err(error) => Err(error_status(error)),
    }
}

#[post("/", format = "application/json", data = "<data>")]
fn create_game(data: Json<Vec<ParticipantRequest>>, conn: DbConn) -> Result<Json, Failure> {
    NewGame::insert(NewGame::new(), &conn)
        .and_then(|game| {
            let participant_list = data
                .into_inner()
                .iter()
                .map(|p| NewParticipant::new(game.id, p.deck_id, p.win))
                .collect::<Vec<NewParticipant>>();

            NewParticipant::insert(&participant_list, &conn)
                .and_then(|list| create_rankings(&list, &conn))
                .and_then(|rankings| Ok(Json(json!({ "game": game, "participants": rankings }))))
        })
        .map_err(|error| error_status(error))
}

fn create_rankings(data: &Vec<Participant>, conn: &DbConn) -> Result<Vec<Ranking>, Error> {
    let result = data
        .into_iter()
        .map(|p| Ranking::from_participant(&p, conn).map(|r| r.to_rankable_entity(p.win)))
        .collect();

    match result {
        Ok(rankings) => {
            let final_rankings = elo::compute_elo(&rankings)
                .iter()
                .map(|e| NewRanking::from_rankable_entity(e))
                .collect();

            NewRanking::insert_all(&final_rankings, &conn)
        }
        Err(error) => Err(error),
    }
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
        .mount("/deck", routes![get_deck, create_deck])
        .mount("/decks", routes![get_decks])
        .mount("/games", routes![get_games])
        .mount("/game", routes![get_game, create_game])
        .launch();
}
