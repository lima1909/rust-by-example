mod brackets_game;
use brackets_game::greeter;

fn main() {
    let g = greeter::Greeter {};
    g.hello("you".to_string());
}
