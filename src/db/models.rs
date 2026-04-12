use diesel::prelude::*;

use crate::db::schema::players;

#[derive(Debug, Queryable, Selectable)]
#[diesel(table_name = players)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Player {
    pub id: i32,
    pub name: String,
    pub variant: i32,
    pub hashed_password: String,
}

#[derive(Insertable)]
#[diesel(table_name = players)]
pub struct NewPlayer<'a> {
    pub name: &'a str,
    pub variant: i32,
    pub hashed_password: &'a str,
}
