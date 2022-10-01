use crate::{state, state_space, strategies};
use std::{io, marker::PhantomData, str::FromStr};

/// Player input could not be parsed
struct PromptError(&'static str);

/// Prompt user for each call to `get_action()`
#[derive(Clone, Default)]
pub struct CommandPrompt<const N: usize, T: state_space::StateSpace<N>> {
    phantom: PhantomData<T>,
}

impl<const N: usize, T: state_space::StateSpace<N> + 'static> strategies::Strategy<N, T>
    for CommandPrompt<N, T>
{
    fn get_action(&mut self, gamestate: &state::State<N, T>) -> state::action::Action<N, T> {
        loop {
            match self.move_prompt(gamestate) {
                Ok(attack) => return attack,
                Err(error) => {
                    let problem = error.0;
                    println!("{problem} wasn't right.");
                    continue;
                }
            }
        }
    }
}

impl<const N: usize, T: state_space::StateSpace<N>> CommandPrompt<N, T> {
    /// Prompts *player* for the move on their id
    fn move_prompt(
        &self,
        gamestate: &state::State<N, T>,
    ) -> Result<state::action::Action<N, T>, PromptError> {
        let i = gamestate.get_status().get_i();
        println!("Player {i}, would you like to attack or split?");
        let mut move_buffer = String::new();
        io::stdin()
            .read_line(&mut move_buffer)
            .map_err(|_| PromptError("action"))?;
        match move_buffer.as_str().trim() {
            "attack" => self.attack_prompt(gamestate),
            "split" => self.split_prompt(gamestate),
            _ => Err(PromptError("action")),
        }
    }

    /// Prompts *player* for attacking input
    fn attack_prompt(
        &self,
        gamestate: &state::State<N, T>,
    ) -> Result<state::action::Action<N, T>, PromptError> {
        let i = gamestate.get_status().get_i();
        let j = if gamestate.players.len() > 2 {
            println!("Player {i}, what is the index of the player you are attacking?");
            read_parsable()?
        } else {
            1 - i
        };
        println!("Player {i}, which hand are you using to attack?");
        let attacking_hand_index = read_parsable()?;
        println!("Player {i}, which hand are you attacking?");
        let defending_hand_index = read_parsable()?;
        Ok(state::action::Action::Attack {
            i,
            j,
            a: attacking_hand_index,
            b: defending_hand_index,
        })
    }

    /// Prompts *player* for defending input
    fn split_prompt(
        &self,
        gamestate: &state::State<N, T>,
    ) -> Result<state::action::Action<N, T>, PromptError> {
        let i = gamestate.get_status().get_i();
        println!("Player {i}, how many fingers will you split for your left hand?");
        let left = read_parsable()?;
        println!("Player {i}, how many fingers will you split for your right hand?");
        let right = read_parsable()?;
        Ok(state::action::Action::Split {
            i,
            hands: [left, right],
        })
    }
}

/// Reads a single line containing a parsable type or errors
fn read_parsable<T: FromStr>() -> Result<T, PromptError> {
    let mut buffer = String::new();
    let value = io::stdin()
        .read_line(&mut buffer)
        .map(|_| buffer.trim().parse())
        .map_err(|_| PromptError("reading line"))?
        .map_err(|_| PromptError("parsing input"))?;
    Ok(value)
}
