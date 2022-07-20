table! {
    friends (active, passive) {
        active -> Int4,
        passive -> Int4,
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
        major -> Int4,
        minor -> Int4,
        note -> Nullable<Text>,
    }
}

table! {
    users (id) {
        id -> Int4,
        name -> Varchar,
        status -> Int4,
        spot -> Nullable<Text>,
        icon_path -> Text,
        salt -> Bytea,
        hash -> Bytea,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        logged_in_at -> Nullable<Timestamp>,
    }
}

allow_tables_to_appear_in_same_query!(
    friends,
    spots,
    users,
);
