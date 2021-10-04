use axum::{
    handler::{get, post},
    Router,
};
use std::net::SocketAddr;

use tracing;
use tracing_subscriber;

mod user;
use user::controller as user_controller;

mod friends;
use friends::controller as friends_controller;

#[tokio::main]
async fn main() {
    // トレーサーを初期化
    tracing_subscriber::fmt::init();

    // ルーターを作成
    let app = Router::new()
        // `GET /` goes to `root`
        .route("/", get(root))
        // `POST /users` goes to `create_user`
        .route("/users", post(user_controller::create_user))
        .route("/v1/friends/add", post(friends_controller::add_friend));

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
