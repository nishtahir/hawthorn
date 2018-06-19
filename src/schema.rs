table! {
    deck (id) {
        id -> Integer,
        alias -> Text,
        player_id -> Integer,
    }
}

table! {
    player (id) {
        id -> Integer,
        alias -> Text,
        win -> Integer,
        loss -> Integer,
    }
}

allow_tables_to_appear_in_same_query!(
    deck,
    player,
);
