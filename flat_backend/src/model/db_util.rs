use crate::model::friends::Friend;
use crate::model::friends::User;

use super::super::schema;
use axum::http::request;
use diesel::mysql::MysqlConnection;
use diesel::prelude::*;
use diesel::serialize::Result;
use diesel::ExpressionMethods;
use diesel::QueryDsl;
use dotenv::dotenv;
use schema::friends::dsl::*;
use schema::users::dsl::*;
use std::env;

pub fn establish_connection() -> MysqlConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    MysqlConnection::establish(&database_url)
        .expect(&format!("Error connetincg to {}", database_url))
}

pub fn is_exist_id(target_id: &str) -> bool {
    // let user_id = id;
    // if id.len() != 6 {
    //     return false;
    // }

    println!("is_exist_id({})", target_id);

    let conn = establish_connection();
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

pub fn get_user_id_name_path(target_id: &String) -> (String, String, String) {
    let conn = establish_connection();
    let result = users
        .filter(user_id.eq(target_id))
        .select((user_id, user_name, icon_path))
        .first::<(String, String, String)>(&conn)
        .unwrap();
    result
}

pub fn get_friends_relation(my_id: &String, target_id: &String) -> (bool, bool) {
    // レコードがあるかどうか
    fn is_exist_record(arg: Vec<Friend>) -> bool {
        if let Some(_) = arg.get(0) {
            true
        } else {
            false
        }
    }
    // レコードをとってくる
    fn get_friend_relation(conn: &MysqlConnection, id1: &String, id2: &String) -> Vec<Friend> {
        friends
            .filter(acctive.eq(id1))
            .filter(pussive.eq(id2))
            .load::<Friend>(conn)
            .unwrap()
    }
    let conn = establish_connection();
    let applied = is_exist_record(get_friend_relation(&conn, &my_id, &target_id));
    let requested = is_exist_record(get_friend_relation(&conn, &target_id, &my_id));

    (applied, requested)
}
