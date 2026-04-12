// @generated automatically by Diesel CLI.

diesel::table! {
    players (id) {
        id -> Integer,
        name -> Text,
        variant -> Integer,
        hashed_password -> Text,
    }
}
