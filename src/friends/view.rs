use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct Friend {}

#[derive(Deserialize)]
pub struct IdPair {
    // String replace UserId
    pub my_id: String,
    pub friend_id: String,
}
