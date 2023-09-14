use okapi::openapi3::Responses;
use rocket::{
    http::Status,
    response::{status, Responder},
    serde::json::Json,
    Request,
};
use rocket_okapi::{gen::OpenApiGenerator, response::OpenApiResponderInner};
use schemars::{gen::SchemaGenerator, schema::Schema, JsonSchema};
use serde::Serialize;
use std::{future::Future, io, num::ParseIntError, string::FromUtf8Error};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ApiError {
    // Internal Errors
    #[error("Io error: `{0}`")]
    Io(#[from] io::Error),
    #[error("ParseIntError: `{0}`")]
    ParseInt(#[from] ParseIntError),
    #[error("Csv error: `{0}`")]
    Csv(#[from] csv::Error),
    #[error("Utf8 error: `{0}`")]
    Utf8(#[from] FromUtf8Error),
    #[error("Json error: `{0}`")]
    Json(#[from] serde_json::Error),
    #[error("Reqwest error: `{0}`")]
    Reqwest(#[from] reqwest::Error),
    #[error("Missing Dict Headers")]
    MissingDictHeaders,

    // User Errors
    #[error("Invalid symbol: `{0}`")]
    InvalidSymbol(String),
}

impl<'r> Responder<'r, 'static> for ApiError {
    fn respond_to(self, request: &'r Request<'_>) -> rocket::response::Result<'static> {
        let msg = self.to_string();
        match self {
            ApiError::Json(_)
            | ApiError::Io(_)
            | ApiError::Utf8(_)
            | ApiError::Reqwest(_)
            | ApiError::Csv(_)
            | ApiError::MissingDictHeaders
            | ApiError::ParseInt(_) => Status::InternalServerError.respond_to(request),
            ApiError::InvalidSymbol(_) => status::BadRequest(Some(msg)).respond_to(request),
        }
    }
}

pub type ApiResult<T> = Result<T, ApiError>;

pub enum ApiResponse<T> {
    Ok(T),
    Error(ApiError),
}

impl<'r, T: Serialize + JsonSchema> Responder<'r, 'static> for ApiResponse<T> {
    fn respond_to(self, request: &'r Request<'_>) -> rocket::response::Result<'static> {
        match self {
            ApiResponse::Ok(value) => Json(value).respond_to(request),
            ApiResponse::Error(err) => err.respond_to(request),
        }
    }
}

impl<T: JsonSchema> JsonSchema for ApiResponse<T> {
    fn schema_name() -> String {
        T::schema_name()
    }

    fn json_schema(gen: &mut SchemaGenerator) -> Schema {
        T::json_schema(gen)
    }
}

impl<T: JsonSchema + Serialize + Send> OpenApiResponderInner for ApiResponse<T> {
    fn responses(gen: &mut OpenApiGenerator) -> rocket_okapi::Result<Responses> {
        Json::<T>::responses(gen)
    }
}

pub async fn api_response<T, F>(f: impl FnOnce() -> F) -> ApiResponse<T>
where
    F: Future<Output = ApiResult<T>>,
{
    let result = f().await;

    match result {
        Ok(value) => ApiResponse::Ok(value),
        Err(err) => {
            eprintln!("ApiError: {}", err.to_string());
            ApiResponse::Error(err)
        }
    }
}
