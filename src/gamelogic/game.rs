use std::marker::PhantomData;

use super::{
    circle::{self, Circle},
    phase::{Finished, GamePhase, Running, Setup},
    player::Player,
};

#[derive(Debug)]
pub struct Game<P: GamePhase> {
    // players: Vec<Player>,
    // circles: [Circle; circle::NUM_CIRCLES],
    phase: P,
}

impl Game<Setup> {
    pub const fn new() -> Self {
        Self {
            // players: vec![],
            phase: Setup::new(),
        }
    }

    pub fn add_player(&mut self, player: Player) {
        self.phase.players.push(player);
    }

    pub fn start<'a>(self, _: &'a ()) -> Game<Running<'a>> {
        Game {
            // players: self.players,
            phase: Running::new(&self.phase.players),
        }
    }
}

impl Game<Running<'_>> {
    fn new(players: Vec<Player>) -> Self {
        todo!()
    }

    pub fn finish(self) -> Game<Finished> {
        Game {
            //players: self.players,
            phase: Finished {},
        }
    }
}
