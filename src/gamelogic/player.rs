#[derive(Debug, Eq, Hash, PartialEq, PartialOrd, Ord)]
#[repr(transparent)]
struct ID(pub u32);

#[derive(Debug, Eq, Hash, PartialEq)]
pub struct Player {
    id: ID,
    name: String,
}
