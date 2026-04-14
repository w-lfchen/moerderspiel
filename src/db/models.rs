use std::ops::Deref;

use chrono::{DateTime, Utc};
use diesel::{
    backend::Backend,
    deserialize::{FromSql, FromSqlRow},
    expression::AsExpression,
    prelude::*,
    serialize::{Output, ToSql},
    sql_types::Integer,
    sqlite::Sqlite,
};
use serde::{Deserialize, Serialize};

use crate::{
    CONFIG,
    db::schema::{games, kills, orders, players},
};

#[derive(Clone, Debug, Queryable, Selectable)]
#[diesel(table_name = players)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Player {
    pub id: i32,
    pub name: String,
    pub variant: i32,
    pub hashed_password: String,
}

/// incoming player struct
#[derive(Debug, Deserialize, Insertable)]
#[diesel(table_name = players)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct PlayerData {
    pub name: String,
    pub variant: i32,
    pub hashed_password: String,
}

impl PlayerData {
    /// we should only receive hashed passwords, but we hash them again before storing them into the db
    pub fn hash_password_in_place(self) -> Result<Self, anyhow::Error> {
        use argon2::{
            Argon2,
            password_hash::{PasswordHasher, SaltString, rand_core::OsRng},
        };
        let salt = SaltString::generate(&mut OsRng);
        // default parameters should be fine
        let argon2 = Argon2::default();
        let hashed_password = argon2
            .hash_password((self.hashed_password + CONFIG.pepper()).as_bytes(), &salt)?
            .to_string();
        Ok(Self {
            hashed_password,
            ..self
        })
    }
}

#[derive(AsExpression, Debug, Eq, FromSqlRow, PartialEq, Serialize)]
#[diesel(sql_type = Integer)]
#[repr(i32)]
pub enum Phase {
    Setup = 0,
    Running = 1,
    Finished = 2,
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

// there might be room for improvement here but this works for now
impl<DB> ToSql<Integer, DB> for Phase
where
    DB: Backend,
    i32: ToSql<Integer, DB>,
{
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, DB>) -> diesel::serialize::Result {
        match self {
            Phase::Setup => 0.to_sql(out),
            Phase::Running => 1.to_sql(out),
            Phase::Finished => 2.to_sql(out),
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

#[derive(Debug, Insertable)]
#[diesel(table_name = games)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct NewGame<'a> {
    pub name: &'a str,
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

#[derive(Clone, Copy, Debug, Insertable)]
#[diesel(table_name = orders)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
#[diesel(treat_none_as_default_value = false)]
pub struct NewOrder {
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
