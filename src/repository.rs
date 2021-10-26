// #[derive(Queryable)]
// pub struct SearchUser {
//     pub user_id: i32,
//     pub user_name: String,
//     pub icon_path: String,
//     pub applied: bool,
//     pub requested: bool,
// }

use crate::schema::friends;
#[derive(Insertable)]
#[table_name = "friends"]
pub struct AddFriend {
    pub acctive: i32,
    pub pussive: i32,
}
