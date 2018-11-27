use api::auth::ApiToken;
use api::error::ApiError;
use api::game::DEFAULT_ELO;
use db::DbConn;
use models::deck::{Deck, NewDeck};
use models::participant::Participant;
use rocket_contrib::json::Json;
use std::cmp::Ordering;
use time;

#[derive(Deserialize)]
pub struct DeckRequest {
    player_id: i32,
    alias: String,
    commander: String,
}

#[derive(Deserialize)]
pub struct UpdateDeckRequest {
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
    elo_delta: f64,
}

impl Deck {
    fn update_from(self, req: UpdateDeckRequest) -> Deck {
        Deck {
            id: self.id,
            alias: req.alias.unwrap_or(self.alias),
            commander: req.commander.unwrap_or(self.commander),
            player_id: self.player_id,
            active: req.active.unwrap_or(self.active),
        }
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

        let current_elo = participations.first().map_or(DEFAULT_ELO, |p| p.elo);
        let previous_elo = participations.get(1).map_or(0.0, |p| p.elo);

        DeckResponse {
            id: deck.id,
            alias: deck.alias,
            player_id: deck.player_id,
            commander: deck.commander,
            active: deck.active,
            games: games,
            wins: wins,
            win_percentage: win_percentage,
            elo: current_elo,
            elo_delta: current_elo - previous_elo,
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
pub fn get_decks(conn: DbConn, _token: ApiToken) -> Result<Json<Vec<DeckResponse>>, ApiError> {
    let decks = Deck::all(&conn)?;
    let participants = Participant::all_grouped_by_deck(decks, &conn)?;

    let response = participants
        .into_iter()
        .map(|(deck, participations)| DeckResponse::new(deck, participations))
        .collect();
    Ok(Json(response))
}

#[get("/<id>")]
pub fn get_deck(id: i32, conn: DbConn, _token: ApiToken) -> Result<Json<DeckResponse>, ApiError> {
    let deck = Deck::find_by_id(id, &conn)?;
    let participations = Participant::find_by_deck(&deck, &conn)?;
    let response = DeckResponse::new(deck, participations);
    Ok(Json(response))
}

#[get("/leaderboard")]
pub fn get_leaderboard(
    conn: DbConn,
    _token: ApiToken,
) -> Result<Json<Vec<DeckResponse>>, ApiError> {
    let decks = Deck::all(&conn)?;
    let query_result = Participant::all_by_deck_join_game(decks, &conn)?;
    let time_four_weeks_ago = current_time() - 2419200.0; /*four weeks*/
    let mut leaderboard = query_result
        .into_iter()
        .filter(|(deck, participations)| {
            let max_time = participations
                .iter()
                .map(|(_, g)| g.time_stamp)
                .fold(0.0, f64::max);
            deck.active && participations.len() >= 5 && max_time > time_four_weeks_ago
        })
        .map(|(deck, participations)| {
            let (parts, _): (Vec<_>, Vec<_>) = participations.into_iter().unzip();
            DeckResponse::new(deck, parts)
        })
        .collect::<Vec<DeckResponse>>();

    leaderboard.sort_by(|a, b| b.elo.partial_cmp(&a.elo).unwrap_or(Ordering::Less));

    let response = leaderboard
        .into_iter()
        .take(20)
        .collect::<Vec<DeckResponse>>();

    Ok(Json(response))
}

#[post("/", format = "application/json", data = "<req>")]
pub fn create_deck(
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
        elo_delta: 0.0,
    };

    Ok(Json(response))
}

#[put("/", format = "application/json", data = "<json>")]
pub fn update_deck(
    json: Json<UpdateDeckRequest>,
    conn: DbConn,
    _token: ApiToken,
) -> Result<Json<DeckResponse>, ApiError> {
    let req = json.into_inner();
    let current_deck = Deck::find_by_id(req.id, &conn)?;
    let updated_deck = Deck::update(current_deck.update_from(req), &conn)?;

    let participations = Participant::find_by_deck(&updated_deck, &conn)?;

    let response = DeckResponse::new(updated_deck, participations);
    Ok(Json(response))
}

fn current_time() -> f64 {
    let timespec = time::get_time();
    timespec.sec as f64
}
