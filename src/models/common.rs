use diesel::prelude::*;
use std::marker::Sized;

pub trait Retrievable
where
    Self: Sized,
{
    fn all(conn: &SqliteConnection) -> QueryResult<Vec<Self>>;
    fn find(id: i32, conn: &SqliteConnection) -> QueryResult<Self>;
    fn update(player: Self, conn: &SqliteConnection) -> QueryResult<Self>;
    fn delete(id: i32, conn: &SqliteConnection) -> bool;
}

pub trait Insertable
where
    Self: Sized,
{
    type T: Retrievable;
    fn insert(model: Self, conn: &SqliteConnection) -> QueryResult<Self::T>;
}
