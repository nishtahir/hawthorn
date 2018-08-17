use api::auth::ApiToken;
use api::error::ApiError;
use api::game::DEFAULT_ELO;
use db::DbConn;
use models::deck::{Deck, NewDeck};
use models::participant::Participant;
use models::{Insertable, Retrievable};
use rocket::request;
use rocket::request::FromFormValue;
use rocket_contrib::Json;
use std::cmp::Ordering;
use time;

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
pub struct DeckResponse {
    id: i32,
    alias: String,
    commander: String,
    player_id: i32,
    active: bool,
    games: i32,
    wins: i32,
    win_percentage: f64,
    elo: f64,
}

#[derive(Deserialize, Debug)]
struct PodReqestParam {
    players: Vec<i32>,
}

///
/// We need this in order to have a variable number of player_id parameters
/// in the request. This looks through the request parameters and populates
/// a Vec with matching keys
///
impl<'f> request::FromForm<'f> for PodReqestParam {
    type Error = ApiError;

    fn from_form(form_items: &mut request::FormItems<'f>, _: bool) -> Result<Self, Self::Error> {
        let mut req = PodReqestParam { players: vec![] };
        for (k, v) in form_items {
            let key: &str = &*k;
            let value = i32::from_form_value(v).map_err(|_| ApiError::BadRequest)?;
            match key {
                "player_id" => req.players.push(value),
                _ => return Err(ApiError::BadRequest),
            }
        }
        Ok(req)
    }
}

impl DeckResponse {
    pub fn new(deck: Deck, participations: Vec<Participant>) -> DeckResponse {
        let games = participations.len() as i32;
        let wins = participations.iter().filter(|&p| p.win == true).count() as i32;
        let win_percentage = if games > 0 {
            wins as f64 / games as f64
        } else {
            0.0
        };

        DeckResponse {
            id: deck.id,
            alias: deck.alias,
            player_id: deck.player_id,
            commander: deck.commander,
            active: deck.active,
            games: games,
            wins: wins,
            win_percentage: win_percentage,
            elo: participations.first().map_or(DEFAULT_ELO, |p| p.elo),
        }
    }

    pub fn into_deck_response(
        decks: Vec<Deck>,
        conn: &DbConn,
    ) -> Result<Vec<DeckResponse>, ApiError> {
        let participants = Participant::all_grouped_by_deck(decks, &conn)?;
        Ok(participants
            .into_iter()
            .map(|(deck, participations)| DeckResponse::new(deck, participations))
            .collect())
    }
}

#[get("/")]
fn get_decks(conn: DbConn, _token: ApiToken) -> Result<Json<Vec<DeckResponse>>, ApiError> {
    let decks = Deck::all(&conn)?;
    let participants = Participant::all_grouped_by_deck(decks, &conn)?;

    let response = participants
        .into_iter()
        .map(|(deck, participations)| DeckResponse::new(deck, participations))
        .collect();
    Ok(Json(response))
}

#[get("/<id>")]
fn get_deck(id: i32, conn: DbConn, _token: ApiToken) -> Result<Json<DeckResponse>, ApiError> {
    let deck = Deck::find(id, &conn)?;
    let participations = Participant::find_by_deck(&deck, &conn)?;
    let response = DeckResponse::new(deck, participations);
    Ok(Json(response))
}

#[get("/leaderboard")]
fn get_leaderboard(conn: DbConn, _token: ApiToken) -> Result<Json<Vec<DeckResponse>>, ApiError> {
    let decks = Deck::all(&conn)?;
    let query_result = Participant::all_by_deck_join_game(decks, &conn)?;
    let time_four_weeks_ago = current_time() - 2419200.0; /*four weeks*/
    let mut response = query_result
        .into_iter()
        .filter(|(_, participations)| {
            let max_time = participations
                .iter()
                .map(|(_, g)| g.time_stamp)
                .fold(0.0, f64::max);
            participations.len() > 5 && max_time > time_four_weeks_ago
        })
        .map(|(deck, participations)| {
            let (parts, _): (Vec<_>, Vec<_>) = participations.into_iter().unzip();
            DeckResponse::new(deck, parts)
        })
        .take(20)
        .collect::<Vec<DeckResponse>>();

    response.sort_by(|a, b| b.elo.partial_cmp(&a.elo).unwrap_or(Ordering::Less));
    Ok(Json(response))
}

#[get("/pods?<params>")]
fn get_pods(
    params: PodReqestParam,
    conn: DbConn,
    _token: ApiToken,
) -> Result<Json<Vec<DeckResponse>>, ApiError> {
    let mut response = vec![];
    for id in params.players {
        let deck = Deck::find(id, &conn)?;
        let participations = Participant::find_by_deck(&deck, &conn)?;
        response.push(DeckResponse::new(deck, participations))
    }
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
        player_id: deck.player_id,
        active: deck.active,
        games: 0,
        wins: 0,
        win_percentage: 0.0,
        elo: DEFAULT_ELO,
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

    let current_deck = Deck::find(update_deck_request.id, &conn)?;
    let new_deck = Deck {
        id: current_deck.id,
        alias: update_deck_request.alias.unwrap_or(current_deck.alias),
        commander: update_deck_request
            .commander
            .unwrap_or(current_deck.commander),
        player_id: current_deck.player_id,
        active: update_deck_request.active.unwrap_or(current_deck.active),
    };

    let deck = Deck::update(new_deck, &conn)?;
    let participations = Participant::find_by_deck(&deck, &conn)?;
    let response = DeckResponse::new(deck, participations);

    Ok(Json(response))
}

fn current_time() -> f64 {
    let timespec = time::get_time();
    timespec.sec as f64
}
