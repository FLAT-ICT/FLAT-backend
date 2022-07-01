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