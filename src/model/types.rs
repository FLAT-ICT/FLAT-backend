use chrono::NaiveDateTime;
use once_cell::sync::Lazy;
use regex::Regex;
use serde::{Deserialize, Serialize};
use validator::Validate;

// 正規表現をグローバルに宣言
static USER_ID: Lazy<regex::Regex> = Lazy::new(|| Regex::new(r"[0-9]{6}$").unwrap());

#[derive(Debug, Validate, Deserialize, Serialize)]
pub struct UserId {
    #[validate(regex = "USER_ID")]
    pub id: String,
}

#[derive(Debug, Validate, Deserialize, Queryable)]
pub struct User {
    pub user_id: i32,
    pub user_name: String,
    pub status: i32,
    pub beacon: Option<String>,
    pub icon_path: String,
    pub hashed_password: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

// struct UserId {}
// type UserId = String;
// impl UserId {}

#[derive(Queryable)]
pub struct Friend {
    pub acctive: i32,
    pub passive: i32,
    pub created_at: NaiveDateTime,
    pub blocked_at: Option<NaiveDateTime>,
}

// #[derive(Queryable, Serialize)]
// pub struct SearchUser {
//     pub user_id: i32,
//     pub user_name: String,
//     pub icon_path: String,
//     pub applied: bool,
//     pub requested: bool,
// }

pub enum SomeError {
    ValidationError,
    NotExistError,
    SameIdError,
}

struct IdAndName {
    pub user_id: i32,
    pub user_name: String,
}

// #[derive(Serialize)]
// pub struct FriendList {
//     pub one_side: Vec<UserView>,
//     pub mutual: Vec<UserView>,
// }

// #[derive(Validate, Serialize, Queryable)]
// pub struct UserView {
//     // #[validate(regex = "USER_ID")]
//     pub user_id: i32,
//     pub user_name: String,
//     pub status: i32,
//     pub icon_path: String,
//     pub beacon: Option<String>,
// }
