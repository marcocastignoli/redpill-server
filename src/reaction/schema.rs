use user::schema::users;
use post::schema::posts;

table! {
    reactions (reaction_id) {
        reaction_id -> Integer,
        reaction_type -> Integer,
        post_id -> Integer,
        author_id -> Integer,
        created -> Nullable<Datetime>,
    }
}

joinable!(reactions -> users (author_id));
joinable!(reactions -> posts (post_id));