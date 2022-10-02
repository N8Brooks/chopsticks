use ::chopsticks::game::*;
use ::chopsticks::state::action::Action;
use ::chopsticks::state::status::Status;
use ::chopsticks::state_space::*;
use ::chopsticks::strategies::*;

fn main() {
    // let player_1 = Box::new(command_prompt::CommandPrompt::<2, chopsticks::Chopsticks>::default());
    let player_1 = Box::new(random::Random::default());
    let player_2 = Box::new(pure_monte_carlo::PureMonteCarlo::new(100));
    let players: [Box<dyn Strategy<2, chopsticks::Chopsticks>>; 2] = [player_1, player_2];
    let mut game =
        multi_strategy::MultiStrategy::new(chopsticks::Chopsticks.get_initial_state(), players);
    while let Status::Turn { .. } = game.state.get_status() {
        if game.state.is_loop_state() {
            break;
        }
        println!("{}", game.state.get_abbreviation());
        let action = game.get_action().unwrap();
        match action {
            Action::Attack { i, j, a, b } => {
                println!("Player {i} uses hand {a} to attack hand {b} of player index {j}")
            }
            Action::Split { i, hands_0: [a, b], hands_1: [c, d] } => {
                println!("Player {i} split {a}{b} into {c}{d}",)
            }
            _ => panic!("expect not phantom"),
        }
        if game.play_action(&action).is_err() {
            // Human player tried something invalid or there is a bug in a controller
            println!("Action was not valid. Try again.");
            continue;
        }
    }
    match game.state.get_status() {
        Status::Over { i } => println!("Player {i}, you won!"),
        Status::Turn { .. } => println!("The game cannot end from here. Tie!"),
    };
}
