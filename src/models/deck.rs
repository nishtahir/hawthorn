use diesel;
use diesel::prelude::*;
use models::player::Player;
use schema::deck;

#[derive(Identifiable, Queryable, Serialize, Deserialize, AsChangeset, Associations)]
#[table_name = "deck"]
#[belongs_to(Player)]
pub struct Deck {
    pub id: i32,
    pub alias: String,
    pub commander: String,
    pub player_id: i32,
    pub active: bool,
}

#[derive(Insertable)]
#[table_name = "deck"]
#[derive(Deserialize)]
pub struct NewDeck {
    pub alias: String,
    pub commander: String,
    pub player_id: i32,
    pub active: bool,
}

impl Deck {
    pub fn all(conn: &SqliteConnection) -> QueryResult<Vec<Deck>> {
        deck::table.order(deck::id).load::<Deck>(conn)
    }

    pub fn find_by_id(id: i32, conn: &SqliteConnection) -> QueryResult<Deck> {
        deck::table.find(id).get_result::<Deck>(conn)
    }

    pub fn find_by_player(player: &Player, conn: &SqliteConnection) -> QueryResult<Vec<Deck>> {
        Deck::belonging_to(player).load::<Deck>(conn)
    }

    pub fn find_by_players(
        players: Vec<Player>,
        conn: &SqliteConnection,
    ) -> QueryResult<Vec<(Player, Vec<Deck>)>> {
        let decks = Deck::belonging_to(&players)
            .load::<Deck>(conn)?
            .grouped_by(&players);
        Ok(players.into_iter().zip(decks).collect::<Vec<_>>())
    }

    pub fn update(deck: Deck, conn: &SqliteConnection) -> QueryResult<Deck> {
        diesel::update(deck::table.find(deck.id))
            .set(&deck)
            .execute(conn)
            .and_then(|_| deck::table.find(deck.id).get_result::<Deck>(conn))
    }
}

impl NewDeck {
    pub fn insert(deck: NewDeck, conn: &SqliteConnection) -> QueryResult<Deck> {
        // Diesel doesn't expose a get result method
        diesel::insert_into(deck::table)
            .values(&deck)
            .execute(conn)
            .and_then(|_| deck::table.order(deck::id.desc()).first(conn))
    }
}
