use bcrypt::BcryptError;
use diesel::result::Error as DieselError;
use jsonwebtoken::errors::Error as JwtError;
use rocket::http::ContentType;
use rocket::http::Status;
use rocket::request::Request;
use rocket::response::{Responder, Response};
use rocket_contrib::json::Json;
use std::convert::From;

#[derive(Debug)]
pub enum ApiError {
    NotFound,
    BadRequest,
    InternalServerError,
    Unauthorized,
}

#[derive(Serialize)]
pub struct ErrorResponse {
    code: u16,
    message: String,
}

impl From<DieselError> for ApiError {
    fn from(e: DieselError) -> Self {
        match e {
            DieselError::NotFound => ApiError::NotFound,
            _ => ApiError::InternalServerError,
        }
    }
}

impl From<BcryptError> for ApiError {
    fn from(e: BcryptError) -> Self {
        match e {
            _ => ApiError::InternalServerError,
        }
    }
}

impl From<JwtError> for ApiError {
    fn from(e: JwtError) -> Self {
        match e {
            _ => ApiError::InternalServerError,
        }
    }
}

impl From<ApiError> for Status {
    fn from(e: ApiError) -> Self {
        match e {
            ApiError::NotFound => Status::NotFound,
            ApiError::BadRequest => Status::BadRequest,
            ApiError::InternalServerError => Status::InternalServerError,
            ApiError::Unauthorized => Status::Unauthorized,
        }
    }
}

impl From<Status> for ErrorResponse {
    fn from(status: Status) -> Self {
        ErrorResponse {
            code: status.code,
            message: status.reason.to_string(),
        }
    }
}

impl<'r> Responder<'r> for ApiError {
    fn respond_to(self, req: &Request) -> Result<Response<'r>, Status> {
        let status = Status::from(self);
        let json_body = Json(ErrorResponse::from(status));

        match json_body.respond_to(req) {
            Ok(json_response) => Ok(Response::build_from(json_response)
                .status(status)
                .header(ContentType::JSON)
                .finalize()),
            Err(_) => Err(Status::InternalServerError),
        }
    }
}

///
///  These catchers are needed in order to provide a custom
///  error response for request guards. Ideally we should be
///  able to remove these as of Rocket 0.4
///  Ref: https://github.com/SergioBenitez/Rocket/issues/596
///
#[catch(400)]
pub fn handle_400(_: &Request) -> Json<ErrorResponse> {
    Json(ErrorResponse::from(Status::BadRequest))
}

#[catch(401)]
pub fn handle_401(_: &Request) -> Json<ErrorResponse> {
    Json(ErrorResponse::from(Status::Unauthorized))
}

#[catch(404)]
pub fn handle_404(_: &Request) -> Json<ErrorResponse> {
    Json(ErrorResponse::from(Status::NotFound))
}
