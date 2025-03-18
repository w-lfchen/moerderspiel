use gamelogic::player::NewPlayerData;

mod gamelogic;

fn main() {
    // just call some stuff to make sure basic apis work
    let mut game = gamelogic::game::Game::new();
    let killer = game.add_player(NewPlayerData::new("player0".into()));
    let rem_1 = game.add_player(NewPlayerData::new("player1".into()));
    let rem_2 = game.add_player(NewPlayerData::new("player2".into()));
    game.add_player(NewPlayerData::new("player3".into()));
    game.add_player(NewPlayerData::new("player4".into()));
    game.add_player(NewPlayerData::new("player5".into()));
    game.add_player(NewPlayerData::new("player6".into()));
    game.remove_player(&rem_1);
    println!("{game:?}");
    let mut game = game.start();
    println!("{game:#?}");
    let status = game.get_first_target(killer).unwrap();
    println!(
        "{:?}",
        game.attempt_to_kill(killer, status.get_target().unwrap())
    );
    game.remove_player(&rem_2);
    println!("{game:#?}");
    println!("{:?}", game.get_kills(killer));
    let game = game.finish();
    println!("{game:#?}");
}
