table! {
    users (id) {
        id -> Integer,
        user_id -> Text,
        user_name -> Text,
        status -> Nullable<Integer>,
        beacon -> Nullable<Text>,
        icon_path -> Nullable<Text>,
        hashed_password -> Nullable<Text>,
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
