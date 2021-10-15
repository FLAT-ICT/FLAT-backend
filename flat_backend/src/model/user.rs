fn creaate_user() {}

#[derive(Debug, Validate, Deserialize, Queryable)]
struct User {
    pub id: i32,
    pub user_id: String,
    pub user_name: String,
    pub status: i32,
    pub beacon: Option<String>,
    pub icon_path: String,
    pub hashed_password: String,
}

struct IdAndName {
    pub user_id: String,
    pub user_name: String,
}
