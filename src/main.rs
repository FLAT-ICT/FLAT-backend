#[macro_use]
extern crate diesel;

use axum::{
    handler::{get, post},
    Router,
};
use std::net::SocketAddr;
use tracing;
use tracing_subscriber;
mod controller;
mod model;
mod view;
use controller::friends::{add_friend, check_friend_status, friend_list, reject_friend};
use controller::users::create_user;
use controller::users::update_beacon;
mod read_csv_and_write_db;
mod repository;
mod schema;

#[tokio::main]
async fn main() {
    if let Err(err) = read_csv_and_write_db::run() {
        println!("{}", err);
        // process::exit(1);
    }

    // トレーサーを初期化
    tracing_subscriber::fmt::init();

    // ルーターを作成
    let app = Router::new()
        // `GET /` goes to `root`
        .route("/", get(root))
        // `POST /users` goes to `create_user`
        .route("/v1/register", post(create_user))
        .route("/v1/users", post(create_user))
        .route("/v1/users/search", get(check_friend_status))
        .route("/v1/users/beacon", post(update_beacon))
        .route("/v1/friends", get(friend_list))
        .route("/v1/friends/add", post(add_friend))
        .route("/v1/friends/reject", post(reject_friend));

    // バインドするアドレス
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));

    tracing::debug!("listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

// basic handler that responds with a static string
async fn root() -> &'static str {
    "Hello, World!"
}

#[cfg(test)]
mod tests {
    use axum::http;

    #[tokio::test]
    async fn my_test() {
        assert!(true);
    }

    #[tokio::test]
    async fn get_root() {
        let client = reqwest::Client::new();
        let res = client.get("http://localhost:3000").send().await.unwrap();
        assert_eq!(res.status(), http::StatusCode::OK);
    }
}

#[cfg(test)]
mod search_user {

    use crate::repository::{Friend, User};
    use crate::schema::friends::dsl::*;
    use crate::schema::users::dsl::*;
    use crate::view::UserView;
    use crate::{
        model::db_util::establish_connection,
        view::{CreateUser, IdPair},
    };
    use axum::http;
    use diesel::RunQueryDsl;

    #[tokio::test]
    async fn basic() {
        // usr1作成
        // usr2作成
        // usr1 -> usr2 に友だち申請
        // search_user
        // レコード初期化
        let base_url = "http://localhost:3000";
        let client = reqwest::Client::new();
        let create_usr1 = client
            .post(base_url.to_string() + "/v1/register")
            .json(&CreateUser {
                user_name: "usr1".to_string(),
                password: "".to_string(),
            })
            .send()
            .await
            .unwrap();
        assert_eq!(create_usr1.status(), http::StatusCode::OK);

        let create_usr2 = client
            .post(base_url.to_string() + "/v1/register")
            .json(&CreateUser {
                user_name: "usr2".to_string(),
                password: "".to_string(),
            })
            .send()
            .await
            .unwrap();
        assert_eq!(create_usr2.status(), http::StatusCode::OK);

        let id_1 = create_usr1.json::<UserView>().await.unwrap().user_id;
        let id_2 = create_usr2.json::<UserView>().await.unwrap().user_id;
        // println!("{:#?}", create_usr1.json::<UserView>().await.unwrap());
        // println!("{:#?}", create_usr2.json::<UserView>().await.unwrap());
        // println!("{}", create_usr1.text().await.unwrap());

        let friend_request = client
            .post(base_url.to_string() + "/v1/friends/add")
            .json(&IdPair {
                my_id: id_1,
                target_id: id_2,
            })
            .send()
            .await
            .unwrap();
        assert_eq!(friend_request.status(), http::StatusCode::OK);

        let conn = establish_connection();
        let result = users.load::<User>(&conn).unwrap();
        println!("{:#?}", result);
        let result = friends.load::<Friend>(&conn).unwrap();
        println!("{:#?}", result);

        let _get_friend_list = client
            .get(
                base_url.to_string()
                    + "/v1/users/search?user_id="
                    + &id_1.to_string()
                    + "&target_name=usr2",
            )
            .send()
            .await
            .unwrap();

        println!("{:#?}", _get_friend_list);
        assert_eq!(_get_friend_list.status(), http::StatusCode::OK);
        // DBをきれいにする
        // diesel::delete(users).execute(&conn).unwrap();
        // println!("delete from basic")
        // assert_eq!(0, get_count());
        // let count = diesel::delete(friends).execute(&conn).unwrap();
        // assert_eq!(1, count);
    }
}

#[cfg(test)]
mod beacon {
    // use crate::model::db_util::establish_connection;
    // use crate::schema::users::dsl::*;
    use crate::view::{CreateUser, ScannedBeacon, UserView};
    use axum::http;
    // use diesel::RunQueryDsl;

    #[tokio::test]
    async fn fn1() {
        // ユーザー登録
        // ビーコンをアップデート
        // ユーザー情報を返却
        let base_url = "http://localhost:3000";
        let client = reqwest::Client::new();
        match client
            .post(base_url.to_string() + "/v1/users")
            .json(&CreateUser {
                user_name: "usr1".to_string(),
                password: "".to_string(),
            })
            .send()
            .await
        {
            Ok(v) => {
                assert_eq!(v.status(), http::StatusCode::OK);
            }
            Err(e) => println!("{:?}", e),
        }

        match client
            .post(base_url.to_string() + "/v1/users/beacon")
            .json(&ScannedBeacon {
                user_id: 1,
                uuid: "9717f39c-a676-46ff-90c7-2d27a4d2477f".to_string(),
                major: 0,
                minor: 43303,
                rssi: 0.,
                distance: 1.0,
            })
            .send()
            .await
        {
            Ok(v) => {
                assert_eq!(v.status(), http::StatusCode::OK);
                println!("update spot");
            }
            Err(e) => println!("{:?}", e),
        };
        // .unwrap();

        // let conn = establish_connection();
        match client
            .get(base_url.to_string() + "/v1/users?user_id=1")
            .send()
            .await
        {
            Ok(v) => match v.json::<UserView>().await {
                Ok(user_info) => {
                    println!("{:#?}", &user_info);
                    assert_eq!((&user_info).beacon, Some("127教員室".to_string()));
                }
                Err(e) => println!("{:?}", e),
            },
            Err(e) => println!("{:?}", e),
        }

        // DBをきれいにする
        // diesel::delete(users).execute(&conn).unwrap();
    }
}
