use axum::response::IntoResponse;
use hyper::{Body, Response, StatusCode};
use serde::{Deserialize, Serialize};

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
}

impl IntoResponse for SomeError {
    type Body = Body;
    type BodyError = <Self::Body as axum::body::HttpBody>::Error;
    fn into_response(self) -> Response<Self::Body> {
        let body = match self {
            SomeError::InvalidValidation => Body::from("invalid validation"),
            SomeError::NotExist => Body::from("user not found"),
            SomeError::AlreadyExistName => Body::from("the name is alreasy used"),
            SomeError::InvalidPassword => Body::from("user not found"),
            SomeError::InvalidStructure => Body::from("invalid structure"),
        };

        let status = match self {
            SomeError::InvalidValidation => StatusCode::UNPROCESSABLE_ENTITY,
            SomeError::NotExist => StatusCode::NOT_FOUND,
            SomeError::AlreadyExistName => StatusCode::BAD_REQUEST,
            SomeError::InvalidPassword => StatusCode::NOT_FOUND,
            SomeError::InvalidStructure => StatusCode::BAD_REQUEST,
        };

        Response::builder().status(status).body(body).unwrap()
    }
}
