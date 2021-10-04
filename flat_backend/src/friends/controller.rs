use axum::{response::IntoResponse, Json};
use hyper::StatusCode;

use super::view::{self, ResultMessage};

use super::model::add_friend;

pub async fn add_friend(Json(payload): Json<view::IdPair>) -> impl IntoResponse {
    let api_result = add_friend::add_friend(payload);

    let ok_or_ng: String;
    if let true = api_result {
        ok_or_ng = "Ok".to_string()
    } else {
        ok_or_ng = "Ng".to_string()
    }

    let result_message = ResultMessage { message: ok_or_ng };
    (StatusCode::OK, Json(result_message))
}
