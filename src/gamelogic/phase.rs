/// trait for keeping track of a game's state
pub trait GamePhase {}

#[derive(Debug)]
pub enum Setup {}
impl GamePhase for Setup {}

#[derive(Debug)]
pub enum Running {}
impl GamePhase for Running {}

#[derive(Debug)]
pub enum Finished {}
impl GamePhase for Finished {}
