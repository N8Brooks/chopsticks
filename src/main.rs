use ::chopsticks::action::Action;
use ::chopsticks::state::status::Status;
use ::chopsticks::state_space::*;
use ::chopsticks::controller::*;
use ::chopsticks::game::*;

fn main() {
    // let player_1 = Box::new(command_prompt::CommandPrompt::<2, chopsticks::Chopsticks>::default());
    let player_1 = Box::new(random::Random::default());
    let player_2 = Box::new(pure_monte_carlo::PureMonteCarlo::new(1000));
    let players: [Box<dyn Controller<2, chopsticks::Chopsticks>>; 2] = [player_1, player_2];
    let mut game = multi_player::MultiPlayer::new(
        chopsticks::Chopsticks.get_initial_state(),
        players,
    );
    while let Status::Turn { id } = game.state.status() {
        let abbreviation = game.state.abbreviation();
        if abbreviation == "0102" {
            panic!("never ending state detected");
        }
        println!("{}", game.state.abbreviation());
        let action = game.get_action().unwrap();
        match action {
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
        if game.play_action(&action).is_err() {
            // Human player tried something invalid or there is a bug in a controller
            println!("Action was not valid. Try again.");
            continue;
        }
    }
    match game.state.status() {
        Status::Over { id } => println!("Player {id}, you won!"),
        Status::Turn { id: _ } => panic!("expect over"),
    };
}
