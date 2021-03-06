extern crate flat_backend;

mod upload_gcs {
    use flat_backend::worker::utils::image_crop::image_to_base64;
    use flat_backend::worker::view::{IdAndIcon, UserCredential, UserView};

    #[tokio::test]
    #[ignore]
    async fn upload_image_test() {
        // creat user
        let base_url = "http://localhost:3000";
        let client = reqwest::Client::new();
        let create_usr = client
            .post(base_url.to_string() + "/v1/register")
            .json(&UserCredential {
                name: "test_901".to_string(),
                password: "password".to_string(),
            })
            .send()
            .await
            .unwrap();
        println!("{:#?}", create_usr);
        assert_eq!(create_usr.status(), reqwest::StatusCode::OK);
        let user_id = create_usr.json::<UserView>().await.unwrap().id;
        println!("{}", user_id);

        // get image from web
        let image_bytes = reqwest::get("https://picsum.photos/200/200")
            .await
            .unwrap()
            .bytes()
            .await
            .unwrap();

        let image = image::load_from_memory(&image_bytes).unwrap();

        // convert image to base64
        let base64_encoded_image = image_to_base64(image);

        // update icon
        let update_icon = client
            .post(base_url.to_string() + "/v1/user/icon")
            .json(&IdAndIcon {
                id: user_id,
                image: base64_encoded_image,
            })
            .send()
            .await
            .unwrap();

        // compare result
        println!("{:#?}", update_icon);
        assert_eq!(update_icon.status(), reqwest::StatusCode::OK);
    }
}
