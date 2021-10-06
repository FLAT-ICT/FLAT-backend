pub mod add_friend;
pub mod connect_db;
// use regex::Regex;

// use serde::{Deserialize, Serialize};
// use validator::{Validate, ValidationError, ValidationErrors};

// use once_cell::sync::Lazy;

// // 友だち追加の流れ
// // API -> (id, id): (String, String)

// fn is_exist_id(id: UserId) -> bool {
//     let user_id = id.id;
//     // db に接続。チェックする
//     true
// }

// fn add_friend(id: UserId, friend_id: UserId) {
//     // IDがレコードに存在してるかチェック
//     is_exist_id(id);
//     is_exist_id(friend_id);
//     // DBにインサート
//     // bool か Result を返す
// }

// // fn get_friend() -> Option {}

// // 正規表現をグローバルに宣言
// static USER_ID: Lazy<regex::Regex> = Lazy::new(|| Regex::new(r"[A-Z0-9]{6}$").unwrap());

// #[derive(Debug, Validate, Deserialize, Serialize)]
// pub struct UserId {
//     #[validate(regex = "USER_ID")]
//     pub id: String,
// }

// #[derive(Debug, Validate, Deserialize)]
// struct User {
//     id: UserId,
// }

// // struct UserId {}
// // type UserId = String;
// // impl UserId {}

// struct Friend {}
