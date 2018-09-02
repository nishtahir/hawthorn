use diesel;
use diesel::prelude::*;
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

impl Player {
    pub fn all(conn: &SqliteConnection) -> QueryResult<Vec<Player>> {
        player::table.order(player::id).load::<Player>(conn)
    }

    pub fn find_by_id(id: i32, conn: &SqliteConnection) -> QueryResult<Player> {
        player::table.find(id).get_result::<Player>(conn)
    }

    pub fn find_by_email(email: &String, conn: &SqliteConnection) -> QueryResult<Player> {
        player::table
            .order(player::email)
            .filter(player::email.eq(email))
            .first(conn)
    }

    pub fn update(player: Self, conn: &SqliteConnection) -> QueryResult<Self> {
        diesel::update(player::table.find(player.id))
            .set(&player)
            .execute(conn)
            .and_then(|_| player::table.find(player.id).get_result::<Player>(conn))
    }

    pub fn delete(id: i32, conn: &SqliteConnection) -> bool {
        diesel::delete(player::table.find(id)).execute(conn).is_ok()
    }

    pub fn update_password(
        self,
        password_hash: String,
        conn: &SqliteConnection,
    ) -> QueryResult<Player> {
        let new_player = Player {
            id: self.id,
            alias: self.alias,
            email: self.email,
            password: password_hash,
        };
        Player::update(new_player, conn)
    }
}

impl NewPlayer {
    pub fn insert(player: NewPlayer, conn: &SqliteConnection) -> QueryResult<Player> {
        // Diesel doesn't expose a get result method
        diesel::insert_into(player::table)
            .values(&player)
            .execute(conn)
            .and_then(|_| player::table.order(player::id.desc()).first(conn))
    }
}
