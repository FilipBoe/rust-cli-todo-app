// @generated automatically by Diesel CLI.

diesel::table! {
    todos (id) {
        id -> Nullable<Integer>,
        title -> Text,
        description -> Nullable<Text>,
        completed -> Bool,
        created_at -> Nullable<Timestamp>,
    }
}
