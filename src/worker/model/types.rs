use axum::{
    body::{self, Bytes},
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::get,
    Router,
};
use serde::{Deserialize, Serialize};
// use http_body::combinators::box_body::UnsyncBoxBody;

#[derive(Debug, Deserialize, Serialize)]
pub struct UserId {
    pub id: i32,
}

#[derive(Debug)]
pub enum SomeError {
    InvalidValidation,
    NotExist,
    InvalidPassword,
    AlreadyExistName,
    InvalidStructure,
    UploadImageError,
    DontReach,
}

impl IntoResponse for SomeError {
    // type Body = Body;
    // type BodyError = <Self::Body as axum::body::HttpBody>::Error;
    fn into_response(self) -> Response {
        let body = match self {
            SomeError::InvalidValidation => "invalid validation",
            SomeError::NotExist => "user not found",
            SomeError::AlreadyExistName => "the name is alreasy used",
            SomeError::InvalidPassword => "user not found",
            SomeError::InvalidStructure => "invalid structure",
            SomeError::UploadImageError => "can't upload image to gcs",
            SomeError::DontReach => "this error is not reachable",
        };

        let status = match self {
            SomeError::InvalidValidation => StatusCode::UNPROCESSABLE_ENTITY,
            SomeError::NotExist => StatusCode::NOT_FOUND,
            SomeError::AlreadyExistName => StatusCode::BAD_REQUEST,
            SomeError::InvalidPassword => StatusCode::NOT_FOUND,
            SomeError::InvalidStructure => StatusCode::BAD_REQUEST,
            SomeError::UploadImageError => StatusCode::INTERNAL_SERVER_ERROR,
            SomeError::DontReach => StatusCode::BAD_REQUEST,
        };

        // Response::builder().status(status).body(body).unwrap()
        (status, body).into_response()
    }
}
