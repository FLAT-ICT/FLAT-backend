use crate::view::{self, UserView};
use axum::{response::IntoResponse, Json};
use hyper::StatusCode;

pub async fn create_user(
    // this argument tells axum to parse the request body
    // as JSON into a `CreateUser` type
    Json(payload): Json<view::CreateUser>,
) -> impl IntoResponse {
    // insert your application logic here
    let user = UserView {
        user_id: 0,
        user_name: payload.username,
        status: 1,
        icon_path: "".to_string(),
        beacon: Some("595教室".to_string()),
    };

    // this will be converted into a JSON response
    // with a status code of `201 Created`
    (StatusCode::CREATED, Json(user))
}
