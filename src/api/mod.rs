pub mod endpoints;

use std::sync::RwLock;

use serde::{Deserialize, Serialize};

use crate::gamelogic::{
    game::Game,
    phase::{Finished, Running, Setup},
};

#[derive(Debug, Deserialize, Serialize)]
#[serde(tag = "phase")]
pub enum GameState {
    Setup(Game<Setup>),
    Running(Game<Running>),
    Finished(Game<Finished>),
}

impl GameState {
    pub fn new() -> Self {
        Self::Setup(Game::new())
    }
}

#[derive(Debug)]
pub struct State {
    gamestate: RwLock<GameState>,
}

impl State {
    pub fn new(gamestate: GameState) -> Self {
        Self {
            gamestate: RwLock::new(gamestate),
        }
    }
}
