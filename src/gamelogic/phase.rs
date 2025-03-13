use std::{iter::repeat_with, sync::Arc};

use super::{
    circle::{self, Circle, NUM_CIRCLES},
    player::Player,
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
}

#[derive(Debug)]
pub struct Running<'a> {
    // players: Arc<[Player]>,
    circles: [circle::Circle<'a>; circle::NUM_CIRCLES],
}
impl GamePhase for Running<'_> {}

impl<'a> Running<'a> {
    pub fn new(players: &'a [Player]) -> Self {
        Self {
            //players,
            circles: repeat_with(|| Circle::new(players))
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
