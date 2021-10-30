table! {
    friends (acctive, pussive) {
        acctive -> Integer,
        pussive -> Integer,
        created_at -> Timestamp,
        blocked_at -> Nullable<Timestamp>,
    }
}

table! {
    spots (major, minor) {
        ja_spot -> Text,
        en_spot -> Text,
        region_identifire -> Text,
        from_date -> Timestamp,
        to_date -> Nullable<Timestamp>,
        major -> Integer,
        minor -> Integer,
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
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

allow_tables_to_appear_in_same_query!(
    friends,
    spots,
    users,
);
