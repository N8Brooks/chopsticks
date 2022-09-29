use ::chopsticks::action::Action;
use ::chopsticks::controller::*;
use ::chopsticks::state::status::Status;
use ::chopsticks::state_space::*;

fn main() {
    let player_1 = Box::new(command_prompt::CommandPrompt::<2, chopsticks::Chopsticks>::default());
    let player_2 = Box::new(random::Random::default());
    let players: [Box<dyn Controller<2, chopsticks::Chopsticks>>; 2] = [player_1, player_2];
    let mut game = chopsticks::Chopsticks.play(players);
    while let Status::Turn { id } = game.state.status() {
        println!("{}", game.state.abbreviation());
        match game.play_next_action() {
            Action::Attack { i, a, b } => {
                println!("Player id {id} uses hand {a} to attack hand {b} of player index {i}")
            }
            Action::Split { hands } => {
                println!(
                    "Player id {id} splits into left {} and right {}",
                    hands[0], hands[1]
                )
            }
            _ => panic!("expect not phantom"),
        }
    }
    match game.state.status() {
        Status::Over { id } => println!("Player {id}, you won!"),
        Status::Turn { id: _ } => panic!("expect over"),
    };
}
