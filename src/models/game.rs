use diesel;
use diesel::prelude::*;
use models::{Insertable, Retrievable};
use schema::game;
use time;

#[derive(Identifiable, Queryable, Serialize, Deserialize, Associations, AsChangeset)]
#[table_name = "game"]
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
    pub fn new(timestamp: &Option<i32>) -> NewGame {
        let millis = match timestamp {
            Some(value) => *value as f64,
            None => {
                let timespec = time::get_time();
                timespec.sec as f64 + (timespec.nsec as f64 / 1000.0 / 1000.0 / 1000.0)
            }
        };

        NewGame { time_stamp: millis }
    }
}
