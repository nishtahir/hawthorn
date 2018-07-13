use api::auth::ApiToken;
use api::elo::Elo;
use api::error::ApiError;
use db::DbConn;
use models::game::{Game, NewGame};
use models::participant::{NewParticipant, Participant};

use models::{Insertable, Retrievable};
use rocket_contrib::Json;

#[derive(Serialize)]
struct GameResponse {
    pub id: i32,
    pub time_stamp: f64,
}

#[derive(Serialize)]
struct GameDetailResponse {
    pub id: i32,
    pub time_stamp: f64,
    pub participants: Vec<ParticipantResponse>,
}

#[derive(Serialize)]
struct ParticipantResponse {
    deck_id: i32,
    elo: f64,
}

#[derive(Deserialize)]
struct GameRequest {
    timestamp: Option<i32>,
    participants: Vec<ParticipantRequest>,
}

#[derive(Deserialize)]
struct ParticipantRequest {
    pub deck_id: i32,
    pub win: bool,
}

#[get("/")]
fn get_games(conn: DbConn, _token: ApiToken) -> Result<Json<Vec<GameResponse>>, ApiError> {
    let games = Game::all(&conn)?;
    let response = games
        .into_iter()
        .map(|game| GameResponse {
            id: game.id,
            time_stamp: game.time_stamp,
        })
        .collect();
    Ok(Json(response))
}

#[get("/<id>")]
fn get_game(id: i32, conn: DbConn, _token: ApiToken) -> Result<Json<GameDetailResponse>, ApiError> {
    let game = Game::find(id, &conn)?;
    let participants = Participant::find_by_game(&game, &conn)?;
    let participant_response = participants
        .into_iter()
        .map(|p| ParticipantResponse {
            deck_id: p.deck_id,
            elo: p.elo,
        })
        .collect();

    let response = GameDetailResponse {
        id: game.id,
        time_stamp: game.time_stamp,
        participants: participant_response,
    };

    Ok(Json(response))
}

#[post("/", format = "application/json", data = "<req>")]
fn create_game(
    req: Json<GameRequest>,
    conn: DbConn,
    _token: ApiToken,
) -> Result<Json<GameDetailResponse>, ApiError> {
    let game_request = req.into_inner();
    let new_game = NewGame::insert(NewGame::new(&game_request.timestamp), &conn)?;

    let new_participants = game_request
        .participants
        .into_iter()
        .map(|x| {
            let last_elo: f64 = Participant::find_latest_participant_by_deck_id(x.deck_id, &conn)
                .map(|p| p.elo)
                .unwrap_or(1000.0);

            NewParticipant {
                game_id: new_game.id,
                deck_id: x.deck_id,
                win: x.win,
                elo: last_elo,
            }
        })
        .collect();

    let updated = NewParticipant::compute_elo(&new_participants);
    let participants = NewParticipant::insert(&updated, &conn)?;

    let rankings = participants
        .into_iter()
        .map(|ranking| ParticipantResponse {
            deck_id: ranking.deck_id,
            elo: ranking.elo,
        })
        .collect();

    let response = GameDetailResponse {
        id: new_game.id,
        time_stamp: new_game.time_stamp,
        participants: rankings,
    };

    Ok(Json(response))
}
