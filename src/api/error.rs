use bcrypt::BcryptError;
use diesel::result::Error as DieselError;
use rocket::http::ContentType;
use rocket::http::Status;
use rocket::request::Request;
use rocket::response::{Responder, Response};
use rocket_contrib::Json;
use std::convert::From;

#[derive(Debug)]
pub enum ApiError {
    NotFound,
    BadRequest,
    InternalServerError,
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

impl From<ApiError> for Status {
    fn from(e: ApiError) -> Self {
        match e {
            ApiError::NotFound => Status::NotFound,
            ApiError::BadRequest => Status::BadRequest,
            ApiError::InternalServerError => Status::InternalServerError,
        }
    }
}

#[derive(Serialize)]
struct ErrorResponse {
    code: u16,
    message: String,
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
