pub mod endpoints;

use std::sync::RwLock;

use serde::{Deserialize, Serialize};

use crate::gamelogic::{
    game::{Game, GameID},
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
    pub fn new(name: String) -> Self {
        Self::Setup(Game::new(name))
    }

    pub fn new_with_id(id: GameID, name: String) -> Self {
        Self::Setup(Game::new_with_id(id, name))
    }

    pub fn id(&self) -> &GameID {
        match self {
            Self::Setup(g) => g.id(),
            Self::Running(g) => g.id(),
            Self::Finished(g) => g.id(),
        }
    }

    pub fn name(&self) -> &str {
        match self {
            Self::Setup(g) => g.name(),
            Self::Running(g) => g.name(),
            Self::Finished(g) => g.name(),
        }
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
