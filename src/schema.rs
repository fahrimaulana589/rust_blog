// @generated automatically by Diesel CLI.

diesel::table! {
    counts (id) {
        id -> Nullable<Integer>,
        count -> Nullable<Integer>,
    }
}

diesel::table! {
    users (id) {
        id -> Nullable<Integer>,
        username -> Text,
        email -> Text,
        password -> Text,
    }
}

diesel::allow_tables_to_appear_in_same_query!(counts, users,);
