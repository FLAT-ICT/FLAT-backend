// use diesel::serialize::Result;

use super::{
    db_util::{get_logedin_at, insert_user, update_spot},
    types::SomeError,
};
use crate::{
    repository::UserHashedCredential,
    view::{IsLogedIn, UserCredential, UserTimestamp, UserView},
};

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
    if let Some(last_login_timestamp) = get_logedin_at(&user_timestamp) {
        if last_login_timestamp == user_timestamp.logedin_at {
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
    let c = &credential.to_hash();
    // パスワードチェック
    // let result = db_util::login(c);
    // return result;
    todo!();
    // Ok(())
}

fn password_validator(password: String) -> String {
    todo!("8文字以上256文字以下, 英数字のみ")
}

fn match_password(password: String) -> bool {
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
