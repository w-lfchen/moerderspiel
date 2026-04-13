// @generated automatically by Diesel CLI.

diesel::table! {
    games (id) {
        id -> Integer,
        name -> Text,
        phase -> Integer,
    }
}

diesel::table! {
    kills (id) {
        id -> Integer,
        timestamp -> Integer,
        killer -> Integer,
        target -> Integer,
    }
}

diesel::table! {
    orders (id) {
        id -> Integer,
        circle -> Integer,
        executor -> Integer,
        target -> Integer,
    }
}

diesel::table! {
    players (id) {
        id -> Integer,
        name -> Text,
        variant -> Integer,
        hashed_password -> Text,
    }
}

diesel::allow_tables_to_appear_in_same_query!(games, kills, orders, players,);
