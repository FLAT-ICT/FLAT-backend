use crate::schema::friends;
use crate::schema::spots;
use crate::schema::users;
use crate::view::UserCredential;
use chrono::DateTime;
use chrono::NaiveDateTime;
use chrono::Utc;
use data_encoding::HEXUPPER;
use ring::error::Unspecified;
use ring::rand::SecureRandom;
use ring::{digest, pbkdf2, rand};
use serde::Deserialize;
use std::num::NonZeroU32;

#[derive(Insertable)]
#[table_name = "friends"]
pub struct AddFriend {
    pub active: i32,
    pub passive: i32,
}

#[derive(Debug, Queryable)]
pub struct Friend {
    pub active: i32,
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
    pub salt: String,
    pub hash: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub logedin_at: Option<NaiveDateTime>,
}

#[derive(Queryable)]
pub struct IdNamePath {
    pub id: i32,
    pub name: String,
    pub icon_path: String,
}

#[derive(Insertable)]
#[table_name = "users"]
pub struct UserHashedCredential {
    pub name: String,
    pub salt: String,
    pub hash: String,
}
 
impl UserCredential {
    pub fn to_hash(&self) -> UserHashedCredential {

        fn convert(bytes: &[u8]) -> String {
            return bytes.iter().map(|&s| s as char).collect::<String>();
        }
        
        const CREDENTIAL_LEN: usize = digest::SHA512_OUTPUT_LEN;
        // let n_iter = NonZeroU32::new(15_000).unwrap();
        let n_iter = NonZeroU32::new(1).unwrap();
        let rng = rand::SystemRandom::new();

        let mut salt = [0u8; CREDENTIAL_LEN];
        rng.fill(&mut salt).unwrap();

        let mut pbkdf2_hash = [0u8; CREDENTIAL_LEN];
        pbkdf2::derive(
            pbkdf2::PBKDF2_HMAC_SHA512,
            n_iter,
            &salt,
            &self.password.as_bytes(),
            &mut pbkdf2_hash,
        );

        let result = UserHashedCredential {
            name: self.name.to_owned(),
            salt: convert(&salt),
            hash: convert(&pbkdf2_hash),
        };
        result
    }
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
