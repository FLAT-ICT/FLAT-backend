use super::{
    db_util::{delete_loggedin_at, get_loggedin_at_from_name, is_exist_name},
    types::SomeError,
};
use crate::{
    model::db_util,
    repository::UserHashedCredential,
    view::{IsOtherUserLoggedIn, PreLoginView, UserCredential, UserTimestamp, UserView},
};
use chrono::NaiveDateTime;
use db_util::{get_loggedin_at, get_secret, insert_user, update_spot};
use ring::pbkdf2;
use std::num::NonZeroU32;

pub fn create_user(credential: UserHashedCredential) -> Result<UserView, SomeError> {
    if let true = is_exist_name(&credential.name) {
        return Err(SomeError::SameNameError);
    }
    let result = insert_user(credential);

    if let Some(v) = result {
        return Ok(v);
    } else {
        return Err(SomeError::SameNameError);
    };
}

pub fn is_logged_in(user_timestamp: UserTimestamp) -> IsOtherUserLoggedIn {
    // あとでリファクタする
    match &user_timestamp {
        UserTimestamp::I(uit) => {
            if let Some(last_login_timestamp) = get_loggedin_at(&user_timestamp) {
                if last_login_timestamp == uit.logged_in_at {
                    return IsOtherUserLoggedIn { others: false };
                }
                return IsOtherUserLoggedIn { others: true };
            };
        }
        UserTimestamp::N(unt) => {
            if let Some(last_login_timestamp) = get_loggedin_at(&user_timestamp) {
                if last_login_timestamp == unt.logged_in_at {
                    return IsOtherUserLoggedIn { others: false };
                }
                return IsOtherUserLoggedIn { others: true };
            };
        }
    }
    return IsOtherUserLoggedIn { others: false };
}

pub fn pre_login(p: &PreLoginView) -> Result<Option<NaiveDateTime>, SomeError> {
    if let false = is_exist_name(&p.name) {
        return Err(SomeError::NotExistError);
    }

    if p.password.is_none() {
        return Err(SomeError::NotExistError);
    }
    if let false = match_password(&UserCredential {
        name: p.name.to_string(),
        password: p.password.as_ref().unwrap().to_string(),
    }) {
        return Err(SomeError::InvalidPasswordError);
    } else {
        return Ok(get_loggedin_at_from_name(p.name.to_string()));
    }
    // Ok(())
}

pub fn login(credential: UserCredential) -> Result<UserView, SomeError> {
    // validation
    // let c = &credential.to_hash();
    // パスワードチェック

    if let false = is_exist_name(&credential.name) {
        return Err(SomeError::NotExistError);
    }

    if let false = match_password(&credential) {
        return Err(SomeError::InvalidPasswordError);
    }
    let result = db_util::login(&credential.name);
    return Ok(result);
}

pub fn match_password(credential: &UserCredential) -> bool {
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

pub fn logout(user_id: i32) -> Result<(), SomeError> {
    delete_loggedin_at(user_id)
}

pub fn update_beacon(user_id: i32, major_id: i32, minor_id: i32) -> bool {
    // is_exist_beacon(major_id, minor_id)
    update_spot(user_id, major_id, minor_id)
}

pub fn update_name(user_id: i32, name: String) -> Result<(), SomeError>{
    if let true = is_exist_name(&name){
        return Err(SomeError::SameNameError);
    }
    // if let false = validate_name(){}
    println!("{}", name);
    if let Ok(result) = db_util::update_name(user_id, name){
        println!("{:?}", result);
        return Ok(())
    }else{
        return Err(SomeError::SameNameError);
    }
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
        )
        .unwrap();
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
        )
        .unwrap();
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
        )
        .unwrap();
        assert_eq!(false, update_beacon(uv.id, 0, 0));
    }
}
