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
        name_ja -> Text,
        name_en -> Text,
        region_identifier -> Text,
        available_term_from -> Timestamp,
        available_term_to -> Nullable<Timestamp>,
        major -> Integer,
        minor -> Integer,
        note -> Nullable<Text>,
    }
}

table! {
    users (id) {
        id -> Integer,
        name -> Text,
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
