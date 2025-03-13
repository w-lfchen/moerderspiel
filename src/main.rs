use gamelogic::player::NewPlayerData;

mod gamelogic;

fn main() {
    let mut game = gamelogic::game::Game::new();
    game.add_player(NewPlayerData::new("player0".into()));
    game.add_player(NewPlayerData::new("player1".into()));
    game.add_player(NewPlayerData::new("player2".into()));
    println!("{game:?}");
    let game = game.start();
    println!("{game:?}");
    let game = game.finish();
    println!("{game:?}");
}
