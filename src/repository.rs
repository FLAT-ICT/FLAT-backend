use crate::schema::friends;
use crate::schema::users;
use chrono::NaiveDateTime;

#[derive(Insertable)]
#[table_name = "friends"]
pub struct AddFriend {
    pub acctive: i32,
    pub pussive: i32,
}

#[derive(Debug, Queryable)]
pub struct Friend {
    pub acctive: i32,
    pub passive: i32,
    pub created_at: NaiveDateTime,
    pub blocked_at: Option<NaiveDateTime>,
}

#[derive(Debug, Queryable)]
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

#[derive(Queryable)]
pub struct IdNamePath {
    pub id: i32,
    pub name: String,
    pub icon_path: String,
}

#[derive(Insertable)]
#[table_name = "users"]
pub struct NameAndPassword<'a> {
    pub user_name: &'a String,
    pub hashed_password: &'a String,
}
