use serde::{Deserialize, Serialize};
// use validator::Validate;

#[derive(Debug, Deserialize, Serialize)]
pub struct UserId {
    pub id: i32,
}

#[derive(Debug)]
pub enum SomeError {
    ValidationError,
    NotExistError,
    InvalidPasswordError,
    SameNameError,
    InvalidStructure
}
