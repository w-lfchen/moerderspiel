//! helpers and guards
use actix_web::error::{ErrorConflict, ErrorInternalServerError};
use diesel_async::{RunQueryDsl, pooled_connection::bb8::PooledConnection};

use crate::{
    CONFIG,
    db::{
        models::{Game, Phase},
        pool::SqliteConnWrapped,
        schema::games,
    },
};

/// db query and then either an error with conflict or unit to serve as gate
///
/// internal server error if query fails
pub(super) async fn assert_phase(
    conn: &mut PooledConnection<'_, SqliteConnWrapped>,
    phase: Phase,
) -> actix_web::Result<()> {
    let game = games::table
        .get_result::<Game>(conn)
        .await
        .map_err(ErrorInternalServerError)?;

    if game.phase != phase {
        Err(ErrorConflict(anyhow::anyhow!(
            "game needs to be in phase {phase:?} for operation, is in phase {:?}",
            game.phase
        )))
    } else {
        Ok(())
    }
}

/// !! might block !!
/// verify an unpeppered password
///
/// we take the password as [String] since we need the allocation to push the pepper
pub(super) fn verify_password(hash: &str, pw: String) -> Result<(), anyhow::Error> {
    use argon2::{
        Argon2,
        password_hash::{PasswordVerifier, phc::PasswordHash},
    };

    let hash = PasswordHash::new(hash)?;
    Argon2::default()
        .verify_password((pw + CONFIG.pepper()).as_bytes(), &hash)
        .map_err(Into::into)
}
