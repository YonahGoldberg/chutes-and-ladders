use chutes::Game;

fn main() {
    let mut game = Game::new([String::from("Yonah"), String::from("Eric")].to_vec()).unwrap();
    while let Ok(()) = game.turn() {};
}
