use std::{collections::HashMap, iter::repeat_with};

use super::{
    circle::{self, Circle, NUM_CIRCLES},
    errors::KillingError,
    player::{ID, NewPlayerData, Player},
};

/// trait for keeping track of a game's state
pub trait GamePhase {}

#[derive(Debug)]
pub struct Setup {
    pub players: Vec<Player>,
}
impl GamePhase for Setup {}

impl Setup {
    pub const fn new() -> Self {
        Self { players: vec![] }
    }

    pub fn add_player(&mut self, player_data: NewPlayerData) {
        let new_id = ID(self.players.len() as u32);
        self.players
            .push(Player::new_from_data(new_id, player_data))
    }
}

#[derive(Debug)]
pub struct Running {
    // constant time lookup for player ids instead of linear time through an arc or vec
    players: HashMap<ID, Player>,
    circles: [circle::Circle; circle::NUM_CIRCLES],
}
impl GamePhase for Running {}

impl Running {
    pub fn new(players: Vec<Player>) -> Self {
        let ids = players.iter().map(Player::id).collect::<Vec<_>>();
        Self {
            // this could probably be replaced with a macro
            circles: repeat_with(|| Circle::new(ids.clone()))
                .take(NUM_CIRCLES)
                .collect::<Vec<_>>()
                .try_into()
                .expect("circle array creation should not fail"),
            players: players.into_iter().map(|p| (p.id(), p)).collect(),
        }
    }

    /// returns the next target
    pub fn attempt_to_kill(&mut self, killer: &ID, target: &ID) -> Result<ID, KillingError> {
        // check whether killer can attempt to kill
        match self.get_lives(killer) {
            None => return Err(KillingError::KillerNotFound(*killer)),
            Some(0) => return Err(KillingError::KillerIsDead(*killer)),
            Some(_) => (),
        }
        // attempt to get the index of a circle where the target is assigned to the killer
        if let Some(circle_index) = self
            .circles
            .iter()
            .enumerate()
            .filter_map(|(i, c)| {
                c.get_target(killer)
                    .expect("invariant was not upheld: killer id not found")
                    .filter(|id| id == target)
                    .is_some()
                    .then_some(i)
            })
            // take the first ring where the target matches even if there might be multiple rings
            .next()
        {
            // do the killing
            self.circles[circle_index]
                .attempt_to_kill(killer, target)
                .map_err(Into::into)
        } else {
            // amount of lives has been checked so target must be invalid if index was not found
            Err(KillingError::InvalidTarget {
                killer: *killer,
                target: *target,
            })
        }
    }

    /// none if given an unknown player id
    fn get_lives(&self, player: &ID) -> Option<usize> {
        self.players.contains_key(player).then(|| {
            self.circles
                .iter()
                .filter(|c| {
                    c.get_target(player)
                        .expect("invariant violated: player is known but not present in circle")
                        .is_some()
                })
                .count()
        })
    }

    pub(crate) fn get_first_target(&self, player: &ID) -> Option<Option<ID>> {
        self.circles[0].get_target(player)
    }
}

#[derive(Debug)]
pub struct Finished {}
impl GamePhase for Finished {}
