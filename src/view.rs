use chrono::NaiveDateTime;
use regex::Regex;
// use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use validator::{Validate, ValidationError};

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
#[derive(Deserialize, Serialize, Validate)]
pub struct UserCredential {
    #[validate(length(min = 1, max = 10))]
    pub name: String,
    #[validate(custom = "validate_password")]
    pub password: String,
}

// fn validate_name(name: &str) -> Result<(), ValidationError> {
//     TODO: 不正な文字の検知とかあったらここでする
//     let re = Regex::new(r".{1, 10}").unwrap();
//     if let false = re.is_match(name) {
//         return Err(ValidationError::new("invalid validation of name"));
//     }
//     Ok(())
// }

fn validate_password(password: &str) -> Result<(), ValidationError> {
    let re = Regex::new(r"[[:alnum:]]{8,256}").unwrap();
    if let false = re.is_match(password) {
        return Err(ValidationError::new("invalid validation of password"));
    }
    Ok(())
}

#[derive(Deserialize, Serialize)]
pub struct PreLoginView {
    pub name: String,
    pub password: Option<String>,
    pub loggedin_at: Option<NaiveDateTime>,
}
#[derive(Deserialize, Serialize)]
pub struct IsOtherUserLoggedIn {
    pub others: bool,
}

#[derive(Serialize, Queryable, Debug, Deserialize, PartialEq)]
pub struct UserView {
    pub id: i32,
    pub name: String,
    pub status: i32,
    pub icon_path: String,
    pub spot: Option<String>,
    pub logged_in_at: Option<NaiveDateTime>,
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

pub enum UserTimestamp {
    I(UserIdTimestamp),
    N(UserNameTimestamp),
}

#[derive(Deserialize)]
pub struct UserIdTimestamp {
    pub id: i32,
    pub logged_in_at: NaiveDateTime,
}

#[derive(Deserialize)]
pub struct UserNameTimestamp {
    pub name: String,
    pub logged_in_at: NaiveDateTime,
}

#[derive(Serialize)]
pub struct IsLoggedIn {
    pub own: bool,
    pub others: bool,
}
