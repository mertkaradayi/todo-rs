// @generated automatically by Diesel CLI.

diesel::table! {
    todos (id) {
        id -> Integer,
        description -> Text,
        status -> Text,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}
