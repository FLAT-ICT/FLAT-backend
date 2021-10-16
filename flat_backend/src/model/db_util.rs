use crate::model::friends::User;

use super::super::schema;
use diesel::mysql::MysqlConnection;
use diesel::prelude::*;
use diesel::ExpressionMethods;
use diesel::QueryDsl;
use dotenv::dotenv;
use std::env;

pub fn establish_connection() -> MysqlConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    MysqlConnection::establish(&database_url)
        .expect(&format!("Error connetincg to {}", database_url))
}

pub fn is_exist_id(target_id: &str) -> bool {
    // let user_id = id;
    // if id.len() != 6 {
    //     return false;
    // }

    println!("is_exist_id({})", target_id);

    use schema::users::dsl::*;

    let conn = establish_connection();
    let user = users.filter(user_id.eq(target_id)).load::<User>(&conn);

    match user {
        Ok(v) => {
            if v.len() == 0 {
                return false;
            }
            return true;
        }
        Err(e) => {
            println!("{}", e);
            return false;
        }
    }
}
