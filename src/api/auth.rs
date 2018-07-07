use dotenv::dotenv;
use jsonwebtoken::{decode, Validation};
use rocket::http::Status;
use rocket::request::{self, FromRequest, Request};
use rocket::Outcome;
use std::env;

#[derive(Serialize, Deserialize)]
pub struct Claims {
    pub id: String,
    pub exp: i64,
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
