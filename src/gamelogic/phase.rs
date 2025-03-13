use std::iter::repeat_with;

use super::{
    circle::{self, Circle, NUM_CIRCLES},
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
    // players: Arc<[Player]>,
    circles: [circle::Circle; circle::NUM_CIRCLES],
}
impl GamePhase for Running {}

impl Running {
    pub fn new(players: Vec<Player>) -> Self {
        Self {
            // players,
            // this could probably be replaced with a macro
            circles: repeat_with(|| Circle::new(players.clone()))
                .take(NUM_CIRCLES)
                .collect::<Vec<_>>()
                .try_into()
                .expect("circle array creation should not fail"),
        }
    }
}

#[derive(Debug)]
pub struct Finished {}
impl GamePhase for Finished {}
