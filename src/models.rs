use diesel;
use diesel::prelude::*;
use schema::player;
use schema::deck;

#[derive(Identifiable, Queryable, Serialize, Deserialize)]
#[table_name = "player"]
#[has_many(decks)]
pub struct Player {
    pub id: i32,
    pub alias: String,
    pub win: i32,
    pub loss: i32,
}

#[derive(Identifiable, Queryable, Serialize, Deserialize)]
#[table_name = "deck"]
#[belongs_to(Player)]
pub struct Deck {
    pub id: i32,
    pub alias: String,
    pub player_id: i32
}

#[derive(Insertable)]
#[table_name = "player"]
#[derive(Deserialize)]
pub struct NewPlayer {
    pub alias: String,

    #[serde(default)]
    pub win: i32,

    #[serde(default)]
    pub loss: i32,
}

impl NewPlayer {
    fn from_player(player: Player) -> NewPlayer {
        NewPlayer {
            alias: player.alias,
            win: 0,
            loss: 0,
        }
    }
}

impl Player {
    pub fn all_players(conn: &SqliteConnection) -> QueryResult<Vec<Player>> {
        player::table.order(player::id).load::<Player>(conn)
    }

    pub fn find_player(id: i32, conn: &SqliteConnection) -> QueryResult<Player> {
        player::table.find(id).get_result::<Player>(conn)
    }

    pub fn insert_player(player: NewPlayer, conn: &SqliteConnection) -> QueryResult<Player> {
        // Diesel doesn't expose a get result method
        diesel::insert_into(player::table)
            .values(&player)
            .execute(conn)
            .and_then(|_| player::table.order(player::id.desc()).first(conn))
    }
}
