use std::{collections::HashMap, iter::repeat_with};

use uuid::Uuid;

use super::{
    circle::{self, Circle, NUM_CIRCLES, Status},
    errors::{KillingError, NotFoundError},
    player::{NewPlayerData, Player, PlayerID},
};

/// trait for keeping track of a game's state
pub trait GamePhase {}

#[derive(Debug)]
pub struct Setup {
    players: Vec<Player>,
}
impl GamePhase for Setup {}

impl Setup {
    pub const fn new() -> Self {
        Self {
            players: Vec::new(),
        }
    }

    /// returns the id of the newly inserted player as that is all the new information this method creates
    pub fn add_player(&mut self, player_data: NewPlayerData) -> PlayerID {
        let new_id = PlayerID::new(Uuid::new_v4());
        self.players
            .push(Player::new_from_data(new_id, player_data));
        new_id
    }

    /// remove a player by id; returns the player if the operation was successful
    pub fn remove_player(&mut self, player: &PlayerID) -> Option<Player> {
        self.players
            .iter()
            // maybe we should store players in a map from the beginning
            .enumerate()
            // there should only be one instance of a given player with the given uuid
            .filter_map(|(index, entry)| (entry.id().uuid() == player.uuid()).then_some(index))
            .next()
            // remove the player if found
            .map(|i| self.players.remove(i))
    }

    pub(super) fn into_players(self) -> Vec<Player> {
        self.players
    }
}

#[derive(Debug)]
pub struct Running {
    // constant time lookup for player ids instead of linear time through an arc or vec
    players: HashMap<PlayerID, Player>,
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
    /// TODO: decide what to do for the case of final kill (killer gets own id as the next target)
    pub fn attempt_to_kill(
        &mut self,
        killer: &PlayerID,
        target: &PlayerID,
    ) -> Result<PlayerID, KillingError> {
        // check whether killer can attempt to kill
        match self.get_lives(killer) {
            None => return Err(NotFoundError::PlayerNotFound(*killer).into()),
            Some(0) => return Err(KillingError::KillerIsDead(*killer)),
            Some(_) => (),
        }
        // attempt to get the index of a circle where the target is assigned to the killer
        if let Some(circle_index) = self
            .circles
            .iter()
            .enumerate()
            .filter_map(|(i, c)| {
                match c
                    .get_status(killer)
                    .expect("invariant was not upheld: killer id not found")
                {
                    Status::Alive { target: order } if order == target => Some(i),
                    _ => None,
                }
            })
            // take the first ring where the target matches even if there might be multiple rings
            .next()
        {
            // do the killing
            self.circles[circle_index].attempt_to_kill(killer, target)
        } else {
            // amount of lives has been checked so target must be invalid if index was not found
            Err(KillingError::InvalidTarget {
                killer: *killer,
                target: *target,
            })
        }
    }

    /// none if given an unknown player id
    fn get_lives(&self, player: &PlayerID) -> Option<usize> {
        self.players.contains_key(player).then(|| {
            self.circles
                .iter()
                .filter(|c| {
                    c.get_status(player)
                        .expect("invariant violated: player is known but not present in circle")
                        .is_alive()
                })
                .count()
        })
    }

    /// if the player id is valid, returns the number of players killed by the given player
    pub fn get_kills(&self, player: &PlayerID) -> Option<usize> {
        // check for id presence
        self.players
            .contains_key(player)
            .then(|| self.circles.iter().map(|c| c.get_kills(player)).sum())
    }

    pub fn remove_player(&mut self, player: &PlayerID) -> Option<Player> {
        self.players.remove(player).inspect(|_| {
            self.circles.iter_mut().for_each(|c| {
                // we currently do not care about the status of the removed player
                let _ = c.remove_player(player);
            })
        })
    }

    pub fn get_status(&self, player: &PlayerID, index: usize) -> Result<&Status, NotFoundError> {
        self.circles
            .get(index)
            .ok_or(NotFoundError::CircleNotFound(index))?
            .get_status(player)
            .ok_or(NotFoundError::PlayerNotFound(*player))
    }
}

#[derive(Debug)]
pub struct Finished {}
impl GamePhase for Finished {}
