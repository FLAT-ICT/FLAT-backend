use crate::{
    model::{db_util::insert_user, users},
    repository::NameAndPassword,
    view::{CreateUser, ScannedBeacon},
};
use axum::{response::IntoResponse, Json};
use hyper::StatusCode;
use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize)]
struct Id {
    user_id: i32
}

pub async fn create_user(
    // this argument tells axum to parse the request body
    // as JSON into a `CreateUser` type
    Json(payload): Json<CreateUser>,
) -> impl IntoResponse {
    // insert your application logic here
    // let user = UserView {
    //     user_id: 0,
    //     user_name: payload.user_name,
    //     status: 1,
    //     icon_path: "".to_string(),
    //     beacon: Some("595教室".to_string()),
    // };

    let inserted = insert_user(NameAndPassword {
        user_name: &payload.user_name,
        hashed_password: &payload.password,
    });


    // this will be converted into a JSON response
    // with a status code of `201 Created`
    (StatusCode::OK, Json(inserted))
}

#[derive(Deserialize)]
pub struct BeaconIdnetifier {
    major: i32,
    minor: i32,
}

pub async fn update_beacon(Json(payload): Json<ScannedBeacon>) -> impl IntoResponse {
    users::udpate_beacon(payload.user_id, payload.major, payload.minor)
}
