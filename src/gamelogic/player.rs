use std::fmt::Display;

use uuid::Uuid;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, PartialOrd, Ord)]
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

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
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

#[derive(Debug)]
pub struct NewPlayerData {
    name: String,
}

impl NewPlayerData {
    pub const fn new(name: String) -> Self {
        Self { name }
    }
}
