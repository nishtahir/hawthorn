use api::auth::ApiToken;
use api::error::ApiError;
use db::DbConn;
use diesel::result::Error;
use elo;
use models::game::{Game, NewGame};
use models::participant::{NewParticipant, Participant};

use models::ranking::{NewRanking, Ranking};
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
            elo: -1.0, // we need to associate games or participants with rankings
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
    let new_game = NewGame::insert(NewGame::new(), &conn)?;

    let new_participants = game_request
        .participants
        .into_iter()
        .map(|p| NewParticipant::new(new_game.id, p.deck_id, p.win))
        .collect::<Vec<NewParticipant>>();

    let rankings = NewParticipant::insert(&new_participants, &conn)
        .and_then(|list| create_rankings(&list, &conn))?
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
