use thiserror::Error;

use super::player::ID;

#[derive(Debug, Error)]
pub enum KillingError {
    #[error("killer {killer} does not have an order for {target}")]
    InvalidTarget { killer: ID, target: ID },
    #[error("killer with id {0} is dead and can't kill")]
    KillerIsDead(ID),
    #[error("no player with id {0} found in circle")]
    PlayerNotFound(ID),
    #[error("target with id {0} is akready dead and can't be killed")]
    TargetIsDead(ID),
    #[error(transparent)]
    ErrorDuringKilling(#[from] anyhow::Error),
}
