// @generated automatically by Diesel CLI.

diesel::table! {
    groups (id) {
        id -> Int4,
        name -> Varchar,
        closed -> Bool,
    }
}

diesel::table! {
    santa (id) {
        id -> Int4,
        user_id -> Int4,
        present_id -> Int4,
        group_id -> Int4,
    }
}

diesel::table! {
    user_group (id) {
        id -> Int4,
        user_id -> Int4,
        group_id -> Int4,
        is_admin -> Bool,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        login -> Varchar,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    groups,
    santa,
    user_group,
    users,
);
