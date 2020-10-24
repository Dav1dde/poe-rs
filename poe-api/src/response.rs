use std::result::Result;
use serde::{Deserialize};
use reqwest;

#[derive(Debug)]
pub enum PoeError {
    ApiError(ApiError),
    Reqwest(reqwest::Error)
}

pub type PoeResult<T> = Result<T, PoeError>;


#[derive(Debug, Deserialize)]
pub struct ApiErrorResponse {
    pub error: ApiError
}

#[derive(Debug, Deserialize)]
pub struct ApiError {
    code: i32,
    message: String
}
