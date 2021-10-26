table! {
    friends (acctive, pussive) {
        acctive -> Integer,
        pussive -> Integer,
        delete_flag -> Bool,
    }
}

table! {
    users (user_id) {
        user_id -> Integer,
        user_name -> Text,
        status -> Integer,
        beacon -> Nullable<Text>,
        icon_path -> Text,
        hashed_password -> Text,
    }
}

allow_tables_to_appear_in_same_query!(friends, users,);
