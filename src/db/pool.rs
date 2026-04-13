//! database pool handling
use diesel_async::{
    pooled_connection::{AsyncDieselConnectionManager, PoolError},
    sync_connection_wrapper::SyncConnectionWrapper,
};

use crate::config::Config;

type SqliteConnWrapped = SyncConnectionWrapper<diesel::SqliteConnection>;
pub type Pool = diesel_async::pooled_connection::bb8::Pool<SqliteConnWrapped>;

/// set up the db connection pool
pub async fn setup_pool(config: &Config) -> Result<Pool, anyhow::Error> {
    let manager = AsyncDieselConnectionManager::<SqliteConnWrapped>::new(config.db_url());
    Pool::builder()
        .connection_customizer(Box::new(SqliteConnectionInit))
        .build(manager)
        .await
        .map_err(Into::into)
}

/// improve sqlite connection concurrency. see:
/// - https://docs.rs/diesel/2.3.7/diesel/prelude/struct.SqliteConnection.html#concurrency
/// - https://stackoverflow.com/questions/57123453/how-to-use-diesel-with-sqlite-connections-and-avoid-database-is-locked-type-of
#[derive(Debug)]
struct SqliteConnectionInit;

impl bb8::CustomizeConnection<SqliteConnWrapped, PoolError> for SqliteConnectionInit {
    fn on_acquire<'a>(
        &'a self,
        _connection: &'a mut SqliteConnWrapped,
    ) -> std::pin::Pin<Box<dyn Future<Output = Result<(), PoolError>> + Send + 'a>> {
        Box::pin(async {
            run_init_pragmas(_connection)
                .await
                .map_err(PoolError::QueryError)
        })
    }
}

/// sets up a connection
async fn run_init_pragmas(connection: &mut SqliteConnWrapped) -> Result<(), diesel::result::Error> {
    use diesel_async::SimpleAsyncConnection as _;
    connection
        .batch_execute("PRAGMA busy_timeout = 2000;")
        .await?;
    connection
        .batch_execute("PRAGMA journal_mode = WAL;")
        .await?;
    connection
        .batch_execute("PRAGMA synchronous = NORMAL;")
        .await?;
    connection
        .batch_execute("PRAGMA wal_autocheckpoint = 1000;")
        .await?;
    connection
        .batch_execute("PRAGMA wal_checkpoint(TRUNCATE);")
        .await
}
