use reqwest as rw;
use serde::Deserialize;
use thiserror::Error;

// TODO
// Now we will be returned ApiError but it would be better probably to have strong typed errors.

pub use http::uri::InvalidUri;

/// error code in unsucesfull response json
#[derive(Debug, Deserialize, Clone, Copy, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ErrorCode {
    #[serde(rename = "internal error")]
    InternalError,
    #[serde(rename = "not found")]
    NotFound,
    Conflict,
    Invalid,
    #[serde(rename = "unprocessable entity")]
    UnprocessableEntity,
    #[serde(rename = "empty value")]
    EmptyValue,
    Unavailable,
    Forbidden,
    #[serde(rename = "too many request")]
    TooManyReuqests,
    Unauthorized,
    #[serde(rename = "method not allowed")]
    MethodNotAllowed,
}

/// Error Response from API
#[derive(Debug, Deserialize, Error)]
#[error("Influx DB v2 API => {:?}, message: {}", .code, .message)]
pub struct ApiError {
    code: ErrorCode,
    message: String,
}

impl ApiError {
    pub fn code(&self) -> ErrorCode {
        self.code
    }
}

#[derive(Debug, Error)]
pub enum Error {
    #[error("{}", .0)]
    Client(#[from] rw::Error),
    #[error("{}", .0)]
    Api(#[from] ApiError),
}
