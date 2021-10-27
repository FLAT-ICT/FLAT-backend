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
use controller::user::create_user;
mod repository;
mod schema;

#[tokio::main]
async fn main() {
    // トレーサーを初期化
    tracing_subscriber::fmt::init();

    // ルーターを作成
    let app = Router::new()
        // `GET /` goes to `root`
        .route("/", get(root))
        // `POST /users` goes to `create_user`
        .route("/v1/users", post(create_user))
        .route("/v1/users/search", get(check_friend_status))
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
    use crate::view::CreateUser;

    #[tokio::test]
    async fn basic() {
        // usr1作成
        // usr2作成
        // usr1 -> usr2 に友だち申請
        // search_user
        // レコード初期化
        let base_url = "http://localhost:3000";
        let client = reqwest::Client::new();
        let _create_usr1 = client
            .post(base_url.to_string() + "/v1/users")
            .json(&CreateUser {
                username: "usr1".to_string(),
            })
            .send()
            .await
            .unwrap();

        let _create_usr2 = client
            .post(base_url.to_string() + "/v1/users")
            .json(&CreateUser {
                username: "usr2".to_string(),
            })
            .send()
            .await
            .unwrap();
        // let _friend_request = client.post(base_url);
    }
}
