use crate::{repository::NameAndPassword, view::UserView};

use super::db_util::{insert_user, update_spot};

pub fn create_user(name_and_password: NameAndPassword) -> UserView {
    // println!("{:#?}", create_usr2.json::<UserView>().await.unwrap());
    let name = name_and_password.name.to_string();
    let raw_password = name_and_password.hashed_password.to_string();
    // TODO: hash with secret salt
    let hashed_password = raw_password;
    let result = insert_user(name, hashed_password);

    // ここだけやるとバグる
    // 中の型的には nullable だけど、返却するときは、"" が返ってほしい。
    // match result.spot {
    //     Some(_) => {}
    //     _ => (result.spot = Some("".to_string())),
    // }

    result
}

pub fn update_beacon(user_id: i32, major_id: i32, minor_id: i32) -> bool {
    // is_exist_beacon(major_id, minor_id)
    update_spot(user_id, major_id, minor_id)
}

#[cfg(test)]
mod tests {
    use crate::{model::users::update_beacon, repository::NameAndPassword};

    use super::create_user;

    #[test]
    fn exist_spot() {
        let uv = create_user(NameAndPassword {
            name: &"spot_test1".to_string(),
            hashed_password: &"".to_string(),
        });
        assert!(update_beacon(uv.id, 0, 7945));
    }
    #[test]
    fn did_exit_region() {
        let uv = create_user(NameAndPassword {
            name: &"spot_test2".to_string(),
            hashed_password: &"".to_string(),
        });
        assert!(update_beacon(uv.id, 0, -1));
    }
    #[test]
    fn not_exist_spot() {
        let uv = create_user(NameAndPassword {
            name: &"spot_test3".to_string(),
            hashed_password: &"".to_string(),
        });
        assert_eq!(false, update_beacon(uv.id, 0, 0));
    }
}
