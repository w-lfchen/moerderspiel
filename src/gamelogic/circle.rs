use std::collections::HashMap;

use anyhow::anyhow;
use rand::seq::SliceRandom as _;
use serde::{Deserialize, Serialize};

use super::{
    errors::{KillingError, NotFoundError},
    player::PlayerID,
};

#[derive(Debug, Deserialize, Serialize)]
pub(super) struct Circle {
    /// every player is assigned their current status in a circle
    mapping: HashMap<PlayerID, Status>,
}

/// current status of a player in a circle
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
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

    pub fn get_target(&self) -> Option<&PlayerID> {
        match self {
            Status::Alive { target } => Some(target),
            Status::Dead { .. } => None,
        }
    }

    pub fn get_killer(&self) -> Option<&PlayerID> {
        match self {
            Status::Dead { killer } => Some(killer),
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
            Status::Alive { target } => Some(std::mem::replace(target, new_target)),
            Status::Dead { .. } => None,
        }
    }
}

impl Circle {
    /// not taking a slice of ids here because we'd have to allocate a vec for shuffling anyways
    /// there might be a way to do this without an allocation but this a premature optimization for now
    pub(super) fn new(mut ids: Vec<PlayerID>) -> Self {
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

    pub(super) fn get_status(&self, player: &PlayerID) -> Option<&Status> {
        self.mapping.get(player)
    }

    /// returns the next target
    /// TODO: this function could probably make more assumptions
    pub(super) fn attempt_to_kill(
        &mut self,
        killer: &PlayerID,
        target: &PlayerID,
    ) -> Result<PlayerID, KillingError> {
        // get killer's order
        let order = self
            .mapping
            .get(killer)
            .ok_or(NotFoundError::PlayerNotFound(*killer))?
            .get_target()
            .ok_or(KillingError::KillerIsDead(*killer))?;
        // check if target and order match
        if target != order {
            return Err(KillingError::InvalidTarget {
                killer: *killer,
                target: *target,
            });
        }
        // kill the target
        let next_target = self
            .mapping
            .get_mut(target)
            .ok_or(NotFoundError::PlayerNotFound(*target))?
            .kill(*killer)
            .ok_or(KillingError::TargetIsDead(*target))?;
        // set killer's order
        self.mapping
            .get_mut(killer)
            .ok_or(anyhow!("killer disappeared from mapping while killing"))?
            .update_target(next_target);
        // return the new target
        Ok(next_target)
    }

    /// returns the number of kills the player made in this circle. it follows that this must be 0 if the player is not known
    pub(super) fn get_kills(&self, player: &PlayerID) -> usize {
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
                    .find(|s| s.get_target().is_some_and(|t| t == player))
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

#[cfg(test)]
mod tests {
    //! tests for [Status] and [Circle]
    use uuid::Uuid;

    use super::*;

    #[test]
    fn test_status_construction() {
        let player_id = PlayerID::new(Uuid::new_v4());
        let alive = Status::Alive { target: player_id };
        let dead = Status::Dead { killer: player_id };

        assert_eq!(Some(&player_id), alive.get_target());
        assert_eq!(None, alive.get_killer());

        assert_eq!(Some(&player_id), dead.get_killer());
        assert_eq!(None, dead.get_target());
    }

    #[test]
    fn test_status_kill() {
        let target = PlayerID::new(Uuid::new_v4());
        let killer = PlayerID::new(Uuid::new_v4());

        let mut alive = Status::Alive { target };
        let mut dead = Status::Dead { killer };

        let retrieved_target = alive.kill(killer);

        assert_eq!(Some(target), retrieved_target);
        assert_eq!(dead, alive);
        assert_eq!(None, dead.kill(PlayerID::new(Uuid::new_v4())));
    }

    #[test]
    fn test_status_update() {
        let id_1 = PlayerID::new(Uuid::new_v4());
        let id_2 = PlayerID::new(Uuid::new_v4());

        let mut alive = Status::Alive { target: id_1 };
        let mut dead = Status::Dead {
            killer: PlayerID::new(Uuid::new_v4()),
        };

        let retrieved_target = alive.update_target(id_2);

        assert_eq!(Some(id_1), retrieved_target);
        assert_eq!(Status::Alive { target: id_2 }, alive);
        assert_eq!(None, dead.update_target(PlayerID::new(Uuid::new_v4())));
    }

    #[test]
    fn test_circle_creation() {
        let mut player_ids: Vec<_> = std::iter::from_fn(|| Some(PlayerID::new(Uuid::new_v4())))
            .take(10)
            .collect();
        let circle = Circle::new(player_ids.clone());

        let mut keys: Vec<_> = circle.mapping.keys().copied().collect();
        let mut values: Vec<_> = circle
            .mapping
            .values()
            .map(|status| status.get_target().expect("dead status in circle"))
            .copied()
            .collect();

        keys.sort();
        player_ids.sort();
        values.sort();

        assert_eq!(player_ids, keys);
        assert_eq!(player_ids, values);
    }

    #[test]
    fn test_circle_killing() {
        let player_ids = vec![
            PlayerID::new(Uuid::new_v4()),
            PlayerID::new(Uuid::new_v4()),
            PlayerID::new(Uuid::new_v4()),
        ];
        let mut circle = Circle::new(player_ids.clone());

        let killer = player_ids[0];
        let target = *circle
            .get_status(&killer)
            .expect("player not in circle")
            .get_target()
            .expect("dead player in circle without kills");

        let next_target = circle
            .attempt_to_kill(&killer, &target)
            .expect("killing should not fail");

        let third_player = *player_ids
            .iter()
            .find(|id| **id != killer && **id != target)
            .unwrap();

        assert_eq!(third_player, next_target);

        // assert state
        assert_eq!(
            Some(&Status::Alive {
                target: next_target
            }),
            circle.get_status(&killer)
        );
        assert_eq!(Some(&Status::Dead { killer }), circle.get_status(&target));
        assert_eq!(
            Some(&Status::Alive { target: killer }),
            circle.get_status(&third_player)
        );

        let next_target = circle
            .attempt_to_kill(&killer, &third_player)
            .expect("killing should not fail");

        assert_eq!(killer, next_target);

        // assert state
        assert_eq!(
            Some(&Status::Alive { target: killer }),
            circle.get_status(&killer)
        );
        assert_eq!(Some(&Status::Dead { killer }), circle.get_status(&target));
        assert_eq!(
            Some(&Status::Dead { killer }),
            circle.get_status(&third_player)
        );

        assert_eq!(2, circle.get_kills(&killer));
        assert_eq!(0, circle.get_kills(&target));
        assert_eq!(0, circle.get_kills(&third_player));
    }

    #[test]
    fn test_remove_player() {
        let player_1 = PlayerID::new(Uuid::new_v4());
        let player_2 = PlayerID::new(Uuid::new_v4());
        let player_3 = PlayerID::new(Uuid::new_v4());
        let mut circle = Circle::new(vec![player_1, player_2, player_3]);

        let status = *circle.get_status(&player_1).expect("player not in circle");

        let removal_status = circle
            .remove_player(&player_1)
            .expect("player not in circle");

        assert_eq!(status, removal_status);
        assert_eq!(None, circle.get_status(&player_1));
        assert_eq!(0, circle.get_kills(&player_1));

        // assert state
        assert_eq!(
            Some(&Status::Alive { target: player_3 }),
            circle.get_status(&player_2)
        );
        assert_eq!(
            Some(&Status::Alive { target: player_2 }),
            circle.get_status(&player_3)
        );
    }
}
