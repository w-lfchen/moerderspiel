use thiserror::Error;

use super::player::ID;

#[derive(Debug, Error)]
pub enum KillingError {
    #[error("no killer with id {0} found")]
    KillerNotFound(ID),
    #[error("killer with id {0} is dead and can't kill")]
    KillerIsDead(ID),
    #[error("killer {killer} does not have an order for {target}")]
    InvalidTarget { killer: ID, target: ID },
    #[error(transparent)]
    ErrorDuringKilling(#[from] anyhow::Error),
}
