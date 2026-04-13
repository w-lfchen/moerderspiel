use std::ops::Deref;

use chrono::{DateTime, Utc};
use diesel::{
    deserialize::{FromSql, FromSqlRow},
    prelude::*,
    sql_types::Integer,
    sqlite::Sqlite,
};
use serde::Deserialize;

use crate::db::schema::{games, kills, orders, players};

#[derive(Debug, Queryable, Selectable)]
#[diesel(table_name = players)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Player {
    pub id: i32,
    pub name: String,
    pub variant: i32,
    pub hashed_password: String,
}

#[derive(Debug, Deserialize, Insertable)]
#[diesel(table_name = players)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct NewPlayer {
    pub name: String,
    pub variant: i32,
    pub hashed_password: String,
}

#[derive(Debug, FromSqlRow)]
pub enum Phase {
    Setup,
    Running,
    Finished,
}

impl FromSql<Integer, Sqlite> for Phase {
    fn from_sql(
        bytes: <Sqlite as diesel::backend::Backend>::RawValue<'_>,
    ) -> diesel::deserialize::Result<Self> {
        match i32::from_sql(bytes)? {
            0 => Ok(Self::Setup),
            1 => Ok(Self::Running),
            2 => Ok(Self::Finished),
            x => Err(anyhow::anyhow!("tried to deserialize {x} as phase").into()),
        }
    }
}

#[derive(Debug, Queryable, Selectable)]
#[diesel(table_name = games)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Game {
    pub id: i32,
    pub name: String,
    pub phase: Phase,
}

#[derive(Debug, Queryable, Selectable)]
#[diesel(table_name = orders)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Order {
    pub id: i32,
    pub circle: i32,
    pub executor: i32,
    pub target: i32,
}

#[derive(Debug, Queryable, Selectable)]
#[diesel(table_name = kills)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Kill {
    pub id: i32,
    pub timestamp: TimeStamp,
    pub killer: i32,
    pub target: i32,
}

/// this type wraps a [`DateTime<Utc>`] and acts as a workaround for sqlite's lack of datetime support
///
/// we store unix timestamps in the sqlite db and interpret them as such
#[derive(Debug, FromSqlRow)]
#[repr(transparent)]
pub struct TimeStamp(DateTime<Utc>);

impl Deref for TimeStamp {
    type Target = DateTime<Utc>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl FromSql<Integer, Sqlite> for TimeStamp {
    fn from_sql(
        bytes: <Sqlite as diesel::backend::Backend>::RawValue<'_>,
    ) -> diesel::deserialize::Result<Self> {
        let unix_timestamp = i64::from_sql(bytes)?;
        DateTime::from_timestamp_secs(unix_timestamp)
            .map(Self)
            .ok_or_else(|| {
                anyhow::anyhow!("failed to deserialize {unix_timestamp} as unix timestamp").into()
            })
    }
}
