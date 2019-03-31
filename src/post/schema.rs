use user::schema::users;
use reaction::schema::reactions;

table! {
    posts (post_id) {
        post_id -> Integer,
        content -> Varchar,
        author_id -> Integer,
        parent_id -> Nullable<Integer>,
        created -> Nullable<Datetime>,
    }
}

joinable!(posts -> users (author_id));

allow_tables_to_appear_in_same_query!(
    posts,
    users,
    reactions
);