use once_cell::sync::Lazy;
use regex::Regex;
use serde::{Deserialize, Serialize};

use validator::{Validate};

#[derive(Serialize)]
pub struct Friend {}

// 正規表現をグローバルに宣言
static USER_ID: Lazy<regex::Regex> = Lazy::new(|| Regex::new(r"[A-Z0-9]{6}$").unwrap());

#[derive(Debug, Validate, Deserialize, Serialize)]
pub struct IdPair {
    // String replace UserId
    #[validate(regex = "USER_ID")]
    pub my_id: String,
    #[validate(regex = "USER_ID")]
    pub target_id: String,
}

#[derive(Serialize)]
pub struct ResultMessage {
    pub message: String,
}

// the input to our `create_user` handler
#[derive(Deserialize)]
pub struct CreateUser {
    pub username: String,
}

// the output to our `create_user` handler
#[derive(Validate, Serialize)]
pub struct User {
    #[validate(regex = "USER_ID")]
    pub id: String,
    pub user_name: String,
    pub status: u8,
    pub icon_path: String,
    pub beacon: String,
}
