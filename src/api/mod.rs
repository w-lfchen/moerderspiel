pub mod endpoints;

use std::{collections::HashMap, sync::RwLock};

use serde::{Deserialize, Serialize};

use crate::gamelogic::{
    NUM_CIRCLES,
    game::{Game, GameID, NewGameData},
    phase::{Finished, Running, Setup},
    player::Player,
};

#[derive(Debug, Deserialize, Serialize)]
#[serde(tag = "phase")]
pub enum GameState {
    Setup(Game<Setup>),
    Running(Game<Running>),
    Finished(Game<Finished>),
}

impl GameState {
    pub fn from_data(data: NewGameData) -> Self {
        Self::Setup(Game::from_data(data))
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

    pub fn get_data(&self) -> GameData {
        match self {
            GameState::Setup(game) => GameData {
                id: game.id(),
                name: game.name(),
                num_circles: NUM_CIRCLES,
                phase: "setup",
                players: game.players().collect(),
            },
            GameState::Running(game) => GameData {
                id: game.id(),
                name: game.name(),
                num_circles: NUM_CIRCLES,
                phase: "running",
                players: game.players().collect(),
            },
            GameState::Finished(game) => GameData {
                id: game.id(),
                name: game.name(),
                num_circles: NUM_CIRCLES,
                phase: "finished",
                players: vec![], // TODO
            },
        }
    }
}

#[derive(Debug)]
pub struct State {
    // hashmap for easy, constant time lookup of games
    games: RwLock<HashMap<GameID, GameState>>,
}

impl State {
    pub fn new() -> Self {
        Self {
            games: RwLock::new(HashMap::new()),
        }
    }
}

/// struct for outgoing data about a game
///
/// doesn't include phase data except the players
#[derive(Debug, Serialize)]
pub struct GameData<'a> {
    id: &'a GameID,
    name: &'a str,
    num_circles: usize,
    phase: &'a str,
    players: Vec<&'a Player>,
}
