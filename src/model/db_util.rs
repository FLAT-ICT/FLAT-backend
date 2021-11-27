use crate::repository::AddFriend;
use crate::repository::Friend;
use crate::repository::IdNamePath;
use crate::repository::InsertableSpot;
use crate::repository::User;
use crate::repository::UserHashedCredential;
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
    let user = users.filter(id.eq(target_id)).load::<User>(&conn);

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

pub fn insert_user(hashed_credential: UserHashedCredential) -> UserView {
    let now = chrono::offset::Utc::now().naive_utc();
    let conn = establish_connection();
    let _inserted_row = diesel::insert_into(users)
        .values((
            name.eq(hashed_credential.name),
            salt.eq(hashed_credential.salt),
            hash.eq(hashed_credential.hash),
            icon_path.eq(&"https://dummyimage.com/256x256/000/fff.png&text=icon".to_string()),
            logedin_at.eq(now),
        ))
        .execute(&conn)
        .unwrap();

    let last_insert_user = users.order(id.desc()).first::<User>(&conn).unwrap();

    let user_view = UserView {
        id: last_insert_user.id,
        name: last_insert_user.name.to_string(),
        status: last_insert_user.status,
        icon_path: last_insert_user.icon_path,
        spot: last_insert_user.spot,
        logined_at: last_insert_user.logedin_at,
    };
    return user_view;
}

pub fn get_user_id_name_path(target_name: String) -> Vec<IdNamePath> {
    let conn = establish_connection();
    let result = users
        .filter(name.like("%".to_string() + &target_name + "%"))
        .select((id, name, icon_path))
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
            .filter(active.eq(id1))
            .filter(passive.eq(id2))
            .load::<Friend>(conn)
            .unwrap()
    }
    let conn = establish_connection();
    let applied = is_exist_record(get_friend_relation(&conn, my_id, target_id));
    let requested = is_exist_record(get_friend_relation(&conn, target_id, my_id));

    (applied, requested)
}

use schema::{friends, users};

joinable!(friends -> users(active));

pub fn get_requested_record(my_id: i32) -> Vec<UserView> {
    let conn = establish_connection();
    let reqested = friends
        .inner_join(users)
        .filter(friends::passive.eq(my_id))
        .select((
            friends::active,
            users::name,
            users::status,
            users::icon_path,
            users::spot,
            users::logedin_at
        ))
        .load::<UserView>(&conn)
        .unwrap();
    return reqested;
}
// allow_tables_to_appear_in_same_query!(friends, users);

pub fn get_applied_record(my_id: i32) -> Vec<i32> {
    let conn = establish_connection();
    let applied = friends
        .filter(active.eq(my_id))
        .select(passive)
        .load::<i32>(&conn)
        .unwrap();
    return applied;
}

pub fn insert_friend(ids: AddFriend) -> Result<usize, diesel::result::Error> {
    let conn = establish_connection();
    diesel::insert_into(friends::table)
        .values(&ids)
        .execute(&conn)
}

pub fn delete_friend(ids: AddFriend) -> Result<usize, diesel::result::Error> {
    let conn = establish_connection();
    diesel::delete(
        friends
            .filter(active.eq(&ids.active))
            .filter(passive.eq(&ids.passive)),
    )
    .execute(&conn)
}

pub fn insert_spots_from_csv(school_spot: InsertableSpot) -> Result<usize, diesel::result::Error> {
    let conn = establish_connection();
    diesel::insert_into(spots)
        .values(&school_spot)
        .execute(&conn)
}

pub fn update_spot(my_id: i32, major_id: i32, minor_id: i32) -> bool {
    let conn = establish_connection();

    if let Some(s) = get_spot(&conn, major_id, minor_id) {
        match diesel::update(users.find(&my_id))
            .set(spot.eq(s))
            .execute(&conn)
        {
            Ok(v) => {
                println!("{}", v);
                return true;
            }
            Err(_) => return false,
        }
    }
    if minor_id == -1 {
        match diesel::update(users.find(&my_id))
            .set(spot.eq::<Option<String>>(None))
            .execute(&conn)
        {
            Ok(v) => {
                println!("{}", v);
                return true;
            }
            Err(_) => return false,
        }
    }
    return false;

    pub fn get_spot(conn: &MysqlConnection, major_id: i32, minor_id: i32) -> Option<String> {
        // let conn = establish_connection();
        let result = spots
            .filter(major.eq(&major_id))
            .filter(minor.eq(&minor_id))
            .select(name_ja)
            .first::<String>(conn);
        if let Ok(v) = result {
            return Some(v);
        }
        return None;
    }
}
