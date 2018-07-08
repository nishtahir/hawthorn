use api::auth::ApiToken;
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

#[derive(Serialize)]
pub struct PlayerResponse {
    id: i32,
    alias: String,
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
fn get_players(conn: DbConn) -> Result<Json<Vec<PlayerResponse>>, ApiError> {
    let players = Player::all(&conn)?;
    let response = players
        .into_iter()
        .map(|player| PlayerResponse {
            id: player.id,
            alias: player.alias,
        })
        .collect();
    Ok(Json(response))
}

#[derive(Serialize)]
pub struct PlayerDetailResponse {
    id: i32,
    alias: String,
    decks: Vec<Deck>,
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
        decks: decks,
    };

    Ok(Json(response))
}
