use std::collections::HashMap;

use anyhow::Context;
use rand::seq::IteratorRandom;

use super::player::Player;

pub(crate) const NUM_CIRCLES: usize = 3;

#[derive(Debug)]
pub(super) struct Circle<'a> {
    // maps player to target
    // if the target is none, the player has been killed in this circle as that is the only reason why someone would lack a target
    mapping: HashMap<&'a Player, Option<&'a Player>>,
}

impl<'a> Circle<'a> {
    pub fn new(players: &'a [Player]) -> Self {
        let shuffled_players = players
            .iter()
            .choose_multiple(&mut rand::rng(), players.len());
        // there probably is a better way to do this but this should work
        Self {
            mapping: (0..shuffled_players.len())
                .map(|i| {
                    (
                        shuffled_players[i],
                        // should always succeed
                        shuffled_players
                            .get((i + 1) % shuffled_players.len())
                            .copied(),
                    )
                })
                .collect::<HashMap<_, _>>(),
        }
    }

    pub(super) fn attempt_to_kill(
        &mut self,
        killer: &Player,
        target: &Player,
    ) -> anyhow::Result<&Player> {
        self.mapping
            .get_mut(killer)
            .with_context(|| format!("unable to find player in mapping: {killer:?}"))?
            .take_if(|potential_target| *potential_target == target)
            .context("target incorrect")
    }
}
