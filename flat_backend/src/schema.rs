table! {
    friends (id) {
        id -> Integer,
        acctive -> Text,
        pussive -> Text,
        delete_flag -> Bool,
    }
}

table! {
    users (id) {
        id -> Integer,
        user_id -> Text,
        user_name -> Text,
        status -> Integer,
        beacon -> Nullable<Text>,
        icon_path -> Text,
        hashed_password -> Text,
    }
}

allow_tables_to_appear_in_same_query!(
    friends,
    users,
);
