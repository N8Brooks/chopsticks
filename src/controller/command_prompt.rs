use crate::state_space::StateSpace;
use crate::{action, controller::Controller, state::State};
use std::{io, marker::PhantomData, str::FromStr};

/// Player input could not be parsed
struct PromptError(&'static str);

#[derive(Clone, Default)]
pub struct CommandPrompt<const N: usize, T: StateSpace<N>> {
    phantom: PhantomData<T>,
}

impl<const N: usize, T: StateSpace<N> + 'static> Controller<N, T> for CommandPrompt<N, T> {
    fn get_action(&mut self, gamestate: &State<N, T>) -> action::Action<N, T> {
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

impl<const N: usize, T: StateSpace<N>> CommandPrompt<N, T> {
    /// Prompts *player* for the move on their id
    fn move_prompt(&self, gamestate: &State<N, T>) -> Result<action::Action<N, T>, PromptError> {
        let id = gamestate.status().get_id();
        println!("Player {id}, would you like to attack or split?");
        let mut move_buffer = String::new();
        io::stdin()
            .read_line(&mut move_buffer)
            .map_err(|_| PromptError("Action"))?;
        match move_buffer.as_str().trim() {
            "attack" => self.attack_prompt(gamestate),
            "split" => self.split_prompt(gamestate),
            _ => Err(PromptError("Action")),
        }
    }

    /// Prompts *player* for attacking input
    fn attack_prompt(&self, gamestate: &State<N, T>) -> Result<action::Action<N, T>, PromptError> {
        let id = gamestate.status().get_id();
        let opponent_index = if gamestate.players.len() > 2 {
            println!("Player {id}, what is the index of the player you are attacking?");
            read_parsable()?
        } else {
            1
        };
        println!("Player {id}, which hand are you using to attack?");
        let attacking_hand_index = read_parsable()?;
        println!("Player {id}, which hand are you attacking?");
        let defending_hand_index = read_parsable()?;
        Ok(action::Action::Attack {
            i: opponent_index,
            a: attacking_hand_index,
            b: defending_hand_index,
        })
    }

    /// Prompts *player* for defending input
    fn split_prompt(&self, gamestate: &State<N, T>) -> Result<action::Action<N, T>, PromptError> {
        let id = gamestate.status().get_id();
        println!("Player {id}, how many fingers will you split for your left hand?");
        let left = read_parsable()?;
        println!("Player {id}, how many fingers will you split for your right hand?");
        let right = read_parsable()?;
        Ok(action::Action::Split {
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
        .map_err(|_| PromptError("Reading line"))?
        .map_err(|_| PromptError("Parsing input"))?;
    Ok(value)
}
