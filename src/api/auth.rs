use api::error::ApiError;
use bcrypt::{hash, verify};
use db::DbConn;
use dotenv::dotenv;
use jsonwebtoken::errors::Error as JwtError;
use jsonwebtoken::{decode, Validation};
use jsonwebtoken::{encode, Header};
use models::player::Player;
use rocket::http::Status;
use rocket::request::{self, FromRequest, Request};
use rocket::Outcome;
use rocket_contrib::json::Json;
use std::env;
use time;

pub const DEFAULT_TOKEN_VALIDITY: i64 = 604800; // 7 days

lazy_static! {
    static ref AUTH_SECRET: String = {
        dotenv().ok();
        env::var("AUTH_SECRET").expect("AUTH_SECRET must be set")
    };
}

#[derive(Serialize, Deserialize)]
pub struct Claims {
    pub id: i32,
    pub exp: i64,
}

#[derive(Deserialize)]
pub struct LoginRequest {
    email: String,
    password: String,
}
#[derive(Serialize)]
pub struct LoginResponse {
    token: String,
}

#[derive(Deserialize)]
pub struct ChangePasswordRequest {
    email: String,
    old_password: String,
    new_password: String,
}

pub struct ApiToken {
    player_id: i32,
    _exp: i64,
    _raw: String,
}

impl<'a, 'r> FromRequest<'a, 'r> for ApiToken {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<ApiToken, ()> {
        let _conn = DbConn::from_request(request);

        let keys: Vec<_> = request.headers().get("x-api-key").collect();
        if keys.len() != 1 {
            return Outcome::Failure((Status::BadRequest, ()));
        }

        let key = keys[0];
        let validation = Validation::default();
        match decode::<Claims>(&key, AUTH_SECRET.as_ref(), &validation) {
            Ok(data) => {
                let claims = data.claims;
                let api_token = ApiToken {
                    player_id: claims.id,
                    _exp: claims.exp,
                    _raw: key.to_string(),
                };
                Outcome::Success(api_token)
            }
            Err(_) => Outcome::Failure((Status::Unauthorized, ())),
        }
    }
}

#[post("/login", format = "application/json", data = "<req>")]
pub fn login(req: Json<LoginRequest>, conn: DbConn) -> Result<Json<LoginResponse>, ApiError> {
    let login_req = req.into_inner();
    let player = Player::find_by_email(&login_req.email, &conn)?;

    if verify(&login_req.password, &player.password)? {
        let token = create_token(player.id)?;
        Ok(Json(LoginResponse { token }))
    } else {
        Err(ApiError::Unauthorized)
    }
}

#[post("/refresh", format = "application/json")]
pub fn refresh_token(_conn: DbConn, token: ApiToken) -> Result<Json<LoginResponse>, ApiError> {
    let new_token = create_token(token.player_id)?;
    Ok(Json(LoginResponse { token: new_token }))
}

#[put("/password", format = "application/json", data = "<req>")]
pub fn change_password(req: Json<ChangePasswordRequest>, conn: DbConn) -> Result<(), ApiError> {
    let change_password_req = req.into_inner();

    let player = Player::find_by_email(&change_password_req.email, &conn)?;
    let old_pass_is_valid = verify(&change_password_req.old_password, &player.password)?;

    if old_pass_is_valid {
        let new_hash = hash(&change_password_req.new_password, /*cost*/ 8)?;
        let _ = player.update_password(new_hash, &conn);
        Ok({}) // just respond with 200 OK
    } else {
        Err(ApiError::Unauthorized)
    }
}

fn create_token(id: i32) -> Result<String, JwtError> {
    let claims = Claims {
        id: id,
        exp: current_time() + DEFAULT_TOKEN_VALIDITY,
    };
    encode(&Header::default(), &claims, AUTH_SECRET.as_ref())
}

fn current_time() -> i64 {
    time::get_time().sec
}
