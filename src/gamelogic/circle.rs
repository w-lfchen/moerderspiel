use std::collections::HashMap;

use anyhow::{Context, bail};
use rand::seq::SliceRandom;

use super::player::{ID, Player};

pub(crate) const NUM_CIRCLES: usize = 3;

#[derive(Debug)]
pub(super) struct Circle {
    // constant time lookup for player ids
    players: HashMap<ID, Player>,
    // maps player to target
    // if the target is none, the player has been killed in this circle as that is the only reason why someone would lack a target
    mapping: HashMap<ID, Option<ID>>,
}

impl Circle {
    pub fn new(mut players: Vec<Player>) -> Self {
        // randomize order of players
        players.shuffle(&mut rand::rng());
        // there probably is a better way to do this but this should work
        Self {
            mapping: (0..players.len())
                .map(|i| {
                    (
                        players[i].id(),
                        // should always succeed, giving the some(id) we want
                        players.get((i + 1) % players.len()).map(Player::id),
                    )
                })
                .collect::<HashMap<_, _>>(),
            players: players.into_iter().map(|p| (p.id(), p)).collect(),
        }
    }

    /// returns the next target
    /// TODO: decide what to do for the case of final kill (killer gets own id as the next target)
    pub(super) fn attempt_to_kill(&mut self, killer: ID, target: ID) -> anyhow::Result<ID> {
        // get killer's order
        let order = self
            .mapping
            .get(&killer)
            .with_context(|| format!("killer {killer:?} not found in mapping"))?;
        // check if target and order match
        if target != order.with_context(|| format!("killer {killer:?} is dead"))? {
            bail!("killer {killer:?} attempted to kill {target:?}, but their target is {order:?}")
        }
        // kill the target by removing their order
        let next_order = self
            .mapping
            .get_mut(&target)
            .with_context(|| format!("target {target:?} not found in mapping"))?
            .take()
            .with_context(|| format!("target {target:?} was already dead"))?;
        // set killer's order
        self.mapping
            .get_mut(&killer)
            .with_context(|| {
                format!("killer {killer:?} not found in mapping after they were found before")
            })?
            .replace(next_order);
        Ok(next_order)
    }
}
