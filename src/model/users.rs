use crate::repository::NameAndPassword;

use super::db_util::{insert_user, update_spot};

pub fn create_user(name_and_password: NameAndPassword) {
    let name = name_and_password.user_name.to_string();
    let raw_password = name_and_password.hashed_password.to_string();
    // TODO: hash with secret salt
    let hashed_password = raw_password;
    insert_user(name, hashed_password);
}

pub fn udpate_beacon(user_id: i32, major_id: i32, minor_id: i32) {
    // is_exist_beacon(major_id, minor_id)
    update_spot(user_id, major_id, minor_id);
}
