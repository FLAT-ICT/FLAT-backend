use axum::response::IntoResponse;
use hyper::{Body, Response, StatusCode};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct UserId {
    pub id: i32,
}

#[derive(Debug)]
pub enum SomeError {
    ValidationError,
    NotExistError,
    InvalidPasswordError,
    SameNameError,
    InvalidStructure,
}

impl IntoResponse for SomeError {
    type Body = Body;
    type BodyError = <Self::Body as axum::body::HttpBody>::Error;
    fn into_response(self) -> Response<Self::Body> {
        let body = match self {
            SomeError::ValidationError => Body::from("invalid validation"),
            SomeError::NotExistError => Body::from("user not found"),
            SomeError::SameNameError => Body::from("the name is alreasy used"),
            SomeError::InvalidPasswordError => Body::from("user not found"),
            SomeError::InvalidStructure => Body::from("invalid structure"),
        };

        let status = match self {
            SomeError::ValidationError => StatusCode::UNPROCESSABLE_ENTITY,
            SomeError::NotExistError => StatusCode::NOT_FOUND,
            SomeError::SameNameError => StatusCode::BAD_REQUEST,
            SomeError::InvalidPasswordError => StatusCode::NOT_FOUND,
            SomeError::InvalidStructure => StatusCode::BAD_REQUEST,
        };

        Response::builder().status(status).body(body).unwrap()
    }
}
