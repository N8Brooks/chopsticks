use chopsticks::chopsticks::Chopsticks;
use chopsticks::chopsticks_state::Action;
use std::io;
use std::str::FromStr;

// TODO: This should probably be refactored
//  - The error handling seems clunky. I'm not familiar enough to fix it.
//  - The user experience could be improved with clear screen and better UI.
//  - Some testing would be nice.

/// Mostly occurs when a *player* does something unexpected
#[derive(Debug)]
struct PromptError;

/// Possible actions on a *players* turn
enum Move {
    Attack,
    Split,
}

fn main() {
    let builder = Chopsticks::<u32>::default();
    let mut state = builder.build();
    loop {
        println!("{}", state.abbreviation());
        let turn = state.get_turn();
        if move_prompt(turn)
            .and_then(|player_move| match player_move {
                Move::Attack => attack_prompt(turn).and_then(|(a, b)| {
                    state
                        .apply_action(Action::Attack { i: 1, a, b })
                        .map_err(|_| PromptError)
                }),
                Move::Split => split_prompt(turn).and_then(|(left, right)| {
                    state
                        .apply_action(Action::Split {
                            new_hands: [left, right],
                        })
                        .map_err(|_| PromptError)
                }),
            })
            .is_err()
        {
            println!("Something wasn't right. Try again.")
        }
        if let Some(id) = state.winner_id() {
            println!("Player {id} won!");
            break;
        }
    }
}

/// Prompts *player* for the move on their turn
fn move_prompt(i: usize) -> Result<Move, PromptError> {
    println!("Player {i}, attack or split?");
    let mut move_buffer = String::new();
    if io::stdin().read_line(&mut move_buffer).is_err() {
        return Err(PromptError);
    }
    match move_buffer.as_str() {
        "attack\n" => Ok(Move::Attack),
        "split\n" => Ok(Move::Split),
        _ => Err(PromptError),
    }
}

/// Reads a single line containing a parsable type or errors
fn read_parsable<T: FromStr>() -> Result<T, PromptError> {
    let mut buffer = String::new();
    let value = io::stdin()
        .read_line(&mut buffer)
        .map(|_| buffer.trim().parse())
        .map_err(|_| PromptError)?
        .map_err(|_| PromptError)?;
    Ok(value)
}

/// Prompts *player* for attacking input
fn attack_prompt(i: usize) -> Result<(usize, usize), PromptError> {
    println!("Player {i}, which hand are you using to attack?");
    let attacker = read_parsable()?;
    println!("Player {i}, which hand are you attacking?");
    let defender = read_parsable()?;
    Ok((attacker, defender))
}

/// Prompts *player* for defending input
fn split_prompt(i: usize) -> Result<(u32, u32), PromptError> {
    println!("Player {i}, how many fingers will you split for your left hand?");
    let left = read_parsable()?;
    println!("Player {i}, how many fingers will you split for your right hand?");
    let right = read_parsable()?;
    Ok((left, right))
}
