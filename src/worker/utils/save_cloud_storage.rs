use cloud_storage::Client;
use image::DynamicImage;

// セッションを作る
// バケットに接続する/
// バケットがなければ作成する
// バケットにファイルを保存する
// バケットのURLを取得する

// バケットのURLを指定する

fn crate_client() -> Client {
    let client = Client::new();
    client
}

async fn is_exist_bucket(client: &Client, bucket_name: &str) -> bool {
    if let Ok(_) = client.bucket().read(bucket_name).await {
        true
    } else {
        false
    }
}

// 画像の名前をどこできめるか
async fn update_image(client: &Client, bucket_name: &str, image_name: &str, image: DynamicImage) {
    client
        .object()
        .create(bucket_name, image.into_bytes(), image_name, "image/png")
        .await;
}
