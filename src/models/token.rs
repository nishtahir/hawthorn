use db::DbConn;
use models::{Insertable, Retrievable};

#[derive(Identifiable, Queryable, Serialize, Deserialize, Associations, AsChangeset)]
#[table_name = "token"]
pub struct Token {
    id: i32,
    player_id: i32,
    content: String,
}
#[derive(Insertable)]
#[table_name = "token"]
pub struct NewToken {
    player_id: i32,
    content: String,
}

impl Retrievable for Token {
    fn all(conn: &SqliteConnection) -> QueryResult<Vec<Token>> {
        token::table.order(token::id).load::<Token>(conn)
    }

    fn find(id: i32, conn: &SqliteConnection) -> QueryResult<Token> {
        token::table.find(id).get_result::<Token>(conn)
    }

    fn update(token: Token, conn: &SqliteConnection) -> QueryResult<Token> {
        diesel::update(token::table.find(token.id))
            .set(&token)
            .execute(conn)
            .and_then(|_| token::table.find(token.id).get_result::<Token>(conn))
    }

    fn delete(id: i32, conn: &SqliteConnection) -> bool {
        diesel::delete(token::table.find(id)).execute(conn).is_ok()
    }
}

impl Insertable for Token {
    fn insert(token: NewToken, conn: &SqliteConnection) -> QueryResult<Token> {
        // Diesel doesn't expose a get result method
        diesel::insert_into(token::table)
            .values(&token)
            .execute(conn)
            .and_then(|_| token::table.order(token::id.desc()).first(conn))
    }
}
