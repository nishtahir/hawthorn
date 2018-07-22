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

#[derive(Deserialize)]
struct UpdateDeckRequest {
    id: i32,
    alias: Option<String>,
    commander: Option<String>,
    active: Option<bool>,
}

#[derive(Serialize)]
struct DeckResponse {
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
        .map(|deck| {
            let participations = Participant::find_by_deck(&deck, &conn);

            let (games, wins, elo) = participations
                .map(|p_list| {
                    let games = p_list.len() as i32;
                    let wins = p_list.iter().filter(|&p| p.win == true).count() as i32;
                    let elo = p_list.first().map_or(1000.0, |p| p.elo);
                    (games, wins, elo)
                })
                .unwrap_or((0, 0, 0.0));

            DeckResponse {
                id: deck.id,
                alias: deck.alias,
                commander: deck.commander,
                games: games,
                wins: wins,
                elo: elo,
            }
        })
        .collect();
    Ok(Json(response))
}

#[get("/<id>")]
fn get_deck(id: i32, conn: DbConn, _token: ApiToken) -> Result<Json<DeckResponse>, ApiError> {
    let deck = Deck::find(id, &conn)?;
    let participations = Participant::find_by_deck(&deck, &conn)?;

    let response = DeckResponse {
        id: deck.id,
        alias: deck.alias,
        commander: deck.commander,
        games: participations.len() as i32,
        wins: participations.iter().filter(|&p| p.win == true).count() as i32,
        elo: participations.first().map_or(1000.0, |p| p.elo),
    };

    Ok(Json(response))
}

#[post("/", format = "application/json", data = "<req>")]
fn create_deck(
    req: Json<DeckRequest>,
    conn: DbConn,
    _token: ApiToken,
) -> Result<Json<DeckResponse>, ApiError> {
    let deck_request = req.into_inner();

    let new_deck = NewDeck {
        alias: deck_request.alias,
        commander: deck_request.commander,
        player_id: deck_request.player_id,
        active: true,
    };

    let deck = NewDeck::insert(new_deck, &conn)?;
    let response = DeckResponse {
        id: deck.id,
        alias: deck.alias,
        commander: deck.commander,
        games: 0,
        wins: 0,
        elo: 1000.0,
    };

    Ok(Json(response))
}

#[put("/", format = "application/json", data = "<req>")]
fn update_deck(
    req: Json<UpdateDeckRequest>,
    conn: DbConn,
    _token: ApiToken,
) -> Result<Json<DeckResponse>, ApiError> {
    let update_deck_request = req.into_inner();

    let _deck = Deck::find(update_deck_request.id, &conn)?;
    let new_deck = Deck {
        id: _deck.id,
        alias: update_deck_request.alias.unwrap_or(_deck.alias),
        commander: update_deck_request.commander.unwrap_or(_deck.commander),
        player_id: _deck.player_id,
        active: update_deck_request.active.unwrap_or(_deck.active),
    };

    let deck = Deck::update(new_deck, &conn)?;
    let participations = Participant::find_by_deck(&deck, &conn)?;

    let response = DeckResponse {
        id: deck.id,
        alias: deck.alias,
        commander: deck.commander,
        games: participations.len() as i32,
        wins: participations.iter().filter(|&p| p.win == true).count() as i32,
        elo: participations.first().map_or(1000.0, |p| p.elo),
    };

    Ok(Json(response))
}
