#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct ID(pub u32);

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Player {
    id: ID,
    name: String,
}

impl Player {
    // id needs to be copy
    pub fn id(&self) -> ID {
        self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub(super) fn new_from_data(id: ID, data: NewPlayerData) -> Self {
        Self {
            id,
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
