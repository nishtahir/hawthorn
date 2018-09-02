use api::auth::ApiToken;
use api::deck::DeckResponse;
use api::error::ApiError;
use bcrypt::hash;
use db::DbConn;
use models::deck::Deck;
use models::player::{NewPlayer, Player};
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

impl Player {
    fn into_player_response(self) -> PlayerResponse {
        PlayerResponse {
            id: self.id,
            alias: self.alias,
        }
    }
}

impl CreatePlayerRequest {
    fn into_new_player(self) -> Result<NewPlayer, ApiError> {
        Ok(NewPlayer {
            alias: self.alias,
            email: self.email,
            password: hash(&self.password, /*cost*/ 8)?,
        })
    }
}

#[post("/", format = "application/json", data = "<json>")]
pub fn create_player(
    json: Json<CreatePlayerRequest>,
    conn: DbConn,
) -> Result<Json<PlayerResponse>, ApiError> {
    let req = json.into_inner();
    let new_player = NewPlayer::insert(req.into_new_player()?, &conn);

    Ok(Json(new_player?.into_player_response()))
}

#[get("/")]
fn get_players(
    conn: DbConn,
    _token: ApiToken,
) -> Result<Json<Vec<PlayerDetailResponse>>, ApiError> {
    let players = Player::all(&conn)?;
    let decks_by_players = Deck::find_by_players(players, &conn)?;

    let mut response = vec![];
    for (player, decks) in decks_by_players {
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
    let player = Player::find_by_id(id, &conn)?;
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

    let old_player = Player::find_by_id(update_request.id, &conn)?;
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
