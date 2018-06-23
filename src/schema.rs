table! {
    deck (id) {
        id -> Integer,
        alias -> Text,
        commander -> Text,
        player_id -> Integer,
    }
}

table! {
    game (id) {
        id -> Integer,
        time_stamp -> Double,
    }
}

table! {
    participant (id) {
        id -> Integer,
        game_id -> Integer,
        deck_id -> Integer,
        win -> Bool,
    }
}

table! {
    player (id) {
        id -> Integer,
        alias -> Text,
    }
}

allow_tables_to_appear_in_same_query!(
    deck,
    game,
    participant,
    player,
);
