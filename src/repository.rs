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
pub struct AddFriend<'a> {
    pub acctive: &'a i32,
    pub pussive: &'a i32,
}
