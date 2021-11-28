// use diesel::serialize::Result;

use std::num::NonZeroU32;

use data_encoding::HEXUPPER;
use hyper::StatusCode;
use ring::pbkdf2;

use super::types::SomeError;
use crate::{
    model::db_util,
    repository::UserHashedCredential,
    view::{IsLogedIn, UserCredential, UserTimestamp, UserView},
};
use db_util::{get_loggedin_at, get_secret, insert_user, update_spot};

pub fn create_user(credential: UserHashedCredential) -> UserView {
    // println!("{:#?}", create_usr2.json::<UserView>().await.unwrap());
    // let name = name_and_password.name.to_string();
    // let raw_password = name_and_password.password.to_string();
    // TODO: hash with secret salt
    // let hashed_password = raw_password;
    // let result = insert_user(name, hashed_password);
    let result = insert_user(credential);

    result
}

pub fn is_loged_in(user_timestamp: UserTimestamp) -> IsLogedIn {
    if let Some(last_login_timestamp) = get_loggedin_at(&user_timestamp) {
        if last_login_timestamp == user_timestamp.loggedin_at {
            return IsLogedIn {
                own: true,
                others: false,
            };
        }
        return IsLogedIn {
            own: true,
            others: false,
        };
    };
    return IsLogedIn {
        own: false,
        others: false,
    };
}

pub fn login(credential: UserCredential) -> Result<UserView, SomeError> {
    // validation
    // let c = &credential.to_hash();
    // パスワードチェック
    if let false = match_password(&credential) {
        return Err(SomeError::InvalidPasswordError);
    }
    let result = db_util::login(&credential.name);
    return Ok(result);
}

fn validate_password(password: String) -> bool {
    todo!("8文字以上256文字以下, 英数字のみ")
}

fn match_password(credential: &UserCredential) -> bool {
    // let salt, hash = get_credential(id)
    //
    let s = get_secret(&credential.name);
    if let Ok(_) = pbkdf2::verify(
        pbkdf2::PBKDF2_HMAC_SHA512,
        NonZeroU32::new(1).unwrap(),
        &s.salt,
        &credential.password.as_bytes(),
        &s.hash,
    ) {
        return true;
    };
    false
}

pub fn update_beacon(user_id: i32, major_id: i32, minor_id: i32) -> bool {
    // is_exist_beacon(major_id, minor_id)
    update_spot(user_id, major_id, minor_id)
}

#[cfg(test)]
mod tests {
    use crate::{model::users::update_beacon, view::UserCredential};

    use super::create_user;

    #[test]
    fn exist_spot() {
        let uv = create_user(
            UserCredential {
                name: "spot_test1".to_string(),
                password: "pass".to_string(),
            }
            .to_hash(),
        );
        assert!(update_beacon(uv.id, 0, 7945));
    }
    #[test]
    fn did_exit_region() {
        let uv = create_user(
            UserCredential {
                name: "spot_test2".to_string(),
                password: "pass".to_string(),
            }
            .to_hash(),
        );
        assert!(update_beacon(uv.id, 0, -1));
    }
    #[test]
    fn not_exist_spot() {
        let uv = create_user(
            UserCredential {
                name: "spot_test3".to_string(),
                password: "pass".to_string(),
            }
            .to_hash(),
        );
        assert_eq!(false, update_beacon(uv.id, 0, 0));
    }
}
