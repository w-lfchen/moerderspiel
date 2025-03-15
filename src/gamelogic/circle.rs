use std::collections::HashMap;

use anyhow::anyhow;
use rand::seq::SliceRandom;

use super::{errors::KillingError, player::PlayerID};

pub(crate) const NUM_CIRCLES: usize = 3;

#[derive(Debug)]
pub(super) struct Circle {
    /// every player is assigned their current status in a circle
    mapping: HashMap<PlayerID, Status>,
}

/// current status of a player in a circle
#[derive(Debug)]
pub enum Status {
    /// the player is alive in this circle, so they have a target to kill
    Alive { target: PlayerID },
    /// the player has been killed by another player
    Dead { killer: PlayerID },
    // this might be worth tracking at some point
    // Removed,
}

impl Status {
    pub fn is_alive(&self) -> bool {
        matches!(self, Status::Alive { .. })
    }

    /* add if needed
    pub fn is_dead(&self) -> bool {
        matches!(self, Status::Dead { .. })
    }
    */

    pub fn get_target(&self) -> Option<PlayerID> {
        match self {
            Status::Alive { target } => Some(target).copied(),
            Status::Dead { .. } => None,
        }
    }

    pub fn get_killer(&self) -> Option<PlayerID> {
        match self {
            Status::Dead { killer } => Some(killer).copied(),
            Status::Alive { .. } => None,
        }
    }

    /// replaces self with [Status::Dead] and sets the killer; returns [Option::None] if already dead
    pub fn kill(&mut self, killer: PlayerID) -> Option<PlayerID> {
        match self {
            Status::Alive { target } => {
                let target = *target;
                *self = Status::Dead { killer };
                Some(target)
            }
            Status::Dead { .. } => None,
        }
    }

    /// returns the old target and sets the `target` field to `new_target` iff self is alive; returns [Option::None] otherwise
    pub fn update_target(&mut self, new_target: PlayerID) -> Option<PlayerID> {
        match self {
            Status::Alive { target } => {
                let old_target = *target;
                *target = new_target;
                Some(old_target)
            }
            Status::Dead { .. } => None,
        }
    }
}

impl Circle {
    /// not taking a slice of ids here because we'd have to allocate a vec for shuffling anyways
    /// there might be a way to do this without an allocation but this a premature optimization for now
    pub fn new(mut ids: Vec<PlayerID>) -> Self {
        // randomize order of players
        ids.shuffle(&mut rand::rng());
        // there probably is a better way to do this but this should work
        Self {
            mapping: (0..ids.len())
                .map(|i| {
                    (
                        ids[i],
                        // every player is alive at the beginning and is assigned the next person in the shuffled vector as target
                        // last person gets the first one
                        Status::Alive {
                            target: ids[(i + 1) % ids.len()],
                        },
                    )
                })
                .collect(),
        }
    }

    pub(super) fn get_target(&self, player: PlayerID) -> Option<&Status> {
        self.mapping.get(&player)
    }

    /// returns the next target
    /// TODO: this function could probably make more assumptions
    pub(super) fn attempt_to_kill(
        &mut self,
        killer: PlayerID,
        target: PlayerID,
    ) -> Result<PlayerID, KillingError> {
        // get killer's order
        let order = self
            .mapping
            .get(&killer)
            .ok_or(KillingError::PlayerNotFound(killer))?
            .get_target()
            .ok_or(KillingError::KillerIsDead(killer))?;
        // check if target and order match
        if target != order {
            return Err(KillingError::InvalidTarget { killer, target });
        }
        // kill the target
        let next_target = self
            .mapping
            .get_mut(&target)
            .ok_or(KillingError::PlayerNotFound(target))?
            .kill(killer)
            .ok_or(KillingError::TargetIsDead(target))?;
        // set killer's order
        self.mapping
            .get_mut(&killer)
            .ok_or(anyhow!("killer disappeared from mapping while killing"))?
            .update_target(next_target);
        // return the new target
        Ok(next_target)
    }

    /// returns the number of kills the player made in this circle. it follows that this must be 0 if the player is not known
    pub(super) fn get_kills(&self, player: PlayerID) -> usize {
        self.mapping
            .values()
            .filter(|status| status.get_killer().is_some_and(|killer| killer == player))
            .count()
    }

    /// removes a player from this circle.
    ///
    /// if the player is dead, simply remove them.
    /// if the player is alive, the person targeting them gets assigned the removed player's target.
    /// returns the removed player's status iff the player was in the circle
    pub(super) fn remove_player(&mut self, player: &PlayerID) -> Option<Status> {
        self.mapping.remove(player).inspect(|status| match status {
            Status::Alive { target } => {
                // find the player with an order for the removed one and update their order to the target
                self.mapping
                    .values_mut()
                    .find(|s| s.get_target().is_some_and(|t| t == *player))
                    .expect(
                        "invariant violated: unable to find player targeting removed, alive one",
                    )
                    .update_target(*target);
            }
            // do nothing if dead
            Status::Dead { .. } => (),
        })
    }
}
