use serde::{Deserialize, Serialize};
use validator::{Validate, ValidationError, ValidationErrors};

// 友だち追加の流れ
// API -> (id, id): (String, String)

fn is_exist_id(id: String) -> bool {
    // db に接続。チェックする
    true
}

fn add_friend(id: String, friend_id: String) {
    // IDがレコードに存在してるかチェック
    is_exist_id(id);
    is_exist_id(friend_id);
    // DBにインサート
    // bool か Result を返す
}

fn get_friend() -> Option {}

lazy_static! {
    static ref USER_ID: Regex = Regex::new(r"[A-Z]{6}").unwrap();
}

#[derive(Debug, Validate, Deserialize)]
pub struct UserId {
    #[validate(regex = "USER_ID")]
    pub id: String,
}

struct User {
    id: UserId,
}

// struct UserId {}
// type UserId = String;
// impl UserId {}

struct Friend {}
