use diesel;
use diesel::prelude::*;
use schema::{deck, game, participant, player};
use std::marker::Sized;

use time;

pub trait Retrievable where Self: Sized {
    fn all(conn: &SqliteConnection) ->  QueryResult<Vec<Self>>;
    fn find(id: i32, conn: &SqliteConnection) ->  QueryResult<Self>;
    fn update(player: Self, conn: &SqliteConnection) -> QueryResult<Self>;
    fn delete(id: i32, conn: &SqliteConnection) -> bool;
}

pub trait Insertable where Self: Sized {
    type T: Retrievable;
    fn insert(model: Self, conn: &SqliteConnection) -> QueryResult<Self::T>;
}

#[derive(Identifiable, Queryable, Serialize, Deserialize, AsChangeset, Associations)]
#[table_name = "player"]
#[has_many(Deck, Participant)]
pub struct Player {
    pub id: i32,
    pub alias: String,
}

#[derive(Insertable)]
#[table_name = "player"]
#[derive(Deserialize)]
pub struct NewPlayer {
    pub alias: String,
}

impl Retrievable for Player {
    fn all(conn: &SqliteConnection) -> QueryResult<Vec<Player>> {
        player::table.order(player::id).load::<Player>(conn)
    }

    fn find(id: i32, conn: &SqliteConnection) -> QueryResult<Player> {
        player::table.find(id).get_result::<Player>(conn)
    }

    fn update(player: Self, conn: &SqliteConnection) -> QueryResult<Self> {
        diesel::update(player::table.find(player.id))
            .set(&player)
            .execute(conn)
            .and_then(|_| player::table.find(player.id).get_result::<Player>(conn))
    }

    fn delete(id: i32, conn: &SqliteConnection) -> bool {
         diesel::delete(player::table.find(id)).execute(conn).is_ok()
    }
}

impl Insertable for NewPlayer {

    type T = Player;

    fn insert (player: NewPlayer, conn: &SqliteConnection) -> QueryResult<Player> {
        // Diesel doesn't expose a get result method
        diesel::insert_into(player::table)
            .values(&player)
            .execute(conn)
            .and_then(|_| player::table.order(player::id.desc()).first(conn))
    }
}

#[derive(Identifiable, Queryable, Serialize, Deserialize, AsChangeset, Associations)]
#[table_name = "deck"]
#[belongs_to(Player)]
pub struct Deck {
    pub id: i32,
    pub alias: String,
    pub commander: String,
    pub player_id: i32,
}

#[derive(Insertable)]
#[table_name = "deck"]
#[derive(Deserialize)]
pub struct NewDeck {
    pub alias: String,
    pub commander: String,
    pub player_id: i32,
}

impl Retrievable for Deck {
    fn all(conn: &SqliteConnection) -> QueryResult<Vec<Deck>> {
        deck::table.order(deck::id).load::<Deck>(conn)
    }

    fn find(id: i32, conn: &SqliteConnection) -> QueryResult<Deck> {
        deck::table.find(id).get_result::<Deck>(conn)
    }

    fn update(deck: Deck, conn: &SqliteConnection) -> QueryResult<Deck> {
        diesel::update(deck::table.find(deck.id))
            .set(&deck)
            .execute(conn)
            .and_then(|_| deck::table.find(deck.id).get_result::<Deck>(conn))
    }

    fn delete(id: i32, conn: &SqliteConnection) -> bool {
        diesel::delete(deck::table.find(id)).execute(conn).is_ok()
    }
}

impl Insertable for NewDeck {

    type T = Deck;

    fn insert(deck: NewDeck, conn: &SqliteConnection) -> QueryResult<Deck> {
        // Diesel doesn't expose a get result method
        diesel::insert_into(deck::table)
            .values(&deck)
            .execute(conn)
            .and_then(|_| deck::table.order(deck::id.desc()).first(conn))
    }
}

impl Deck {
    pub fn find_by_player(player: &Player, conn: &SqliteConnection) -> QueryResult<Vec<Deck>> {
        Deck::belonging_to(player).load::<Deck>(conn)
    }
}

#[derive(Identifiable, Queryable, Serialize, Deserialize, Associations, AsChangeset)]
#[table_name = "game"]
#[has_many(Participant)]
pub struct Game {
    pub id: i32,
    pub time_stamp: f64,
}

#[derive(Insertable)]
#[table_name = "game"]
#[derive(Deserialize)]
pub struct NewGame {
    pub time_stamp: f64,
}

impl Retrievable for Game {

    fn all(conn: &SqliteConnection) -> QueryResult<Vec<Game>> {
        game::table.order(game::id).load::<Game>(conn)
    }

    fn find(id: i32, conn: &SqliteConnection) -> QueryResult<Game> {
        game::table.find(id).get_result::<Game>(conn)
    }

    fn update(game: Game, conn: &SqliteConnection) -> QueryResult<Game> {
        diesel::update(game::table.find(game.id))
            .set(&game)
            .execute(conn)
            .and_then(|_| game::table.find(game.id).get_result::<Game>(conn))
    }

    fn delete(id: i32, conn: &SqliteConnection) -> bool {
        diesel::delete(game::table.find(id)).execute(conn).is_ok()
    }
}

impl Insertable for NewGame {

    type T = Game;

    fn insert(game: NewGame, conn: &SqliteConnection) -> QueryResult<Game> {
        // Diesel doesn't expose a get result method
        diesel::insert_into(game::table)
            .values(&game)
            .execute(conn)
            .and_then(|_| game::table.order(game::id.desc()).first(conn))
    }
}

impl NewGame {
    pub fn new() -> NewGame {
            let timespec = time::get_time();
            let millis: f64 = timespec.sec as f64 + (timespec.nsec as f64 / 1000.0 / 1000.0 / 1000.0);
            NewGame { time_stamp: millis }
    }
}


#[derive(Identifiable, Queryable, Serialize, Deserialize, Associations)]
#[table_name = "participant"]
#[belongs_to(Game, Deck)]
pub struct Participant {
    pub id: i32,
    pub game_id: i32,
    pub deck_id: i32,
    pub win: bool,
}

#[derive(Insertable)]
#[table_name = "participant"]
pub struct NewParticipant {
    pub deck_id: i32,
    pub game_id: i32,
    pub win: bool,
}

/*
* This is what comes in the request. We don't know the 
* Game id yet so we want to turn this into a NewParticipant
* Before insertion
*/
#[derive(Deserialize)]
pub struct ParticipantRequest {
    pub deck_id: i32,
    pub win: bool,
}

impl NewParticipant {

    pub fn new (game_id: i32, deck_id: i32, win: bool) -> NewParticipant {
        NewParticipant { deck_id, game_id, win }
    }

    pub fn insert(new_participants: &Vec<NewParticipant>, conn: &SqliteConnection) -> QueryResult<Vec<Participant>> {
        diesel::insert_into(participant::table)
            .values(new_participants)
            .execute(conn)
            .and_then(|count| participant::table.order(participant::id.desc())
                                .limit(count as i64)
                                .get_results::<Participant>(conn))
    }
}
