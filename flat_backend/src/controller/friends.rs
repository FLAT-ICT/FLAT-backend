use axum::{response::IntoResponse, Json};
use hyper::StatusCode;

use super::super::view::{self, ResultMessage};

use super::super::model::friends;

pub async fn add_friend(Json(payload): Json<view::IdPair>) -> impl IntoResponse {
    let api_result = friends::add_friend(payload);

    let ok_or_ng: String;
    if let true = api_result {
        ok_or_ng = "Ok".to_string()
    } else {
        ok_or_ng = "Ng".to_string()
    }

    let result_message = ResultMessage { message: ok_or_ng };
    (StatusCode::OK, Json(result_message))
}

pub async fn check_friend_status(Json(payload): Json<view::IdPair>) -> impl IntoResponse {
    // if Ok
    //   (id: String, name: String, icon_path: String, applied: bool, requested: bool)
    // if Error
    //   (status_code: int, message: String)
    //
    // 200: Ok
    // 404: id is not found
    // 422: invalid validation
    // 422: invalid structure
    // 471: can't assign same id

    let result = friends::search_user(payload);

    // 200のときの処理

    // それ以外のときの処理
    // (StatusCode::from_u16(result).unwrap(), Json(ResultMessage { message: result.1 }))
    ()
}
