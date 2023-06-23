// @generated automatically by Diesel CLI.

diesel::table! {
    todos (id) {
        id -> Text,
        title -> Text,
        description -> Nullable<Text>,
        created_on -> Nullable<Text>,
        deadline -> Nullable<Text>,
    }
}
