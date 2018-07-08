use bcrypt::BcryptError;
use diesel::result::Error as DieselError;
use rocket::http::Status;
use rocket::request::Request;
use rocket::response::{Responder, Response};
use std::convert::From;

#[derive(Debug)]
pub enum ApiError {
    NotFound,
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

impl<'r> Responder<'r> for ApiError {
    fn respond_to(self, _: &Request) -> Result<Response<'r>, Status> {
        match self {
            ApiError::NotFound => Err(Status::NotFound),
            _ => Err(Status::InternalServerError),
        }
    }
}
