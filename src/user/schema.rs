table! {
    users (user_id) {
        user_id -> Nullable<Integer>,
        email -> Varchar,
        password -> Varchar,
        created -> Nullable<Datetime>,
    }
}