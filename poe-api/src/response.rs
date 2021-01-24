use serde::Deserialize;
use std::result::Result;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum PoeError {
    #[error("the resource is not available")]
    NotFound(ApiError),
    #[error("unknown API error")]
    UnknownApiError(ApiError),
    #[error("unexpected transport or decoding error occured")]
    Reqwest(#[from] reqwest::Error),
    #[error("deserialization error")]
    Serde(#[from] serde_json::Error),
    #[error("unknown")]
    Unknown,
}

impl From<ApiError> for PoeError {
    fn from(err: ApiError) -> PoeError {
        match err.code {
            1 => PoeError::NotFound(err),
            _ => PoeError::UnknownApiError(err),
        }
    }
}

pub type PoeResult<T> = Result<T, PoeError>;

#[derive(Debug, Deserialize)]
pub struct ApiErrorResponse {
    pub error: ApiError,
}

#[derive(Debug, Deserialize)]
pub struct ApiError {
    pub code: i32,
    pub message: String,
}
