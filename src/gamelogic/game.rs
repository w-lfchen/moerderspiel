use super::{
    circle::Status,
    errors::KillingError,
    phase::{Finished, GamePhase, Running, Setup},
    player::{NewPlayerData, Player, PlayerID},
};

#[derive(Debug)]
pub struct Game<P: GamePhase> {
    phase: P,
}

impl Game<Setup> {
    pub const fn new() -> Self {
        Self {
            phase: Setup::new(),
        }
    }

    pub fn add_player(&mut self, player_data: NewPlayerData) -> PlayerID {
        self.phase.add_player(player_data)
    }

    pub fn remove_player(&mut self, player: &PlayerID) -> Option<Player> {
        self.phase.remove_player(player)
    }

    pub fn start(self) -> Game<Running> {
        Game::<Running>::new(self.phase.into_players())
    }
}

impl Game<Running> {
    fn new(players: Vec<Player>) -> Self {
        Self {
            phase: Running::new(players),
        }
    }

    pub fn remove_player(&mut self, player: &PlayerID) -> Option<Player> {
        self.phase.remove_player(player)
    }

    pub fn finish(self) -> Game<Finished> {
        Game { phase: Finished {} }
    }

    pub fn get_first_target(&self, player: PlayerID) -> Option<&Status> {
        self.phase.get_first_target(player)
    }

    pub fn attempt_to_kill(
        &mut self,
        killer: PlayerID,
        target: PlayerID,
    ) -> Result<PlayerID, KillingError> {
        self.phase.attempt_to_kill(killer, target)
    }

    pub fn get_kills(&self, player: PlayerID) -> Option<usize> {
        self.phase.get_kills(player)
    }
}
