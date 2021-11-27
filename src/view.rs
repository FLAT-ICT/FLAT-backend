use chrono::NaiveDateTime;
// use once_cell::sync::Lazy;
// use regex::Regex;
use serde::{Deserialize, Serialize};
use validator::Validate;

// `input` /v1/users/check
// `input` /v1/friends/add
// `input` /v1/friends/reject
#[derive(Debug, Validate, Deserialize, Serialize)]
pub struct IdPair {
    pub my_id: i32,
    pub target_id: i32,
}

// `output` /v1/users/check
#[derive(Queryable, Serialize, Deserialize, Clone, Debug)]
pub struct SearchUser {
    pub id: i32,
    pub name: String,
    pub icon_path: String,
    pub applied: bool,
    pub requested: bool,
}

// `output` /v1/friends/add
// `output` /v1/friends/reject
// IntoResponse

// `input` /v1/friends
// Path<i32>

// `output` /v1/friends
// (StatusCode, Json<FriendList>)
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct FriendList {
    pub one_side: Vec<UserView>,
    pub mutual: Vec<UserView>,
}

// #[derive(Serialize)]
// pub struct Friend {}

// 正規表現をグローバルに宣言
// static USER_ID: Lazy<regex::Regex> = Lazy::new(|| Regex::new(r"[A-Z0-9]{6}$").unwrap());

#[derive(Serialize)]
pub struct ResultMessage {
    pub message: String,
}

// the input to our `create_user` handler
#[derive(Deserialize, Serialize)]
pub struct UserCredential {
    pub name: String,
    pub password: String,
}

#[derive(Serialize, Queryable, Debug, Deserialize, PartialEq)]
pub struct UserView {
    pub id: i32,
    pub name: String,
    pub status: i32,
    pub icon_path: String,
    pub spot: Option<String>,
    pub logined_at: Option<NaiveDateTime>,
}

#[derive(Serialize, Deserialize)]
pub struct IdAndName {
    pub my_id: i32,
    pub target_name: String,
}

#[derive(Deserialize, Serialize)]
pub struct ScannedBeacon {
    pub user_id: i32,
    // Beacon & {rssi, distance}
    // pub uuid: String,
    pub major: i32,
    pub minor: i32,
    pub rssi: i32,
    // pub distance: f32,
}

#[derive(Deserialize)]
pub struct UserTimestamp {
    pub id: i32,
    pub logedin_at: String,
}