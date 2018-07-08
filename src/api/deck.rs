use api::auth::ApiToken;
use bcrypt::hash;
use db::DbConn;
use models::deck::Deck;
use models::deck::{Deck, NewDeck};
use models::{Insertable, Retrievable};
use rocket::http::Status;
use rocket::response::Failure;
use rocket_contrib::Json;

#[derive(Serialize)]
struct DeckResponse {
    id: String,
    alias: String,
    commander: String,
    games: i32,
    wins: i32,
    elo: f64,
}

#[get("/")]
fn get_decks(conn: DbConn, _token: ApiToken) -> Result<Json<Vec<Deck>>, Failure> {
    Deck::all(&conn)
        .map(|deck| Json(deck))
        .map_err(|error| error_status(error))
}

#[get("/<id>")]
fn get_deck(id: i32, conn: DbConn, _token: ApiToken) -> Result<Json, Failure> {
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
fn create_deck(
    new_deck: Json<NewDeck>,
    conn: DbConn,
    _token: ApiToken,
) -> Result<Json<Deck>, Failure> {
    NewDeck::insert(new_deck.into_inner(), &conn)
        .map(|deck| Json(deck))
        .map_err(|error| error_status(error))
}
