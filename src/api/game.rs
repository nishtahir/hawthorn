use api::auth::ApiToken;
use api::common::PaginatedResponse;
use api::elo::Elo;
use api::error::ApiError;
use db::DbConn;
use models::game::{Game, NewGame};
use models::participant::{NewParticipant, Participant};
use rocket_contrib::Json;

pub const DEFAULT_ELO: f64 = 1000.0;
pub const DEFAULT_LIMIT: i32 = 25;
pub const DEFAULT_OFFSET: i32 = 0;

#[derive(Deserialize)]
struct GameRequest {
    timestamp: Option<i32>,
    participants: Vec<ParticipantRequest>,
}

#[derive(Deserialize)]
struct EditGameRequest {
    id: i32,
    participants: Vec<ParticipantRequest>,
}

#[derive(FromForm, Debug)]
struct GameRequestParams {
    limit: Option<i32>,
    offset: Option<i32>,
}

#[derive(Serialize)]
struct GameResponse {
    pub id: i32,
    pub time_stamp: f64,
    pub participants: Vec<ParticipantResponse>,
}

#[derive(Deserialize)]
struct ParticipantRequest {
    pub deck_id: i32,
    pub win: bool,
}

#[derive(Serialize)]
struct ParticipantResponse {
    deck_id: i32,
    elo: f64,
    previous_elo: f64,
}

#[get("/?<params>")]
fn get_games_paginated(
    params: GameRequestParams,
    conn: DbConn,
    _token: ApiToken,
) -> Result<Json<PaginatedResponse<GameResponse>>, ApiError> {
    let limit = params.limit.unwrap_or(DEFAULT_LIMIT);
    let offset = params.offset.unwrap_or(DEFAULT_OFFSET);

    let mut response = vec![];
    for game in Game::all(limit, offset, &conn)? {
        let participants = Participant::find_by_game(&game, &conn)?;
        let participant_response = participants
            .into_iter()
            .map(|participant| {
                let previous_elo = participant
                    .find_previous(&conn)
                    .ok()
                    .map_or(DEFAULT_ELO, |p| p.elo);
                ParticipantResponse {
                    deck_id: participant.deck_id,
                    elo: participant.elo,
                    previous_elo: previous_elo,
                }
            })
            .collect();

        response.push(GameResponse {
            id: game.id,
            time_stamp: game.time_stamp,
            participants: participant_response,
        });
    }

    Ok(Json(PaginatedResponse {
        limit,
        offset,
        data: response,
    }))
}

#[get("/")]
fn get_games(
    conn: DbConn,
    _token: ApiToken,
) -> Result<Json<PaginatedResponse<GameResponse>>, ApiError> {
    let default_params = GameRequestParams {
        limit: None,
        offset: None,
    };
    get_games_paginated(default_params, conn, _token)
}

#[get("/<id>")]
fn get_game(id: i32, conn: DbConn, _token: ApiToken) -> Result<Json<GameResponse>, ApiError> {
    let game = Game::find_by_id(id, &conn)?;
    let participants = Participant::find_by_game(&game, &conn)?;
    let participant_response = participants
        .into_iter()
        .map(|participant| {
            let previous_elo = participant
                .find_previous(&conn)
                .ok()
                .map_or(DEFAULT_ELO, |p| p.elo);
            ParticipantResponse {
                deck_id: participant.deck_id,
                elo: participant.elo,
                previous_elo: previous_elo,
            }
        })
        .collect();

    let response = GameResponse {
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
) -> Result<Json<GameResponse>, ApiError> {
    let game_request = req.into_inner();
    let new_game = NewGame::insert(NewGame::new(&game_request.timestamp), &conn)?;

    let new_participants = game_request
        .participants
        .into_iter()
        .map(|x| {
            let current_elo: f64 = Participant::find_latest_by_deck_id(x.deck_id, &conn)
                .map(|p| p.elo)
                .unwrap_or(DEFAULT_ELO);

            NewParticipant {
                game_id: new_game.id,
                deck_id: x.deck_id,
                win: x.win,
                elo: current_elo,
            }
        })
        .collect();

    let updated = NewParticipant::compute_elo(&new_participants);
    let participants = NewParticipant::insert(&updated, &conn)?;

    let rankings = participants
        .into_iter()
        .map(|ranking| {
            let previous_elo = ranking
                .find_previous(&conn)
                .ok()
                .map_or(DEFAULT_ELO, |p| p.elo);
            ParticipantResponse {
                deck_id: ranking.deck_id,
                elo: ranking.elo,
                previous_elo: previous_elo,
            }
        })
        .collect();

    let response = GameResponse {
        id: new_game.id,
        time_stamp: new_game.time_stamp,
        participants: rankings,
    };

    Ok(Json(response))
}

#[delete("/<id>")]
fn delete_game(
    id: i32,
    conn: DbConn,
    _token: ApiToken,
) -> Result<Json<PaginatedResponse<GameResponse>>, ApiError> {
    let game = Game::find_by_id(id, &conn)?;
    let participants = Participant::find_by_game(&game, &conn)?;

    Participant::delete_all(participants, &conn);
    let _ = game.delete(&conn);
    let _ = refresh_elo_after(game, &conn)?;

    get_games(conn, _token)
}

#[put("/", format = "application/json", data = "<req>")]
fn update_game(
    req: Json<EditGameRequest>,
    conn: DbConn,
    _token: ApiToken,
) -> Result<Json<GameResponse>, ApiError> {
    let request = req.into_inner();
    let game = Game::find_by_id(request.id, &conn)?;

    let mut new_participants = vec![];
    for p in request.participants {
        let latest_elo_before_game = Participant::latest_by_deck_id_before_game(
            p.deck_id, &game, &conn,
        ).map(|p| p.elo)
            .unwrap_or(DEFAULT_ELO);

        let new_p = NewParticipant {
            game_id: game.id,
            deck_id: p.deck_id,
            win: p.win,
            elo: latest_elo_before_game,
        };
        new_participants.push(new_p)
    }

    let participants = Participant::find_by_game(&game, &conn)?;
    Participant::delete_all(participants, &conn);
    let _ = NewParticipant::insert(&new_participants, &conn);

    let previous_game = game.find_previous(&conn).unwrap_or(game);
    let _ = refresh_elo_after(previous_game, &conn)?;

    get_game(request.id, conn, _token)
}

fn refresh_elo_after(game: Game, conn: &DbConn) -> Result<(), ApiError> {
    let next_games = game.all_after(&conn)?;
    for g in next_games {
        let parts_with_previous_elo = Participant::find_by_game(&g, &conn)?
            .into_iter()
            .map(|_p| {
                let previous_elo = _p
                    .find_previous(&conn)
                    .map(|it| it.elo)
                    .unwrap_or(DEFAULT_ELO);

                Participant {
                    id: _p.id,
                    game_id: _p.game_id,
                    deck_id: _p.deck_id,
                    win: _p.win,
                    elo: previous_elo,
                }
            })
            .collect();

        let updated = Participant::compute_elo(&parts_with_previous_elo);
        Participant::update_all(&updated, &conn);
    }
    Ok({})
}
