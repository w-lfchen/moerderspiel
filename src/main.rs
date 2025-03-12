mod gamelogic;

fn main() {
    let x = gamelogic::game::Game::new();
    println!("{x:?}");
}
