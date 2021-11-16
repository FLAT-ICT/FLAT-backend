use std::collections::HashMap;

use crate::model;
use crate::model::friends::search_user;
use crate::view::{FriendList, IdAndName, IdPair, ResultMessage, SearchUser};
use axum::extract::{Path, Query};
use axum::{response::IntoResponse, Json};
use hyper::StatusCode;
use model::friends::{self, get_friend_list};
use model::types::SomeError;

pub async fn add_friend(Json(payload): Json<IdPair>) -> impl IntoResponse {
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

pub async fn reject_friend(Json(payload): Json<IdPair>) -> impl IntoResponse {
    let api_result = friends::reject_friend(payload);

    let ok_or_ng: String;
    if let true = api_result {
        ok_or_ng = "Ok".to_string()
    } else {
        ok_or_ng = "Ng".to_string()
    }

    let result_message = ResultMessage { message: ok_or_ng };
    (StatusCode::OK, Json(result_message))
}

pub async fn check_friend_status(
    Query(payload): Query<IdAndName>,
) -> Result<(StatusCode, axum::Json<Vec<SearchUser>>), SomeError> {
    let result = search_user(payload);
    match result {
        Ok(v) => {
            println!("{:#?}", v);
            return Ok((StatusCode::OK, Json(v)))},
        Err(e) => {
            println!("{:#?}", e);
            return Err(e);
        }
    };

    // 200のときの処理

    // それ以外のときの処理
    // (StatusCode::from_u16(result).unwrap(), Json(ResultMessage { message: result.1 }))
}

// 構造体かHashMapで受けなきゃいけなかった
pub async fn friend_list(Query(user_id): Query<HashMap<String, i32>>) -> impl IntoResponse {
    
    let fl = get_friend_list(*(user_id.get("my_id").unwrap()));
    println!("{:#?}", fl);
    return (StatusCode::OK, Json(fl));
}
