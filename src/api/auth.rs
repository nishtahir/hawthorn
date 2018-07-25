use bcrypt::verify;
use db::DbConn;
use dotenv::dotenv;
use jsonwebtoken::{decode, Validation};
use jsonwebtoken::{encode, Header};
use models::player::Player;
use rocket::http::Status;
use rocket::request::{self, FromRequest, Request};
use rocket::response::Failure;
use rocket::Outcome;
use rocket_contrib::Json;
use std::env;
use time;

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
pub fn login(req: Json<LoginRequest>, conn: DbConn) -> Result<Json<LoginResponse>, Failure> {
    dotenv().ok();
    let secret = env::var("AUTH_SECRET").expect("AUTH_SECRET must be set");
    let login_req = req.into_inner();
    match Player::find_by_email(&login_req.email, &conn) {
        Ok(player) => match verify(&login_req.password, &player.password) {
            Ok(success) => {
                if success {
                    let claims = Claims {
                        id: player.id.to_string(),
                        exp: current_time() + 604800, // + 7 days
                    };
                    match encode(&Header::default(), &claims, secret.as_ref()) {
                        Ok(token) => Ok(Json(LoginResponse { token })),
                        Err(_) => Err(Failure(Status::InternalServerError)),
                    }
                } else {
                    Err(Failure(Status::Unauthorized))
                }
            }
            Err(_) => Err(Failure(Status::InternalServerError)),
        },
        Err(_) => Err(Failure(Status::BadRequest)),
    }
}

fn current_time() -> i64 {
    time::get_time().sec
}
