table! {
    users(id) {
        id -> Integer,
        user_id -> Text,
        user_name -> Text,
        status -> Integer,
        beacon -> Text,
        icon_path -> Text,
        hashed_password -> Text,
    }
}

table! {
    friends(id) {
        id -> Integer,
        acctive -> Text,
        pussive -> Text,
        delete_flag -> Bool,
    }
}
