use std::{env, io::Cursor};

use cloud_storage::{Client, Object};
use image::{DynamicImage, ImageOutputFormat};
// セッションを作る
// バケットに接続する/
// バケットがなければ作成する
// バケットにファイルを保存する
// バケットのURLを取得する

// バケットのURLを指定する

pub fn set_gcs_env() {
    let key = "SERVICE_ACCOUNT";
    // /run/secrets/service_account.json
    env::set_var(key, "/run/secrets/service-account.json");
}

pub fn create_client() -> Client {
    let client = Client::new();
    client
}

// async fn is_exist_bucket(client: &Client, bucket_name: &str) -> bool {
//     if let Ok(_) = client.bucket().read(bucket_name).await {
//         true
//     } else {
//         false
//     }
// }

// async fn get_item(client: &Client, bucket_name: &str, item_name: &str) -> Option<Object> {
//     if let Ok(item) = client.object().read(bucket_name, item_name).await {
//         Some(item)
//     } else {
//         None
//     }
// }

pub async fn upload_image(
    client: &Client,
    image_name: &str,
    image: DynamicImage,
) -> Result<Object, cloud_storage::Error> {
    let mut _bytes = Cursor::new(Vec::new());
    image.write_to(&mut _bytes, ImageOutputFormat::Png).unwrap();
    let bytes = _bytes.into_inner();
    let bucket_name: String = env::var("BUCKET_NAME").unwrap();

    let result = client
        .object()
        .create(bucket_name.as_str(), bytes, image_name, "image/png")
        .await;

    result
}

// ユーザーがアカウントを削除したときのために，後で実装する
// async fn delete_image(){

// }
