// @generated automatically by Diesel CLI.

diesel::table! {
    post_topics (id) {
        id -> Integer,
        post_id -> Integer,
        topic_id -> Integer,
    }
}

diesel::table! {
    posts (id) {
        id -> Integer,
        title -> Text,
        body -> Text,
        published -> Bool,
        user_id -> Integer,
    }
}

diesel::table! {
    topics (id) {
        id -> Integer,
        name -> Text,
    }
}

diesel::table! {
    users (id) {
        id -> Integer,
        name -> Text,
    }
}

diesel::joinable!(post_topics -> posts (post_id));
diesel::joinable!(post_topics -> topics (topic_id));
diesel::joinable!(posts -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    post_topics,
    posts,
    topics,
    users,
);
