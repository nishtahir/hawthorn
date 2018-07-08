use api::auth::ApiToken;
use api::error::ApiError;
use db::DbConn;
use models::deck::{Deck, NewDeck};
use models::participant::Participant;
use models::{Insertable, Retrievable};
use rocket_contrib::Json;

#[derive(Deserialize)]
struct DeckRequest {
    player_id: i32,
    alias: String,
    commander: String,
}

#[derive(Serialize)]
struct DeckResponse {
    id: i32,
    alias: String,
    commander: String,
}

#[derive(Serialize)]
struct DeckDetailResponse {
    id: i32,
    alias: String,
    commander: String,
    games: i32,
    wins: i32,
    elo: f64,
}

#[get("/")]
fn get_decks(conn: DbConn, _token: ApiToken) -> Result<Json<Vec<DeckResponse>>, ApiError> {
    let decks = Deck::all(&conn)?;
    let response = decks
        .into_iter()
        .map(|deck| DeckResponse {
            id: deck.id,
            alias: deck.alias,
            commander: deck.commander,
        })
        .collect();
    Ok(Json(response))
}

#[get("/<id>")]
fn get_deck(id: i32, conn: DbConn, _token: ApiToken) -> Result<Json<DeckDetailResponse>, ApiError> {
    let deck = Deck::find(id, &conn)?;
    let participations = Participant::find_by_deck(&deck, &conn)?;
    println!("{:?}", participations);

    let response = DeckDetailResponse {
        id: deck.id,
        alias: deck.alias,
        commander: deck.commander,
        games: participations.len() as i32,
        wins: participations.iter().filter(|&p| p.win == true).count() as i32,
        elo: participations.first().map_or(900.0, |p| p.elo),
    };

    Ok(Json(response))
}

#[post("/", format = "application/json", data = "<req>")]
fn create_deck(
    req: Json<DeckRequest>,
    conn: DbConn,
    _token: ApiToken,
) -> Result<Json<DeckDetailResponse>, ApiError> {
    let deck_request = req.into_inner();

    let new_deck = NewDeck {
        alias: deck_request.alias,
        commander: deck_request.commander,
        player_id: deck_request.player_id,
    };

    let deck = NewDeck::insert(new_deck, &conn)?;
    let response = DeckDetailResponse {
        id: deck.id,
        alias: deck.alias,
        commander: deck.commander,
        games: 0,
        wins: 0,
        elo: 1000.0,
    };

    Ok(Json(response))
}
