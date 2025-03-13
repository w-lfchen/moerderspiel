use super::{
    phase::{Finished, GamePhase, Running, Setup},
    player::{NewPlayerData, Player},
};

#[derive(Debug)]
pub struct Game<P: GamePhase> {
    phase: P,
}

impl Game<Setup> {
    pub const fn new() -> Self {
        Self {
            // players: vec![],
            phase: Setup::new(),
        }
    }

    pub fn add_player(&mut self, player_data: NewPlayerData) {
        self.phase.add_player(player_data);
    }

    pub fn start<'a>(self) -> Game<Running> {
        Game::<Running>::new(self.phase.players)
    }
}

impl Game<Running> {
    fn new(players: Vec<Player>) -> Self {
        Self {
            phase: Running::new(players),
        }
    }

    pub fn finish(self) -> Game<Finished> {
        Game {
            // players: self.players,
            phase: Finished {},
        }
    }
}
