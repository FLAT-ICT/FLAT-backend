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
#[derive(Queryable, Serialize)]
pub struct SearchUser {
    pub user_id: i32,
    pub user_name: String,
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
#[derive(Serialize)]
pub struct FriendList {
    pub one_side: Vec<UserView>,
    pub mutual: Vec<UserView>,
}

#[derive(Serialize)]
pub struct Friend {}

// 正規表現をグローバルに宣言
// static USER_ID: Lazy<regex::Regex> = Lazy::new(|| Regex::new(r"[A-Z0-9]{6}$").unwrap());

#[derive(Serialize)]
pub struct ResultMessage {
    pub message: String,
}

// the input to our `create_user` handler
#[derive(Deserialize)]
pub struct CreateUser {
    pub username: String,
}

#[derive(Serialize, Queryable)]
pub struct UserView {
    pub user_id: i32,
    pub user_name: String,
    pub status: i32,
    pub icon_path: String,
    pub beacon: Option<String>,
}
