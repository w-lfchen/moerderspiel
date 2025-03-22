use thiserror::Error;

use super::player::PlayerID;

/// errors that happen when trying to kill someone
#[derive(Debug, Error)]
pub enum KillingError {
    #[error("killer {killer} does not have an order for {target}")]
    InvalidTarget { killer: PlayerID, target: PlayerID },
    #[error("killer with id {0} is dead and can't kill")]
    KillerIsDead(PlayerID),
    #[error("target with id {0} is akready dead and can't be killed")]
    TargetIsDead(PlayerID),
    #[error(transparent)]
    PlayerNotFound(#[from] NotFoundError),
    #[error(transparent)]
    ErrorDuringKilling(#[from] anyhow::Error),
}

#[derive(Debug, Error)]
pub enum NotFoundError {
    #[error("no circle with index {0} found")]
    CircleNotFound(usize),
    #[error("no player with id {0} found")]
    PlayerNotFound(PlayerID),
}
