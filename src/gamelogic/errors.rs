use thiserror::Error;

use super::player::PlayerID;

#[derive(Debug, Error)]
pub enum KillingError {
    #[error("killer {killer} does not have an order for {target}")]
    InvalidTarget { killer: PlayerID, target: PlayerID },
    #[error("killer with id {0} is dead and can't kill")]
    KillerIsDead(PlayerID),
    #[error("no player with id {0} found in circle")]
    PlayerNotFound(PlayerID),
    #[error("target with id {0} is akready dead and can't be killed")]
    TargetIsDead(PlayerID),
    #[error(transparent)]
    ErrorDuringKilling(#[from] anyhow::Error),
}
