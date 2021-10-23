#[macro_use]
extern crate diesel;

use axum::{
    handler::{get, post},
    Router,
};
use std::net::SocketAddr;

use tracing;
use tracing_subscriber;

mod model;
mod view;

mod controller;
// use controller::
use controller::friends::add_friend;
use controller::user::create_user;

use crate::controller::friends::{check_friend_status, friend_list, reject_friend};

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
        .route("/users", post(create_user))
        .route("/v1/users/check", get(check_friend_status))
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
    #[tokio::test]
    async fn my_test() {
        assert!(true);
    }
}
