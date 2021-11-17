use crate::schema::friends;
use crate::schema::spots;
use crate::schema::users;
use chrono::DateTime;
use chrono::NaiveDateTime;
use chrono::Utc;
use serde::Deserialize;

#[derive(Insertable)]
#[table_name = "friends"]
pub struct AddFriend {
    pub active: i32,
    pub passive: i32,
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
    pub id: i32,
    pub name: String,
    pub status: i32,
    pub spot: Option<String>,
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
    pub name: &'a String,
    pub hashed_password: &'a String,
}
#[derive(Debug, Insertable)]
#[table_name = "spots"]
pub struct InsertableSpot {
    pub name_ja: String,
    pub name_en: String,
    pub region_identifier: String,
    pub available_term_from: NaiveDateTime,
    pub available_term_to: Option<NaiveDateTime>,
    pub major: i32,
    pub minor: i32,
    pub note: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct DeserializableSpot {
    pub name_ja: String,
    pub name_en: String,
    pub region_identifier: String,
    pub available_term_from: DateTime<Utc>,
    pub available_term_to: Option<DateTime<Utc>>,
    pub major: i32,
    pub minor: i32,
    pub note: Option<String>,
}

impl DeserializableSpot {
    pub fn to_insertable(self) -> InsertableSpot {
        InsertableSpot {
            name_ja: self.name_ja,
            name_en: self.name_en,
            region_identifier: self.region_identifier,
            available_term_from: self.available_term_from.naive_utc(),
            available_term_to: self.available_term_to.map(|x| x.naive_local()),
            major: self.major,
            minor: self.minor,
            note: self.note,
        }
    }
}
