// 友だち追加の流れ
// API -> (id, id): (String, String)

fn is_exist_id(id: String) -> bool {
    // db に接続。チェックする
    true
}

fn add_friend(id: String, friend_id: String) {
    // IDがレコードに存在してるかチェック
    is_exist_id(id);
    is_exist_id(friend_id);
    // DBにインサート
    // bool か Result を返す
}

fn get_friend() -> Option {}

struct UserId {}

struct Friend {}
