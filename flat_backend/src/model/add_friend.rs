use diesel::ExpressionMethods;
use diesel::QueryDsl;
use diesel::RunQueryDsl;
use regex::Regex;

use serde::{Deserialize, Serialize};
use validator::{Validate, ValidationError, ValidationErrors};

use once_cell::sync::Lazy;

use super::super::view::IdPair;

use super::super::schema;

use super::db_connect;

// 友だち追加の流れ
// API -> (id, id): (String, String)

fn is_exist_id(target_id: &str) -> bool {
    // let user_id = id;
    // if id.len() != 6 {
    //     return false;
    // }

    println!("is_exist_id({})", target_id);

    use schema::users::dsl::*;

    let conn = db_connect::establish_connection();
    let user = users.filter(user_id.eq(target_id)).load::<User>(&conn);

    match user {
        Ok(v) => {
            if v.len() == 0 {
                return false;
            }
            return true;
        }
        Err(e) => {
            println!("{}", e);
            return false;
        }
    }
}

pub fn add_friend(id_pair: IdPair) -> bool {
    let my_id = id_pair.my_id;
    let friend_id = id_pair.target_id;
    // IDがレコードに存在してるかチェック
    if !is_exist_id(&my_id) || !is_exist_id(&friend_id) {
        return false;
    }

    let ids = AddFriend {
        acctive: &my_id,
        pussive: &friend_id,
    };

    let conn = db_connect::establish_connection();
    diesel::insert_into(friends::table)
        .values(&ids)
        .execute(&conn)
        .expect("挿入失敗");

    return true;
    // DBにインサート
    // bool か Result を返す
}

// fn get_friend() -> Option {}

// 正規表現をグローバルに宣言
static USER_ID: Lazy<regex::Regex> = Lazy::new(|| Regex::new(r"[0-9]{6}$").unwrap());

#[derive(Debug, Validate, Deserialize, Serialize)]
pub struct UserId {
    #[validate(regex = "USER_ID")]
    pub id: String,
}

#[derive(Debug, Validate, Deserialize, Queryable)]
struct User {
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
struct Friend {
    pub id: i32,
    pub acctive: String,
    pub passive: String,
    pub block_flag: bool,
}

use schema::friends;
#[derive(Insertable)]
#[table_name = "friends"]
pub struct AddFriend<'a> {
    pub acctive: &'a str,
    pub pussive: &'a str,
}
