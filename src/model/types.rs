use serde::{Deserialize, Serialize};
// use validator::Validate;

// 正規表現をグローバルに宣言
// static USER_ID: Lazy<regex::Regex> = Lazy::new(|| Regex::new(r"[0-9]{6}$").unwrap());

#[derive(Debug, Deserialize, Serialize)]
pub struct UserId {
    // #[validate(regex = "USER_ID")]
    pub id: i32,
}

pub enum SomeError {
    ValidationError,
    NotExistError,
    SameIdError,
}

// struct IdAndName {
//     pub user_id: i32,
//     pub target_name: String,
// }

// #[derive(Serialize)]
// pub struct FriendList {
//     pub one_side: Vec<UserView>,
//     pub mutual: Vec<UserView>,
// }

// #[derive(Validate, Serialize, Queryable)]
// pub struct UserView {
//     // #[validate(regex = "USER_ID")]
//     pub user_id: i32,
//     pub user_name: String,
//     pub status: i32,
//     pub icon_path: String,
//     pub beacon: Option<String>,
// }

pub struct Beacon {
    pub uuid: String,
    pub major: u32,
    pub minor: u32,
}

pub struct ScannedBeacon {
    // Beacon & {rssi, distance}
    pub uuid: String,
    pub major: u32,
    pub minor: u32,
    pub rssi: f32,
    pub distance: f32,
}

pub struct Scan {
    pub rssi: f32,
    pub distance: f32,
}

pub enum _ScannedBeacon {
    Beacon(Beacon),
    Scan(Scan),
}

pub struct Region {
    identifier: String,
    uuid: String,
}
