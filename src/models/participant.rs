use diesel;
use diesel::prelude::*;
use models::deck::Deck;
use models::game::Game;
use models::Retrievable;
use schema::participant;

#[derive(Identifiable, Queryable, Serialize, Deserialize, Associations, Debug)]
#[table_name = "participant"]
#[belongs_to(Game)]
#[belongs_to(Deck)]
pub struct Participant {
    pub id: i32,
    pub game_id: i32,
    pub deck_id: i32,
    pub win: bool,
    pub elo: f64,
}

#[derive(Insertable)]
#[table_name = "participant"]
pub struct NewParticipant {
    pub game_id: i32,
    pub deck_id: i32,
    pub win: bool,
    pub elo: f64,
}

impl Participant {
    pub fn find_by_game(game: &Game, conn: &SqliteConnection) -> QueryResult<Vec<Participant>> {
        Participant::belonging_to(game).load::<Participant>(conn)
    }

    pub fn find_by_deck(deck: &Deck, conn: &SqliteConnection) -> QueryResult<Vec<Participant>> {
        Participant::belonging_to(deck)
            .order(participant::id.desc())
            .load::<Participant>(conn)
    }

    pub fn find_latest_participant_by_deck_id(
        deck_id: i32,
        conn: &SqliteConnection,
    ) -> QueryResult<Participant> {
        let deck = Deck::find(deck_id, conn)?;
        Participant::belonging_to(&deck)
            .order(participant::id.desc())
            .first(conn)
    }
}

impl NewParticipant {
    pub fn new(game_id: i32, deck_id: i32, win: bool, elo: f64) -> NewParticipant {
        NewParticipant {
            deck_id,
            game_id,
            win,
            elo,
        }
    }

    pub fn insert(
        new_participants: &Vec<NewParticipant>,
        conn: &SqliteConnection,
    ) -> QueryResult<Vec<Participant>> {
        diesel::insert_into(participant::table)
            .values(new_participants)
            .execute(conn)
            .and_then(|count| {
                participant::table
                    .order(participant::id.desc())
                    .limit(count as i64)
                    .get_results::<Participant>(conn)
            })
    }
}
