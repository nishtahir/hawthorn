use diesel;
use diesel::prelude::*;
use models::{Insertable, Retrievable};
use schema::player;

#[derive(Identifiable, Queryable, Serialize, Deserialize, AsChangeset, Associations)]
#[table_name = "player"]
pub struct Player {
    pub id: i32,
    pub alias: String,
    pub email: String,
    pub password: String,
}

#[derive(Insertable)]
#[table_name = "player"]
#[derive(Deserialize)]
pub struct NewPlayer {
    pub alias: String,
    pub email: String,
    pub password: String,
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

    fn insert(player: NewPlayer, conn: &SqliteConnection) -> QueryResult<Player> {
        // Diesel doesn't expose a get result method
        diesel::insert_into(player::table)
            .values(&player)
            .execute(conn)
            .and_then(|_| player::table.order(player::id.desc()).first(conn))
    }
}
