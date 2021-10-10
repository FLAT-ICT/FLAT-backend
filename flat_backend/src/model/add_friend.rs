use regex::Regex;

use serde::{Deserialize, Serialize};
use validator::{Validate, ValidationError, ValidationErrors};

use once_cell::sync::Lazy;

// use crate::friends::view::IdPair;

use super::super::view::IdPair;

use super::db_connect;

// 友だち追加の流れ
// API -> (id, id): (String, String)

fn is_exist_id(id: String) -> bool {
    let user_id = id;
    if user_id.len() != 6 {
        return false;
    }

    let conn = db_connect::establish_connection();
    // let results = p


    // db に接続。チェックする
    true
}

pub fn add_friend(id_pair: IdPair) -> bool {
    let my_id = id_pair.my_id;
    let friend_id = id_pair.friend_id;
    // IDがレコードに存在してるかチェック
    if is_exist_id(my_id) && is_exist_id(friend_id) {
        return true;
    }
    return false;
    // DBにインサート
    // bool か Result を返す
}

// fn get_friend() -> Option {}

// 正規表現をグローバルに宣言
static USER_ID: Lazy<regex::Regex> = Lazy::new(|| Regex::new(r"[A-Z0-9]{6}$").unwrap());

#[derive(Debug, Validate, Deserialize, Serialize)]
pub struct UserId {
    #[validate(regex = "USER_ID")]
    pub id: String,
}

#[derive(Debug, Validate, Deserialize)]
struct User {
    id: UserId,
}

// struct UserId {}
// type UserId = String;
// impl UserId {}

struct Friend {}
