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
    pub id: i32,
    pub user_id: String,
    pub user_name: String,
    pub status: i32,
    pub beacon: Option<String>,
    pub icon_path: String,
    pub hashed_password: String,
}

// struct UserId {}
// type UserId = String;
// impl UserId {}

#[derive(Queryable)]
pub struct Friend {
    pub id: i32,
    pub acctive: String,
    pub passive: String,
    pub block_flag: bool,
}

use crate::schema::friends;
#[derive(Insertable)]
#[table_name = "friends"]
pub struct AddFriend<'a> {
    pub acctive: &'a str,
    pub pussive: &'a str,
}

#[derive(Queryable, Serialize)]
pub struct SearchUser {
    pub user_id: String,
    pub user_name: String,
    pub icon_path: String,
    pub applied: bool,
    pub requested: bool,
}

pub enum SomeError {
    ValidationError,
    NotExistError,
    SameIdError,
}

struct IdAndName {
    pub user_id: String,
    pub user_name: String,
}
