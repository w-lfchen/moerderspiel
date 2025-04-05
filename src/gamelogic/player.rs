use std::fmt::Display;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq, PartialOrd, Ord, Serialize)]
#[repr(transparent)]
pub struct PlayerID(Uuid);

impl Display for PlayerID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Player({})", self.0)
    }
}

impl PlayerID {
    pub fn new(uuid: Uuid) -> Self {
        Self(uuid)
    }

    pub fn uuid(&self) -> Uuid {
        self.0
    }
}

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct Player {
    /// needs to be unique
    id: PlayerID,
    // /// amount of lives. if 0, the player is considered dead as they have been eliminated from all circles
    // /// this value is initialized to the number of circles.
    // lives: u32,
    /// a name the player may choose themselves
    name: String,
}

impl Player {
    /// id needs to be copy
    pub fn id(&self) -> PlayerID {
        self.id
    }

    #[cfg(test)]
    pub fn name(&self) -> &str {
        &self.name
    }

    pub(super) fn new_from_data(id: PlayerID, data: NewPlayerData) -> Self {
        Self {
            id,
            // lives: NUM_CIRCLES as u32,
            name: data.name,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct NewPlayerData {
    name: String,
}

impl NewPlayerData {
    #[cfg(test)]
    pub const fn new(name: String) -> Self {
        Self { name }
    }
}

/// outgoing data about a player
#[derive(Debug, Serialize)]
pub struct PlayerData<'a> {
    #[serde(flatten)]
    player: &'a Player,
    lives: usize,
    kills: usize,
}

impl<'a> PlayerData<'a> {
    pub(super) fn new(player: &'a Player, lives: usize, kills: usize) -> Self {
        Self {
            player,
            kills,
            lives,
        }
    }
}

#[cfg(test)]
mod tests {
    //! tests to ensure basic apis of the player structs
    use super::*;

    #[test]
    fn test_player_id() {
        let uuid = Uuid::new_v4();
        let player_id = PlayerID::new(uuid);
        assert_eq!(uuid, player_id.uuid());
    }

    #[test]
    fn test_player() {
        const NAME: &str = "player";
        let uuid = Uuid::new_v4();
        let player_id = PlayerID::new(uuid);
        let data = NewPlayerData::new(NAME.into());
        let player = Player::new_from_data(player_id, data);

        assert_eq!(player_id, player.id());
        assert_eq!(NAME, player.name());
    }
}
