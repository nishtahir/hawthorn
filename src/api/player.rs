use api::auth::ApiToken;
use bcrypt::hash;
use db::DbConn;
use models::deck::Deck;
use models::player::{NewPlayer, Player};
use models::{Insertable, Retrievable};
use rocket::http::Status;
use rocket::response::Failure;
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
) -> Result<Json<PlayerResponse>, Failure> {
    let create_player_req = req.into_inner();
    match hash(&create_player_req.password, /*cost*/ 8) {
        Ok(hash) => {
            let new_player = NewPlayer {
                alias: create_player_req.alias,
                email: create_player_req.email,
                password: hash,
            };
            NewPlayer::insert(new_player, &conn)
                .map(|player| {
                    Json(PlayerResponse {
                        id: player.id,
                        alias: player.alias,
                    })
                })
                .map_err(|_| Failure(Status::InternalServerError))
        }
        Err(_) => Err(Failure(Status::InternalServerError)),
    }
}

#[get("/")]
fn get_players(conn: DbConn) -> Result<Json<Vec<PlayerResponse>>, Failure> {
    Player::all(&conn)
        .map(|players| {
            Json(
                players
                    .into_iter()
                    .map(|player| PlayerResponse {
                        id: player.id,
                        alias: player.alias,
                    })
                    .collect(),
            )
        })
        .map_err(|_| Failure(Status::InternalServerError))
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
) -> Result<Json<PlayerDetailResponse>, Failure> {
    match Player::find(id, &conn) {
        Ok(player) => match Deck::find_by_player(&player, &conn) {
            Ok(decks) => Ok(Json(PlayerDetailResponse {
                id: player.id,
                alias: player.alias,
                decks: decks,
            })),
            Err(_) => Err(Failure(Status::InternalServerError)),
        },
        Err(_) => Err(Failure(Status::InternalServerError)),
    }
}
