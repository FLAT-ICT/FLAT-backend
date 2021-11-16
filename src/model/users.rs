use crate::{repository::NameAndPassword, view::UserView};

use super::db_util::{insert_user, update_spot};

pub fn create_user(name_and_password: NameAndPassword) -> UserView {
    // println!("{:#?}", create_usr2.json::<UserView>().await.unwrap());
    let name = name_and_password.name.to_string();
    let raw_password = name_and_password.hashed_password.to_string();
    // TODO: hash with secret salt
    let hashed_password = raw_password;
    let mut result = insert_user(name, hashed_password);

    // 中の型的には nullable だけど、返却するときは、"" が返ってほしい。
    match result.spot {
        Some(_) => {}
        _ => (result.spot = Some("".to_string())),
    }

    result
}

pub fn udpate_beacon(user_id: i32, major_id: i32, minor_id: i32) {
    // is_exist_beacon(major_id, minor_id)
    update_spot(user_id, major_id, minor_id);
}
