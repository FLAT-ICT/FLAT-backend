// use crate::model::types::User;
use crate::repository::AddFriend;
use crate::repository::Friend;
use crate::repository::IdNamePath;
use crate::repository::NameAndPassword;
use crate::repository::InsertableSpot;
use crate::repository::User;
use crate::schema;
use crate::view::UserView;
use diesel::mysql::MysqlConnection;
use diesel::prelude::*;
use diesel::ExpressionMethods;
use diesel::QueryDsl;
use dotenv::dotenv;
use schema::friends::dsl::*;
use schema::spots::dsl::*;
use schema::users::dsl::*;
use std::env;

pub fn establish_connection() -> MysqlConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    MysqlConnection::establish(&database_url)
        .expect(&format!("Error connetincg to {}", database_url))
}

pub fn is_exist_id(target_id: i32) -> bool {
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

pub fn insert_user(name_and_pass: NameAndPassword) -> UserView {
    let name = name_and_pass.user_name;
    let pass = name_and_pass.hashed_password;

    let conn = establish_connection();
    let inserted_row = diesel::insert_into(users)
        .values((user_name.eq(name), hashed_password.eq(pass)))
        .execute(&conn)
        .unwrap();

    let last_insert_user = users.order(user_id.desc()).first::<User>(&conn).unwrap();

    let user_view = UserView {
        user_id: last_insert_user.user_id,
        user_name: last_insert_user.user_name.to_string(),
        status: last_insert_user.status,
        icon_path: last_insert_user.icon_path,
        beacon: last_insert_user.beacon,
    };
    return user_view;
}

pub fn get_user_id_name_path(target_name: String) -> Vec<IdNamePath> {
    let conn = establish_connection();
    let result = users
        .filter(user_name.like(target_name))
        .select((user_id, user_name, icon_path))
        .load::<IdNamePath>(&conn)
        .unwrap();
    // .first::<(i32, String, String)>(&conn)
    // .unwrap();
    result
}

pub fn get_friends_relation(my_id: i32, target_id: i32) -> (bool, bool) {
    // レコードがあるかどうか
    fn is_exist_record(arg: Vec<Friend>) -> bool {
        if let Some(_) = arg.get(0) {
            true
        } else {
            false
        }
    }
    // レコードをとってくる
    fn get_friend_relation(conn: &MysqlConnection, id1: i32, id2: i32) -> Vec<Friend> {
        friends
            .filter(acctive.eq(id1))
            .filter(pussive.eq(id2))
            .load::<Friend>(conn)
            .unwrap()
    }
    let conn = establish_connection();
    let applied = is_exist_record(get_friend_relation(&conn, my_id, target_id));
    let requested = is_exist_record(get_friend_relation(&conn, target_id, my_id));

    (applied, requested)
}

use schema::{friends, users};

use super::types::Beacon;

joinable!(friends -> users(acctive));

pub fn get_applied_record(my_id: i32) -> Vec<UserView> {
    let conn = establish_connection();
    let applied = friends
        .inner_join(users)
        .filter(friends::acctive.eq(my_id))
        .select((
            users::user_id,
            users::user_name,
            users::status,
            users::icon_path,
            users::beacon,
        ))
        .load::<UserView>(&conn)
        .unwrap();
    return applied;
}
// allow_tables_to_appear_in_same_query!(friends, users);

pub fn get_requested_record(my_id: i32) -> Vec<i32> {
    let conn = establish_connection();
    let applied = friends
        .filter(pussive.eq(my_id))
        .select(acctive)
        .load::<i32>(&conn)
        .unwrap();
    return applied;
}

pub fn insert_friend(ids: AddFriend) {
    let conn = establish_connection();
    diesel::insert_into(friends::table)
        .values(&ids)
        .execute(&conn)
        .expect("挿入失敗");
    println!("insert_friend")
}

pub fn delete_friend(ids: AddFriend) {
    let conn = establish_connection();
    diesel::delete(
        friends
            .filter(acctive.eq(&ids.acctive))
            .filter(pussive.eq(&ids.pussive)),
    )
    .execute(&conn)
    .expect("削除失敗");
}

pub fn insert_spots_from_csv(spot: InsertableSpot) {
    let conn = establish_connection();
    diesel::insert_into(spots)
        .values(&spot)
        .execute(&conn)
        .expect("挿入失敗");
}

pub fn update_spot(my_id: i32, major_id: i32, minor_id: i32) {
    let conn = establish_connection();

    diesel::update(users.find(&my_id))
        .set(beacon.eq(get_spot(&conn, major_id, minor_id)))
        .execute(&conn)
        .expect("更新失敗");

    pub fn get_spot(conn: &MysqlConnection, major_id: i32, minor_id: i32) -> String {
        // let conn = establish_connection();
        let result = spots
            .filter(major.eq(&major_id))
            .filter(minor.eq(&minor_id))
            .select(name_ja)
            .first::<String>(conn)
            .unwrap();
        return result;
    }
}
