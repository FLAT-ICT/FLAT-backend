use axum::{response::IntoResponse, Json};
use hyper::StatusCode;

use super::super::view::{self, User};

pub async fn create_user(
    // this argument tells axum to parse the request body
    // as JSON into a `CreateUser` type
    Json(payload): Json<view::CreateUser>,
) -> impl IntoResponse {
    // insert your application logic here
    let user = User {
        id: "000000".to_string(),
        user_name: payload.username,
        status: 1,
        icon_path: "".to_string(),
        beacon: "595教室".to_string(),
    };

    // this will be converted into a JSON response
    // with a status code of `201 Created`
    (StatusCode::CREATED, Json(user))
}
