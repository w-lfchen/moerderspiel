use std::marker::PhantomData;

use super::{
    circle::{self, Circle},
    phase::{Finished, GamePhase, Running, Setup},
    player::Player,
};

#[derive(Debug)]
pub struct Game<T: GamePhase> {
    players: Vec<Player>,
    // circles: [Circle; circle::NUM_CIRCLES],
    phantom: PhantomData<T>,
}

impl Game<Setup> {
    pub const fn new() -> Self {
        Self {
            players: vec![],
            phantom: PhantomData,
        }
    }

    pub fn add_player(&mut self, player: Player) {
        self.players.push(player);
    }

    pub fn start(self) -> Game<Running> {
        Game {
            players: self.players,
            phantom: PhantomData,
        }
    }
}

impl Game<Running> {
    pub fn finish(self) -> Game<Finished> {
        Game {
            players: self.players,
            phantom: PhantomData,
        }
    }
}
