table! {
    deck (id) {
        id -> Integer,
        alias -> Text,
        commander -> Text,
        player_id -> Integer,
        active -> Bool,
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
        elo -> Double,
    }
}

table! {
    player (id) {
        id -> Integer,
        alias -> Text,
        email -> Text,
        password -> Text,
    }
}

table! {
    token (id) {
        id -> Integer,
        player_id -> Integer,
        content -> Text,
    }
}

joinable!(deck -> player (player_id));
joinable!(participant -> deck (deck_id));
joinable!(participant -> game (game_id));
joinable!(token -> player (player_id));

allow_tables_to_appear_in_same_query!(deck, game, participant, player, token,);
