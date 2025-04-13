use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::{
    circle::Status,
    errors::{KillingError, NotFoundError},
    phase::{Finished, GamePhase, Running, Setup},
    player::{NewPlayerData, Player, PlayerData, PlayerID},
};

#[derive(Clone, Copy, Debug, Default, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct GameID(Uuid);

/// struct representing a game in a given phase.
///
/// the apis that are available depend on the phase and there are clear ways to transition between phases.
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct Game<P: GamePhase> {
    id: GameID,
    name: String,
    #[serde(flatten)]
    phase: P,
}

impl<P: GamePhase> Game<P> {
    pub fn id(&self) -> &GameID {
        &self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}

impl Game<Setup> {
    /// assigns a new id
    pub fn new(name: String) -> Self {
        Self {
            id: GameID(Uuid::new_v4()),
            name,
            phase: Setup::new(),
        }
    }

    /// build from given id
    pub fn new_with_id(id: GameID, name: String) -> Self {
        Self {
            id,
            name,
            phase: Setup::new(),
        }
    }

    pub fn players(&self) -> impl Iterator<Item = &Player> {
        self.phase.players()
    }

    pub fn add_player(&mut self, player_data: NewPlayerData) -> &Player {
        self.phase.add_player(player_data)
    }

    pub fn remove_player(&mut self, player: &PlayerID) -> Option<Player> {
        self.phase.remove_player(player)
    }

    pub fn start(self) -> Game<Running> {
        Game {
            id: self.id,
            name: self.name,
            phase: Running::new(self.phase.into_players()),
        }
    }
}

impl Game<Running> {
    pub fn players(&self) -> impl Iterator<Item = &Player> {
        self.phase.players()
    }

    pub fn remove_player(&mut self, player: &PlayerID) -> Option<Player> {
        self.phase.remove_player(player)
    }

    pub fn attempt_to_kill(
        &mut self,
        killer: &PlayerID,
        target: &PlayerID,
    ) -> Result<PlayerID, KillingError> {
        self.phase.attempt_to_kill(killer, target)
    }

    pub fn get_kills(&self, player: &PlayerID) -> Option<usize> {
        self.phase.get_kills(player)
    }

    pub fn get_player_data(&self, player_id: &PlayerID) -> Result<PlayerData, NotFoundError> {
        let player = self
            .phase
            .get_player(player_id)
            .ok_or(NotFoundError::PlayerNotFound(*player_id))?;
        // safe to unwrao since get_player already should have checked whether the player exists
        let kills = self.get_kills(player_id).expect("player disappeared");
        let lives = self.phase.get_lives(player_id).expect("player disappeared");
        Ok(PlayerData::new(player, lives, kills))
    }

    pub fn get_status_map(&self, player: &PlayerID) -> Option<HashMap<usize, &Status>> {
        self.phase.get_status_map(player)
    }

    /* might be useful
    /// get the status of the player in the circle `index` (zero-indexed).
    pub fn get_status(&self, player: &PlayerID, index: usize) -> Result<&Status, NotFoundError> {
        self.phase.get_status(player, index)
    }
    */

    pub fn finish(self) -> Game<Finished> {
        Game {
            id: self.id,
            name: self.name,
            phase: Finished {},
        }
    }
}
