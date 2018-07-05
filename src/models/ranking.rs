use diesel;
use diesel::prelude::*;
use elo::{GameOutcome, RankableEntity};
use models::deck::Deck;
use models::participant::Participant;
use models::{Insertable, Retrievable};
use schema::ranking;

#[derive(Identifiable, Queryable, Serialize, Deserialize, AsChangeset, Associations)]
#[belongs_to(Deck)]
#[table_name = "ranking"]
pub struct Ranking {
    pub id: i32,
    pub deck_id: i32,
    pub elo: f64,
}

#[derive(Insertable, AsExpression)]
#[table_name = "ranking"]
pub struct NewRanking {
    pub elo: f64,
    pub deck_id: i32,
}

impl Retrievable for Ranking {
    fn all(conn: &SqliteConnection) -> QueryResult<Vec<Ranking>> {
        ranking::table.order(ranking::id).load::<Ranking>(conn)
    }

    fn find(id: i32, conn: &SqliteConnection) -> QueryResult<Ranking> {
        ranking::table.find(id).get_result::<Ranking>(conn)
    }

    fn update(ranking: Self, conn: &SqliteConnection) -> QueryResult<Self> {
        diesel::update(ranking::table.find(ranking.id))
            .set(&ranking)
            .execute(conn)
            .and_then(|_| ranking::table.find(ranking.id).get_result::<Ranking>(conn))
    }

    fn delete(id: i32, conn: &SqliteConnection) -> bool {
        diesel::delete(ranking::table.find(id))
            .execute(conn)
            .is_ok()
    }
}

impl Ranking {
    pub fn from_participant(
        request: &Participant,
        conn: &SqliteConnection,
    ) -> QueryResult<Ranking> {
        Deck::find(request.deck_id, conn)
            .and_then(|deck| Ranking::get_latest_ranking_by_deck(&deck, conn))
    }

    fn get_latest_ranking_by_deck(deck: &Deck, conn: &SqliteConnection) -> QueryResult<Ranking> {
        let ranking = Ranking::belonging_to(deck)
            .order(ranking::id.desc())
            .first(conn);

        match ranking {
            Ok(value) => Ok(value),
            // if one doesn't exist - Create a new ranking for it with a base score
            Err(_) => NewRanking::insert(NewRanking::new(deck.id), conn),
        }
    }

    pub fn to_rankable_entity(&self, win: bool) -> RankableEntity {
        RankableEntity::new(
            self.deck_id,
            self.elo,
            if win {
                GameOutcome::WIN
            } else {
                GameOutcome::LOSE
            },
        )
    }
}

impl Insertable for NewRanking {
    type T = Ranking;

    fn insert(ranking: NewRanking, conn: &SqliteConnection) -> QueryResult<Ranking> {
        // Diesel doesn't expose a get result method
        diesel::insert_into(ranking::table)
            .values(&ranking)
            .execute(conn)
            .and_then(|_| ranking::table.order(ranking::id.desc()).first(conn))
    }
}

impl NewRanking {
    fn new(deck_id: i32) -> NewRanking {
        NewRanking {
            deck_id: deck_id,
            elo: 1000.0,
        }
    }

    pub fn from_rankable_entity(rankable_entity: &RankableEntity) -> NewRanking {
        NewRanking {
            deck_id: rankable_entity.id,
            elo: rankable_entity.elo,
        }
    }

    pub fn insert_all(
        new_rankings: &Vec<NewRanking>,
        conn: &SqliteConnection,
    ) -> QueryResult<Vec<Ranking>> {
        diesel::insert_into(ranking::table)
            .values(new_rankings)
            .execute(conn)
            .and_then(|count| {
                ranking::table
                    .order(ranking::id.desc())
                    .limit(count as i64)
                    .get_results::<Ranking>(conn)
            })
    }
}
