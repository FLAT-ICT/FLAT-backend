use crate::{
    model::users,
    repository::NameAndPassword,
    view::{CreateUser, ResultMessage, ScannedBeacon},
};
use axum::{response::IntoResponse, Json};
use hyper::StatusCode;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Id {
    user_id: i32,
}

pub async fn create_user(
    // this argument tells axum to parse the request body
    // as JSON into a `CreateUser` type
    Json(payload): Json<CreateUser>,
) -> impl IntoResponse {
    let inserted = users::create_user(NameAndPassword {
        name: &payload.name,
        hashed_password: &payload.password,
    });


    // 実装するものたち 
    // TODO: パスワードのバリデーションをする
    // TODO: パスワードのハッシュ化を行う
    // TODO: 名前の重複チェックを行う
    // TODO: 排他的ログイン
    // TODO: ForceLogin
    // TODO: あるアカウントがログインされているかのチェックフラグ
    // TODO: 別のクライアントでログインしたとき、旧クライアントはログアウトされるようにするが、それをどう実現するか決める

    // this will be converted into a JSON response
    // with a status code of `201 Created`
    (StatusCode::OK, Json(inserted))
}

// #[derive(Deserialize)]
// pub struct BeaconIdnetifier {
//     major: i32,
//     minor: i32,
// }

pub async fn update_beacon(Json(payload): Json<ScannedBeacon>) -> impl IntoResponse {
    if let true = users::update_beacon(payload.user_id, payload.major, payload.minor) {
        (
            StatusCode::OK,
            Json(ResultMessage {
                message: "Ok".to_string(),
            }),
        )
    } else {
        (
            StatusCode::NOT_FOUND,
            Json(ResultMessage {
                message: "Ng".to_string(),
            }),
        )
    }
}
