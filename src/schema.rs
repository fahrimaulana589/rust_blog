// @generated automatically by Diesel CLI.

diesel::table! {
    blog (id) {
        id -> Integer,
        title -> Text,
        content -> Text,
        category_id -> Integer,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        slug -> Text,
        excerpt -> Nullable<Text>,
        thumbnail -> Nullable<Text>,
        status -> Text,
        published_at -> Nullable<Timestamp>,
        view_count -> Integer,
    }
}

diesel::table! {
    blog_tags (blog_id, tag_id) {
        blog_id -> Integer,
        tag_id -> Integer,
    }
}

diesel::table! {
    categories (id) {
        id -> Integer,
        name -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    counts (id) {
        id -> Nullable<Integer>,
        count -> Nullable<Integer>,
    }
}

diesel::table! {
    tags (id) {
        id -> Integer,
        name -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
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

diesel::joinable!(blog -> categories (category_id));
diesel::joinable!(blog_tags -> blog (blog_id));
diesel::joinable!(blog_tags -> tags (tag_id));

diesel::allow_tables_to_appear_in_same_query!(blog, blog_tags, categories, counts, tags, users,);
