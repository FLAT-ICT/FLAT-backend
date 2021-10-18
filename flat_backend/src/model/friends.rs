use super::super::schema::friends;
use super::super::view::IdPair;
use super::db_util;
use super::db_util::is_exist_id;
use super::types::{AddFriend, SearchUser, SomeError};
use crate::model::db_util::{get_friends_relation, get_user_id_name_path};
use axum::response::IntoResponse;
use diesel::RunQueryDsl;
use hyper::{Body, Response, StatusCode};
use validator::Validate;

// 友だち追加の流れ
// API -> (id, id): (String, String)

pub fn add_friend(id_pair: IdPair) -> bool {
    // -> (status_code: int, message: String)
    let my_id = id_pair.my_id;
    let friend_id = id_pair.target_id;

    if my_id == friend_id {
        return false;
    }

    // IDがレコードに存在してるかチェック
    if !is_exist_id(&my_id) || !is_exist_id(&friend_id) {
        return false;
    }

    let ids = AddFriend {
        acctive: &my_id,
        pussive: &friend_id,
    };

    let conn = db_util::establish_connection();
    diesel::insert_into(friends::table)
        .values(&ids)
        .execute(&conn)
        .expect("挿入失敗");

    return true;
    // DBにインサート
    // bool か Result を返す
}

impl IntoResponse for SomeError {
    type Body = Body;
    type BodyError = <Self::Body as axum::body::HttpBody>::Error;
    fn into_response(self) -> Response<Self::Body> {
        let body = match self {
            SomeError::ValidationError => Body::from("something went wrong"),
            SomeError::NotExistError => Body::from("something else went wrong"),
            SomeError::SameIdError => Body::from("something else went wrong"),
        };

        Response::builder()
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .body(body)
            .unwrap()
    }
}

pub fn search_user(id_pair: IdPair) -> Result<SearchUser, SomeError> {
    // pub fn search_user(id_pair: IdPair) -> (u16, String) {

    // バリデーション
    if let Err(_r) = &id_pair.validate() {
        // return (422, r.to_string());
        return Err(SomeError::ValidationError);
    }

    let my_id = id_pair.my_id;
    let friend_id = id_pair.target_id;

    // レコード存在確認
    if !is_exist_id(&my_id) || !is_exist_id(&friend_id) {
        // return (404, "Err, id not found".to_string());
        return Err(SomeError::NotExistError);
    }

    // if invalid validation {
    //      どこでキャッチすればいいのかわかってない。axumの仕様調べる。
    // }

    // 自身を登録
    if my_id == friend_id {
        // return (471, "Err, send same id".to_string());
        return Err(SomeError::SameIdError);
    }

    // db_util::get_user_id_name_path(id) -> (id, name, path)
    // db_util::get_friends_relation(id1, id2) -> (bool, bool)

    let (id, name, path) = get_user_id_name_path(&friend_id);
    let (ap, req) = get_friends_relation(&my_id, &friend_id);
    return Ok(SearchUser {
        user_id: id,
        user_name: name,
        icon_path: path,
        applied: ap,
        requested: req,
    });
}

// fn get_friend() -> Option {}


#[cfg(test)]
mod tests {
    use crate::{model::friends::add_friend, view::IdPair};

    #[test]
    fn t_add_friend() {
        // insert 000000
        // insert 000001

        // 上の状態までDBを復帰させる必要あり

        // 正常
        assert_eq!(
            add_friend(IdPair {
                my_id: "000000".to_string(),
                target_id: "000001".to_string()
            }),
            true
        );
        // 同じIDが挿入されるのはおかしい
        assert_eq!(
            add_friend(IdPair {
                my_id: "000000".to_string(),
                target_id: "000000".to_string()
            }),
            false
        );

        // 存在しないIDに友だち申請するのはおかしい
        assert_eq!(
            add_friend(IdPair {
                my_id: "000000".to_string(),
                target_id: "000002".to_string()
            }),
            false
        );

        // 不正なID
        assert_eq!(
            add_friend(IdPair {
                my_id: "abcdef".to_string(),
                target_id: "000000".to_string()
            }),
            false
        );

        // 不正なID
        assert_eq!(
            add_friend(IdPair {
                my_id: "12345".to_string(),
                target_id: "000000".to_string()
            }),
            false
        );
    }
}
