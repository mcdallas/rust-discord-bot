
use serde::Serialize;
use std::fmt;

use crate::error::Error;


#[derive(Serialize)]
pub(crate) struct HttpResponse {
    pub status: u16,
    pub body: String,
}

#[derive(Debug)]
pub(crate) enum HttpStatus {
    BadRequest = 400,
    Unauthorized = 401,
    InternalServerError = 500,
}

#[derive(Debug)]
pub struct HttpError {
    pub(crate) status: HttpStatus,
    reason: Error,
}

impl fmt::Display for HttpError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "An HTTP error occurred: {}", self.reason.to_string())
    }
}

impl From<Error> for HttpError {
    fn from(error: Error) -> HttpError {
        HttpError {
            status: match &error {
                Error::HeaderNotFound(_) | Error::JsonFailed(_) | Error::InvalidPayload(_) => {
                    HttpStatus::BadRequest
                }
                Error::VerificationFailed(_) => HttpStatus::Unauthorized,
                _ => HttpStatus::InternalServerError,
            },
            reason: error,
        }
    }
}