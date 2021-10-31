use super::db_util::update_spot;

fn creaate_user() {}

pub fn udpate_beacon(user_id: i32, major_id: i32, minor_id: i32) {
    // is_exist_beacon(major_id, minor_id)
    update_spot(user_id, major_id, minor_id);
}
