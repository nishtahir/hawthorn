use api::auth::ApiToken;
use api::deck::DeckResponse;
use api::error::ApiError;
use bcrypt::hash;
use db::DbConn;
use models::deck::Deck;
use models::player::{NewPlayer, Player};
use models::{Insertable, Retrievable};
use rocket_contrib::Json;
#[derive(Deserialize)]
pub struct CreatePlayerRequest {
    alias: String,
    email: String,
    password: String,
}

#[derive(Deserialize)]
pub struct UpdatePlayerRequest {
    id: i32,
    alias: Option<String>,
    email: Option<String>,
}

#[derive(Serialize)]
pub struct PlayerResponse {
    id: i32,
    alias: String,
}

#[derive(Serialize)]
pub struct PlayerDetailResponse {
    id: i32,
    alias: String,
    decks: Vec<DeckResponse>,
}

#[post("/", format = "application/json", data = "<req>")]
pub fn create_player(
    req: Json<CreatePlayerRequest>,
    conn: DbConn,
) -> Result<Json<PlayerResponse>, ApiError> {
    let create_player_req = req.into_inner();

    let hash = hash(&create_player_req.password, /*cost*/ 8)?;

    let new_player = NewPlayer {
        alias: create_player_req.alias,
        email: create_player_req.email,
        password: hash,
    };

    let response = NewPlayer::insert(new_player, &conn).map(|player| {
        Json(PlayerResponse {
            id: player.id,
            alias: player.alias,
        })
    })?;

    Ok(response)
}

#[get("/")]
fn get_players(
    conn: DbConn,
    _token: ApiToken,
) -> Result<Json<Vec<PlayerDetailResponse>>, ApiError> {
    let players = Player::all(&conn)?;
    let mut response = vec![];

    for (player, decks) in Deck::all_decks_grouped_by_player(players, &conn)? {
        let deck_response = DeckResponse::into_deck_response(decks, &conn)?;
        response.push(PlayerDetailResponse {
            id: player.id,
            alias: player.alias,
            decks: deck_response,
        });
    }

    Ok(Json(response))
}

#[get("/<id>")]
pub fn get_player(
    id: i32,
    conn: DbConn,
    _token: ApiToken,
) -> Result<Json<PlayerDetailResponse>, ApiError> {
    let player = Player::find(id, &conn)?;
    let decks = Deck::find_by_player(&player, &conn)?;

    let response = PlayerDetailResponse {
        id: player.id,
        alias: player.alias,
        decks: DeckResponse::into_deck_response(decks, &conn)?,
    };

    Ok(Json(response))
}

#[put("/", format = "application/json", data = "<req>")]
pub fn update_player(
    req: Json<UpdatePlayerRequest>,
    conn: DbConn,
    _token: ApiToken,
) -> Result<Json<PlayerResponse>, ApiError> {
    let update_request = req.into_inner();

    let old_player = Player::find(update_request.id, &conn)?;
    let new_player = Player::update(
        Player {
            id: old_player.id,
            alias: update_request.alias.unwrap_or(old_player.alias),
            email: update_request.email.unwrap_or(old_player.email),
            password: old_player.password,
        },
        &conn,
    )?;

    let response = PlayerResponse {
        id: new_player.id,
        alias: new_player.alias,
    };

    Ok(Json(response))
}
