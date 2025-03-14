use gamelogic::player::{ID, NewPlayerData};

mod gamelogic;

fn main() {
    // just call some stuff to make sure basic apis work
    let mut game = gamelogic::game::Game::new();
    game.add_player(NewPlayerData::new("player0".into()));
    game.add_player(NewPlayerData::new("player1".into()));
    game.add_player(NewPlayerData::new("player2".into()));
    println!("{game:?}");
    let mut game = game.start();
    let killer = ID(0);
    let status = game.get_first_target(killer).unwrap();
    println!("{game:#?}");
    println!(
        "{:?}",
        game.attempt_to_kill(killer, status.get_target().unwrap())
    );
    println!("{game:#?}");
    println!("{:?}", game.get_kills(killer));
    let game = game.finish();
    println!("{game:#?}");
}
