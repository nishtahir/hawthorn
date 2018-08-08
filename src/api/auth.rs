use api::error::ApiError;
use bcrypt::{hash, verify};
use db::DbConn;
use dotenv::dotenv;
use jsonwebtoken::{decode, Validation};
use jsonwebtoken::{encode, Header};
use models::player::Player;
use rocket::http::Status;
use rocket::request::{self, FromRequest, Request};
use rocket::Outcome;
use rocket_contrib::Json;
use std::env;
use time;

pub const DEFAULT_TOKEN_VALIDITY: i64 = 604800; // 7 days

#[derive(Serialize, Deserialize)]
pub struct Claims {
    pub id: String,
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

pub struct ApiToken(String);

impl<'a, 'r> FromRequest<'a, 'r> for ApiToken {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<ApiToken, ()> {
        let keys: Vec<_> = request.headers().get("x-api-key").collect();
        if keys.len() != 1 {
            return Outcome::Failure((Status::BadRequest, ()));
        }

        dotenv().ok();
        let secret = env::var("AUTH_SECRET").expect("AUTH_SECRET must be set");

        let key = keys[0];
        let validation = Validation::default();
        match decode::<Claims>(&key, secret.as_ref(), &validation) {
            Ok(_) => Outcome::Success(ApiToken(key.to_string())),
            Err(_) => Outcome::Failure((Status::Unauthorized, ())),
        }
    }
}

#[post("/login", format = "application/json", data = "<req>")]
pub fn login(req: Json<LoginRequest>, conn: DbConn) -> Result<Json<LoginResponse>, ApiError> {
    dotenv().ok();
    let secret = env::var("AUTH_SECRET").expect("AUTH_SECRET must be set");
    let login_req = req.into_inner();
    let player = Player::find_by_email(&login_req.email, &conn)?;
    let is_match = verify(&login_req.password, &player.password)?;
    if is_match {
        let claims = Claims {
            id: player.id.to_string(),
            exp: current_time() + DEFAULT_TOKEN_VALIDITY,
        };
        let token = encode(&Header::default(), &claims, secret.as_ref())?;
        Ok(Json(LoginResponse { token }))
    } else {
        Err(ApiError::Unauthorized)
    }
}

#[put("/password", format = "application/json", data = "<req>")]
pub fn change_password(req: Json<ChangePasswordRequest>, conn: DbConn) -> Result<(), ApiError> {
    let change_password_req = req.into_inner();

    let player = Player::find_by_email(&change_password_req.email, &conn)?;
    let old_pass_is_valid = verify(&change_password_req.old_password, &player.password)?;

    if old_pass_is_valid {
        let new_hash = hash(&change_password_req.new_password, /*cost*/ 8)?;
        let _ = player.update_password(new_hash, &conn);
        Ok({})
    } else {
        Err(ApiError::Unauthorized)
    }
}

fn current_time() -> i64 {
    time::get_time().sec
}
