use axum::{response::IntoResponse, Json};
use hyper::StatusCode;

use super::view::{self, ResultMessage};

pub async fn add_friend(Json(payload): Json<view::IdPair>) -> impl IntoResponse {
    // if let api_result =

    let result_message = ResultMessage {
        message: "Ok".to_string(),
    };
    (StatusCode::OK, Json(result_message))
}
