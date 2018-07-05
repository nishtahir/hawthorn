use diesel;
use diesel::prelude::*;
use models::deck::Deck;
use models::game::Game;
use schema::participant;

#[derive(Identifiable, Queryable, Serialize, Deserialize, Associations)]
#[table_name = "participant"]
#[belongs_to(Game)]
#[belongs_to(Deck)]
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

impl Participant {
    pub fn find_by_game(game: &Game, conn: &SqliteConnection) -> QueryResult<Vec<Participant>> {
        Participant::belonging_to(game).load::<Participant>(conn)
    }

    pub fn find_by_deck(deck: &Deck, conn: &SqliteConnection) -> QueryResult<Vec<Participant>> {
        Participant::belonging_to(deck).load::<Participant>(conn)
    }
}

impl NewParticipant {
    pub fn new(game_id: i32, deck_id: i32, win: bool) -> NewParticipant {
        NewParticipant {
            deck_id,
            game_id,
            win,
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
