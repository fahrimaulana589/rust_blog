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
    portofolios (id) {
        id -> Integer,
        project_id -> Integer,
        judul -> Text,
        deskripsi -> Nullable<Text>,
        is_active -> Bool,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    profile_languages (id) {
        id -> Integer,
        profile_id -> Integer,
        name -> Text,
        level -> Text,
    }
}

diesel::table! {
    profile_specializations (id) {
        id -> Integer,
        profile_id -> Integer,
        specialization -> Text,
    }
}

diesel::table! {
    profile_tech_focus (id) {
        id -> Integer,
        profile_id -> Integer,
        tech_focus -> Text,
    }
}

diesel::table! {
    profiles (id) {
        id -> Integer,
        full_name -> Text,
        headline -> Text,
        summary -> Text,
        role -> Text,
        location -> Text,
        profile_image -> Text,
        availability -> Text,
        years_of_experience -> Integer,
        resume_url -> Text,
        email -> Text,
        work_philosophy -> Text,
        timezone -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    project_stack (id) {
        id -> Integer,
        project_id -> Integer,
        stack_id -> Integer,
    }
}

diesel::table! {
    projects (id) {
        id -> Integer,
        nama_projek -> Text,
        deskripsi -> Text,
        status -> Text,
        progress -> Integer,
        link_demo -> Nullable<Text>,
        repository -> Nullable<Text>,
        tanggal_mulai -> Date,
        tanggal_selesai -> Nullable<Date>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    stacks (id) {
        id -> Integer,
        nama_stack -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
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
diesel::joinable!(portofolios -> projects (project_id));
diesel::joinable!(profile_languages -> profiles (profile_id));
diesel::joinable!(profile_specializations -> profiles (profile_id));
diesel::joinable!(profile_tech_focus -> profiles (profile_id));
diesel::joinable!(project_stack -> projects (project_id));
diesel::joinable!(project_stack -> stacks (stack_id));

diesel::allow_tables_to_appear_in_same_query!(
    blog,
    blog_tags,
    categories,
    counts,
    portofolios,
    profile_languages,
    profile_specializations,
    profile_tech_focus,
    profiles,
    project_stack,
    projects,
    stacks,
    tags,
    users,
);
