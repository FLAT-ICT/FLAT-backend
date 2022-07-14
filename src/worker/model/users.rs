use super::{
    db_util::{
        delete_loggedin_at, get_loggedin_at_from_name, get_user_view, is_exist_name,
        update_icon_url,
    },
    types::SomeError,
};
use crate::worker::utils::save_cloud_storage::{create_client, upload_image};
use crate::worker::{
    model::db_util,
    repository::UserHashedCredential,
    utils::image_crop::base64_to_image,
    view::{IsOtherUserLoggedIn, PreLoginView, UserCredential, UserTimestamp, UserView},
};
use chrono::NaiveDateTime;
use db_util::{get_loggedin_at, get_secret, insert_user, update_spot};
use ring::pbkdf2;
use std::num::NonZeroU32;

pub fn create_user(credential: UserHashedCredential) -> Result<UserView, SomeError> {
    if let true = is_exist_name(&credential.name) {
        return Err(SomeError::AlreadyExistName);
    }
    let result = insert_user(credential);

    if let Some(v) = result {
        return Ok(v);
    } else {
        return Err(SomeError::AlreadyExistName);
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
        return Err(SomeError::NotExist);
    }

    if p.password.is_none() {
        return Err(SomeError::NotExist);
    }
    if let false = match_password(&UserCredential {
        name: p.name.to_string(),
        password: p.password.as_ref().unwrap().to_string(),
    }) {
        return Err(SomeError::InvalidPassword);
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
        return Err(SomeError::NotExist);
    }

    if let false = match_password(&credential) {
        return Err(SomeError::InvalidPassword);
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

pub fn update_name(user_id: i32, name: String) -> Result<UserView, SomeError> {
    // 自分と同じ名前は許容
    // 名前が一致している かつ IDが一致している 場合 Ok
    // 名前が一致している かつ IDが一致していない 場合 Err
    if let Ok(old_user) = get_user_view(user_id) {
        if old_user.name == name {
            match old_user.id == user_id {
                true => return Ok(old_user),
                false => return Err(SomeError::AlreadyExistName),
            };
        } else {
            match db_util::update_name(user_id, name) {
                Ok(user) => return Ok(user),
                Err(e) => return Err(e),
            }
        }
    } else {
        return Err(SomeError::NotExist);
    }
}

pub fn update_status(user_id: i32, status: i32) -> Result<UserView, diesel::result::Error> {
    db_util::update_status(user_id, status)
}

#[cfg(test)]
mod tests {
    use crate::worker::{model::users::update_beacon, view::UserCredential};

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

pub async fn update_icon(user_id: i32, icon_string: String) -> Result<UserView, SomeError> {
    let client = create_client();
    let image_name = format!("{}.png", user_id);
    let image = base64_to_image(&icon_string);

    match upload_image(&client, &image_name, image).await {
        Ok(r) => {
            let url = r.media_link.as_str();
            match update_icon_url(user_id, url) {
                Ok(r) => Ok(r),
                Err(e) => {
                    println!("{}", e);
                    Err(SomeError::DontReach)
                }
            }
        }
        Err(e) => {
            println!("{}", e);
            Err(SomeError::UploadImageError)
        }
    }

    // if let Err(_) = upload_image(user_id, icon).await{
    //     return Err(SomeError::NotExist);
    // }else {
    //     return Ok(get_user_view(user_id).unwrap());
    // }
}
