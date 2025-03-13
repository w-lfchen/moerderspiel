use gamelogic::player::{ID, NewPlayerData};

mod gamelogic;

fn main() {
    let mut game = gamelogic::game::Game::new();
    game.add_player(NewPlayerData::new("player0".into()));
    game.add_player(NewPlayerData::new("player1".into()));
    game.add_player(NewPlayerData::new("player2".into()));
    println!("{game:?}");
    let mut game = game.start();
    let killer = &ID(0);
    let target = game.get_first_target(killer).unwrap().unwrap();
    println!("{game:#?}");
    println!("{:?}", game.attempt_to_kill(killer, &target));
    println!("{game:#?}");
    let game = game.finish();
    println!("{game:#?}");
}
