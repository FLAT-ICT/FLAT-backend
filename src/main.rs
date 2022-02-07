#[macro_use]
extern crate diesel;

use axum::{
    routing::{get, post},
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

use crate::controller::users::{is_loggedin, login, logout, pre_login, update_name, update_status};
mod read_csv_and_write_db;
mod repository;
mod schema;

#[tokio::main]
async fn main() {
    if let Err(err) = read_csv_and_write_db::run() {
        println!("{}", err);
    }

    // トレーサーを初期化
    tracing_subscriber::fmt::init();

    // ルーターを作成
    let app = Router::new()
        // `GET /` goes to `root`
        .route("/", get(root))
        // `POST /user` goes to `create_user`
        .route("/v1/register", post(create_user))
        .route("/v1/login", post(login))
        .route("/v1/logout", post(logout))
        .route("/v1/pre_login", post(pre_login))
        .route("/v1/user", post(create_user))
        .route("/v1/user/search", get(check_friend_status))
        .route("/v1/user/beacon", post(update_beacon))
        .route("/v1/user/status", post(update_status))
        .route("/v1/user/name", post(update_name))
        // .route("/v1/user/icon", post({}))
        // .route(":id.png", get({}))
        .route("/v1/user/is_loggedin", post(is_loggedin))
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
    use crate::view::{SearchUser, UserCredential, UserView};
    use crate::{model::db_util::establish_connection, view::IdPair};
    use axum::http;
    use diesel::RunQueryDsl;

    #[tokio::test]
    async fn get_search_user() {
        let base_url = "http://localhost:3000";
        let client = reqwest::Client::new();
        let create_usr1 = client
            .post(base_url.to_string() + "/v1/register")
            .json(&UserCredential {
                name: "usr1".to_string(),
                password: "password".to_string(),
            })
            .send()
            .await
            .unwrap();
        assert_eq!(create_usr1.status(), http::StatusCode::OK);

        let id_1 = create_usr1.json::<UserView>().await.unwrap().id;
        // let name_2 = create_usr2.json::<UserView>().await.unwrap().id;
        let search_user = client
            .get(
                base_url.to_string()
                    + "/v1/user/search?my_id="
                    + &id_1.to_string()
                    + "&target_name=usr2",
            )
            .send()
            .await
            .unwrap();
        println!("{:#?}", search_user);
        assert_eq!(search_user.status(), http::StatusCode::OK);
        let result = search_user.json::<Vec<SearchUser>>().await.unwrap();
        println!("{:#?}", result);
        // assert_eq!(result.iter().len(), 0);
    }

    #[tokio::test]
    async fn get_friend_list() {
        // usr1作成
        // usr2作成
        // usr1 -> usr2 に友だち申請
        // search_user
        // レコード初期化
        let base_url = "http://localhost:3000";
        let client = reqwest::Client::new();
        let create_usr1 = client
            .post(base_url.to_string() + "/v1/register")
            .json(&UserCredential {
                name: "usr2_1".to_string(),
                password: "password".to_string(),
            })
            .send()
            .await
            .unwrap();
        assert_eq!(create_usr1.status(), http::StatusCode::OK);

        let create_usr2 = client
            .post(base_url.to_string() + "/v1/register")
            .json(&UserCredential {
                name: "usr2_2".to_string(),
                password: "password".to_string(),
            })
            .send()
            .await
            .unwrap();
        assert_eq!(create_usr2.status(), http::StatusCode::OK);

        let id_1 = create_usr1.json::<UserView>().await.unwrap().id;
        let id_2 = create_usr2.json::<UserView>().await.unwrap().id;
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
            .get(base_url.to_string() + "/v1/friends?my_id=" + &id_1.to_string())
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
    use crate::view::{ScannedBeacon, UserCredential, UserView};
    use axum::http;
    // use diesel::RunQueryDsl;

    #[tokio::test]
    async fn fn1() {
        // ユーザー登録
        // ビーコンをアップデート
        // ユーザー情報を返却
        let base_url = "http://localhost:3000";
        let client = reqwest::Client::new();
        let create_usr1 = client
            .post(base_url.to_string() + "/v1/register")
            .json(&UserCredential {
                name: "usr3_1".to_string(),
                password: "password".to_string(),
            })
            .send()
            .await
            .unwrap();
        assert_eq!(create_usr1.status(), http::StatusCode::OK);

        let user_id = create_usr1.json::<UserView>().await.unwrap().id;

        match client
            .post(base_url.to_string() + "/v1/user/beacon")
            .json(&ScannedBeacon {
                user_id: user_id,
                major: 0,
                minor: 7945,
                rssi: 0,
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
            .get(base_url.to_string() + "/v1/user?id=1")
            .send()
            .await
        {
            Ok(v) => match v.json::<UserView>().await {
                Ok(user_info) => {
                    println!("{:#?}", &user_info);
                    assert_eq!((&user_info).spot, Some("そらの家".to_string()));
                }
                Err(e) => println!("{:?}", e),
            },
            Err(e) => println!("{:?}", e),
        }

        // DBをきれいにする
        // diesel::delete(users).execute(&conn).unwrap();
    }
}

#[cfg(test)]
mod create_user {
    use crate::view::UserCredential;
    use axum::http;

    #[tokio::test]
    async fn collect() {
        let base_url = "http://localhost:3000";
        let client = reqwest::Client::new();
        let create_usr = client
            .post(base_url.to_string() + "/v1/register")
            .json(&UserCredential {
                name: "usr5_1".to_string(),
                password: "password".to_string(),
            })
            .send()
            .await
            .unwrap();
        assert_eq!(create_usr.status(), http::StatusCode::OK);
    }

    #[tokio::test]
    async fn failure_to_short_name() {
        let base_url = "http://localhost:3000";
        let client = reqwest::Client::new();
        let create_usr = client
            .post(base_url.to_string() + "/v1/register")
            .json(&UserCredential {
                name: "".to_string(),
                password: "password".to_string(),
            })
            .send()
            .await
            .unwrap();
        assert_eq!(create_usr.status(), http::StatusCode::UNPROCESSABLE_ENTITY);
    }

    #[tokio::test]
    async fn failure_to_long_name() {
        let base_url = "http://localhost:3000";
        let client = reqwest::Client::new();
        let create_usr = client
            .post(base_url.to_string() + "/v1/register")
            .json(&UserCredential {
                name: "usr5_2xxxxxxxxxxx".to_string(),
                password: "password".to_string(),
            })
            .send()
            .await
            .unwrap();
        assert_eq!(create_usr.status(), http::StatusCode::UNPROCESSABLE_ENTITY);
    }

    #[tokio::test]
    async fn failure_to_short_password() {
        let base_url = "http://localhost:3000";
        let client = reqwest::Client::new();
        let create_usr = client
            .post(base_url.to_string() + "/v1/register")
            .json(&UserCredential {
                name: "5_3".to_string(),
                password: "pass".to_string(),
            })
            .send()
            .await
            .unwrap();
        assert_eq!(create_usr.status(), http::StatusCode::UNPROCESSABLE_ENTITY);
    }

    #[tokio::test]
    async fn failure_to_duplicate_name() {
        let base_url = "http://localhost:3000";
        let client = reqwest::Client::new();
        let create_usr1 = client
            .post(base_url.to_string() + "/v1/register")
            .json(&UserCredential {
                name: "5_4_d".to_string(),
                password: "password".to_string(),
            })
            .send()
            .await
            .unwrap();
        assert_eq!(create_usr1.status(), http::StatusCode::OK);

        let create_usr2 = client
            .post(base_url.to_string() + "/v1/register")
            .json(&UserCredential {
                name: "5_4_d".to_string(),
                password: "password".to_string(),
            })
            .send()
            .await
            .unwrap();
        assert_eq!(create_usr2.status(), http::StatusCode::BAD_REQUEST);
    }
}

#[cfg(test)]
mod login {
    use axum::http;

    use crate::view::UserCredential;

    #[tokio::test]
    async fn collect() {
        let base_url = "http://localhost:3000";
        let client = reqwest::Client::new();
        let create_usr1 = client
            .post(base_url.to_string() + "/v1/register")
            .json(&UserCredential {
                name: "usr4_1".to_string(),
                password: "password".to_string(),
            })
            .send()
            .await
            .unwrap();
        assert_eq!(create_usr1.status(), http::StatusCode::OK);
        let login_test = client
            .post(base_url.to_string() + "/v1/login")
            .json(&UserCredential {
                name: "usr4_1".to_string(),
                password: "password".to_string(),
            })
            .send()
            .await
            .unwrap();
        assert_eq!(login_test.status(), http::StatusCode::OK);
    }

    #[tokio::test]
    async fn failure_invalid_password() {
        let base_url = "http://localhost:3000";
        let client = reqwest::Client::new();
        let create_usr1 = client
            .post(base_url.to_string() + "/v1/register")
            .json(&UserCredential {
                name: "usr4_2".to_string(),
                password: "password".to_string(),
            })
            .send()
            .await
            .unwrap();
        assert_eq!(create_usr1.status(), http::StatusCode::OK);
        let login_test = client
            .post(base_url.to_string() + "/v1/login")
            .json(&UserCredential {
                name: "usr4_2".to_string(),
                password: "invalid_password".to_string(),
            })
            .send()
            .await
            .unwrap();
        assert_eq!(login_test.status(), http::StatusCode::NOT_FOUND);
    }
    #[tokio::test]
    async fn failure_not_exist_name() {
        let base_url = "http://localhost:3000";
        let client = reqwest::Client::new();
        let create_usr = client
            .post(base_url.to_string() + "/v1/register")
            .json(&UserCredential {
                name: "usr4_3".to_string(),
                password: "password".to_string(),
            })
            .send()
            .await
            .unwrap();
        assert_eq!(create_usr.status(), http::StatusCode::OK);
        let login_test = client
            .post(base_url.to_string() + "/v1/login")
            .json(&UserCredential {
                name: "usr4_3_".to_string(),
                password: "password".to_string(),
            })
            .send()
            .await
            .unwrap();
        assert_eq!(login_test.status(), http::StatusCode::NOT_FOUND);
    }
}

#[cfg(test)]
mod logout {
    use axum::http;

    use crate::{
        model::types::UserId,
        view::{UserCredential, UserView},
    };

    #[tokio::test]
    async fn correct() {
        let base_url = "http://localhost:3000";
        let client = reqwest::Client::new();
        let create_usr = client
            .post(base_url.to_string() + "/v1/register")
            .json(&UserCredential {
                name: "usr6_1".to_string(),
                password: "password".to_string(),
            })
            .send()
            .await
            .unwrap();
        assert_eq!(create_usr.status(), http::StatusCode::OK);
        let user_id = create_usr.json::<UserView>().await.unwrap().id;
        let logout = client
            .post(base_url.to_string() + "/v1/logout")
            .json(&UserId { id: user_id })
            .send()
            .await
            .unwrap();
        assert_eq!(logout.status(), http::StatusCode::OK);
    }
    #[tokio::test]
    async fn correct_1() {
        let base_url = "http://localhost:3000";
        let client = reqwest::Client::new();
        let create_usr = client
            .post(base_url.to_string() + "/v1/register")
            .json(&UserCredential {
                name: "usr6_2".to_string(),
                password: "password".to_string(),
            })
            .send()
            .await
            .unwrap();
        assert_eq!(create_usr.status(), http::StatusCode::OK);
        let user_id = create_usr.json::<UserView>().await.unwrap().id;
        let logout = client
            .post(base_url.to_string() + "/v1/logout")
            .json(&UserId { id: user_id })
            .send()
            .await
            .unwrap();
        assert_eq!(logout.status(), http::StatusCode::OK);
        let login_test = client
            .post(base_url.to_string() + "/v1/login")
            .json(&UserCredential {
                name: "usr6_2".to_string(),
                password: "password".to_string(),
            })
            .send()
            .await
            .unwrap();
        assert_eq!(login_test.status(), http::StatusCode::OK);
        let user_id = login_test.json::<UserView>().await.unwrap().id;
        let logout = client
            .post(base_url.to_string() + "/v1/logout")
            .json(&UserId { id: user_id })
            .send()
            .await
            .unwrap();
        assert_eq!(logout.status(), http::StatusCode::OK);
    }
}

#[cfg(test)]
pub mod update_name_test {
    use axum::http;

    use crate::view::{IdAndName, UserCredential, UserView};

    #[tokio::test]
    async fn success() {
        let base_url = "http://localhost:3000";
        let client = reqwest::Client::new();
        let create_usr = client
            .post(base_url.to_string() + "/v1/register")
            .json(&UserCredential {
                name: "usr7_1".to_string(),
                password: "password".to_string(),
            })
            .send()
            .await
            .unwrap();
        assert_eq!(create_usr.status(), http::StatusCode::OK);

        let user = create_usr.json::<UserView>().await.unwrap();
        let id = user.id;
        let name_1 = user.name;
        
        let update_name = client
            .post(base_url.to_string() + "/v1/user/name")
            .json(&IdAndName {
                my_id: id,
                target_name: "usr_7_1_1".to_string(),
            })
            .send()
            .await
            .unwrap();
        assert_eq!(update_name.status(), http::StatusCode::OK);

        let user_1 = update_name.json::<UserView>().await.unwrap();
        let name_2 = user_1.name;

        assert_eq!(name_1, name_2)

    }
    #[tokio::test]
    async fn success_update_same_name() {
        let base_url = "http://localhost:3000";
        let client = reqwest::Client::new();
        let create_usr = client
            .post(base_url.to_string() + "/v1/register")
            .json(&UserCredential {
                name: "usr7_2".to_string(),
                password: "password".to_string(),
            })
            .send()
            .await
            .unwrap();
        assert_eq!(create_usr.status(), http::StatusCode::OK);
        let id = create_usr.json::<UserView>().await.unwrap().id;
        let update_name = client
            .post(base_url.to_string() + "/v1/user/name")
            .json(&IdAndName {
                my_id: id,
                target_name: "usr7_2".to_string(),
            })
            .send()
            .await
            .unwrap();
        assert_eq!(update_name.status(), http::StatusCode::OK);
    }

    #[tokio::test]
    async fn failure_1_duplicate_nickname() {
        let base_url = "http://localhost:3000";
        let client = reqwest::Client::new();
        let create_usr = client
            .post(base_url.to_string() + "/v1/register")
            .json(&UserCredential {
                name: "usr7_3".to_string(),
                password: "password".to_string(),
            })
            .send()
            .await
            .unwrap();
        assert_eq!(create_usr.status(), http::StatusCode::OK);

        let create_usr_1 = client
            .post(base_url.to_string() + "/v1/register")
            .json(&UserCredential {
                name: "usr7_4".to_string(),
                 password: "password".to_string(),
            })
            .send()
            .await
            .unwrap();
        assert_eq!(create_usr_1.status(), http::StatusCode::OK);
        let id = create_usr_1.json::<UserView>().await.unwrap().id;

        let update_name = client
            .post(base_url.to_string() + "/v1/user/name")
            .json(&IdAndName {
                my_id: id,
                target_name: "usr7_3".to_string(),
            })
            .send()
            .await
            .unwrap();
        assert_eq!(update_name.status(), http::StatusCode::BAD_REQUEST);
    }
}

#[cfg(test)]
pub mod update_status_test {
    use axum::http;

    use crate::view::{IdAndStatus, UserCredential, UserView};

    #[tokio::test]
    async fn success() {
        let base_url = "http://localhost:3000";
        let client = reqwest::Client::new();
        let create_usr = client
            .post(base_url.to_string() + "/v1/register")
            .json(&UserCredential {
                name: "usr8_1".to_string(),
                password: "password".to_string(),
            })
            .send()
            .await
            .unwrap();
        assert_eq!(create_usr.status(), http::StatusCode::OK);

        let id = create_usr.json::<UserView>().await.unwrap().id;
        let update_status_0 = client
            .post(base_url.to_string() + "/v1/user/status")
            .json(&IdAndStatus { id, status: 0 })
            .send()
            .await
            .unwrap();
        assert_eq!(update_status_0.status(), http::StatusCode::OK);
        let update_status_1 = client
            .post(base_url.to_string() + "/v1/user/status")
            .json(&IdAndStatus { id, status: 1 })
            .send()
            .await
            .unwrap();
        assert_eq!(update_status_1.status(), http::StatusCode::OK);
        let update_status_2 = client
            .post(base_url.to_string() + "/v1/user/status")
            .json(&IdAndStatus { id, status: 2 })
            .send()
            .await
            .unwrap();
        assert_eq!(update_status_2.status(), http::StatusCode::OK);
        let update_status_3 = client
            .post(base_url.to_string() + "/v1/user/status")
            .json(&IdAndStatus { id, status: 3 })
            .send()
            .await
            .unwrap();
        assert_eq!(update_status_3.status(), http::StatusCode::OK);
    }

    #[tokio::test]
    async fn failure() {
        let base_url = "http://localhost:3000";
        let client = reqwest::Client::new();
        let create_usr = client
            .post(base_url.to_string() + "/v1/register")
            .json(&UserCredential {
                name: "usr8_2".to_string(),
                password: "password".to_string(),
            })
            .send()
            .await
            .unwrap();
        assert_eq!(create_usr.status(), http::StatusCode::OK);

        let id = create_usr.json::<UserView>().await.unwrap().id;
        let update_status_4 = client
            .post(base_url.to_string() + "/v1/user/status")
            .json(&IdAndStatus { id, status: 4 })
            .send()
            .await
            .unwrap();
        assert_eq!(
            update_status_4.status(),
            http::StatusCode::UNPROCESSABLE_ENTITY
        );

        let update_status__1 = client
            .post(base_url.to_string() + "/v1/user/status")
            .json(&IdAndStatus { id, status: -1 })
            .send()
            .await
            .unwrap();
        assert_eq!(
            update_status__1.status(),
            http::StatusCode::UNPROCESSABLE_ENTITY
        );
    }
}
