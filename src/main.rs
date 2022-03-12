use chutes::Game;

fn main() {
    let mut game = Game::new([String::from("Yonah"), String::from("Eric")].to_vec());
    while let Ok(()) = game.turn() {};
}
