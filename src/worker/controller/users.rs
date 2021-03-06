use crate::worker::{
    model::{
        db_util::{is_exist_id, is_exist_name},
        types::{SomeError, UserId},
        users::{self, is_logged_in},
    },
    view::{
        IdAndName, IdAndStatus, IsOtherUserLoggedIn, PreLoginView, ResultMessage, ScannedBeacon,
        UserCredential, UserIdTimestamp, UserNameTimestamp, UserTimestamp, UserView, IdAndIcon,
    },
};
use axum::{response::IntoResponse, Json};
use hyper::StatusCode;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize, Deserialize)]
struct Id {
    user_id: i32,
}

pub async fn create_user(
    // this argument tells axum to parse the request body
    // as JSON into a `CreateUser` type
    Json(payload): Json<UserCredential>,
) -> Result<(StatusCode, axum::Json<UserView>), SomeError> {
    if let Err(_) = payload.validate() {
        return Err(SomeError::InvalidValidation);
    }

    if let true = is_exist_name(&payload.name) {
        return Err(SomeError::AlreadyExistName);
    }

    if let Ok(inserted) = users::create_user(
        UserCredential {
            name: payload.name,
            password: payload.password,
        }
        .to_hash(),
    ) {
        Ok((StatusCode::OK, Json(inserted)))
    } else {
        Err(SomeError::AlreadyExistName)
    }

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
}

pub async fn login(Json(credential): Json<UserCredential>) -> impl IntoResponse {
    if let Err(_) = credential.validate() {
        return Err(SomeError::InvalidValidation);
    }
    match users::login(credential) {
        Ok(result) => Ok((StatusCode::OK, Json(result))),
        Err(e) => Err(e),
    }

    // 400 invalid password
    // 404 user notfound
    // ()
}

pub async fn is_loggedin(Json(user_timestamp): Json<UserIdTimestamp>) -> impl IntoResponse {
    let result = users::is_logged_in(UserTimestamp::I(user_timestamp));
    (StatusCode::OK, Json(result))
}

pub async fn logout(
    Json(user_id): Json<UserId>,
) -> Result<(StatusCode, axum::Json<ResultMessage>), SomeError> {
    match users::logout(user_id.id) {
        Ok(_) => Ok((
            StatusCode::OK,
            Json(ResultMessage {
                message: "Ok".to_string(),
            }),
        )),
        Err(e) => Err(e),
    }
}

pub async fn pre_login(
    Json(payload): Json<PreLoginView>,
) -> Result<(StatusCode, Json<IsOtherUserLoggedIn>), SomeError> {
    let name = &payload.name;
    let password = &payload.password;
    let loggedin_at = &payload.loggedin_at;
    let pv = &payload;

    if (password.is_some() && loggedin_at.is_some()) || (password.is_none() && password.is_none()) {
        return Err(SomeError::InvalidStructure);
    }

    // ログイン時 ログインされてたらTrueを返す
    if let Some(_) = password {
        match users::pre_login(pv) {
            Err(e) => return Err(e),
            Ok(v) => {
                return Ok((
                    StatusCode::OK,
                    Json(IsOtherUserLoggedIn {
                        others: v.is_some(),
                    }),
                ));
            }
        };
    }

    // 通常起動時
    if let Some(l) = loggedin_at {
        let result = is_logged_in(UserTimestamp::N(UserNameTimestamp {
            name: name.to_string(),
            logged_in_at: l.to_owned(),
        }));
        return Ok((
            StatusCode::OK,
            Json(IsOtherUserLoggedIn {
                others: result.others,
            }),
        ));
    }

    return Ok((StatusCode::OK, Json(IsOtherUserLoggedIn { others: true })));
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

pub async fn update_name(Json(payload): Json<IdAndName>) -> impl IntoResponse {
    // 200
    // 400 same name error
    // 404 id not exist
    // 422 validation error
    if let false = is_exist_id(payload.my_id) {
        return Err(SomeError::NotExist);
    }
    if let Err(_) = payload.validate() {
        return Err(SomeError::InvalidValidation);
    }
    match users::update_name(payload.my_id, payload.target_name) {
        Ok(v) => return Ok((StatusCode::OK, Json(v))),
        Err(e) => return Err(e),
    };
}

pub async fn update_status(
    Json(payload): Json<IdAndStatus>,
) -> Result<(StatusCode, axum::Json<UserView>), SomeError> {
    if let false = is_exist_id(payload.id) {
        return Err(SomeError::NotExist);
    }
    if 0 <= payload.status && payload.status <= 3 {
        let result = users::update_status(payload.id, payload.status).unwrap();
        return Ok((StatusCode::OK, Json(result)));
    } else {
        return Err(SomeError::InvalidValidation);
    }
}

pub async fn update_icon(
    Json(payload): Json<IdAndIcon>
) -> Result<(StatusCode, axum::Json<UserView>) , SomeError> {
    println!("called");
    if let false = is_exist_id(payload.id) {
        return Err(SomeError::NotExist);
    }
    match users::update_icon(payload.id, payload.image).await {
        Ok(result) => Ok((StatusCode::OK, Json(result))),
        Err(e) => Err(e),
    }
}